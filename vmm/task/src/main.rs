/*
Copyright 2022 The Kuasar Authors.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

#![warn(clippy::expect_fun_call, clippy::expect_used)]

use std::{
    collections::HashMap, convert::TryFrom, os::fd::AsRawFd, path::Path, process::exit,
    str::FromStr, sync::Arc,
};

use containerd_shim::{
    asynchronous::{monitor::monitor_notify_by_pid, util::asyncify},
    error::Error,
    io_error, other, other_error,
    protos::{shim::shim_ttrpc_async::create_task, ttrpc::asynchronous::Server},
    util::{mkdir, IntoOption},
    Result,
};
use futures::StreamExt;
use lazy_static::lazy_static;
use log::{debug, error, info, warn, LevelFilter};
use nix::{
    errno::Errno,
    sched::{unshare, CloneFlags},
    sys::wait::{self, WaitPidFlag, WaitStatus},
    unistd::{fork, getpid, pause, pipe, ForkResult, Pid},
};
use signal_hook_tokio::Signals;
use tokio::{fs::File, sync::mpsc::channel};
use vmm_common::{
    api::sandbox_ttrpc::create_sandbox_service, mount::mount, ETC_RESOLV, HOSTNAME_FILENAME,
    IPC_NAMESPACE, KUASAR_STATE_DIR, PID_NAMESPACE, RESOLV_FILENAME, SANDBOX_NS_PATH,
    UTS_NAMESPACE,
};

use crate::{
    config::TaskConfig,
    debug::listen_debug_console,
    mount::{get_cgroup_mounts, PROC_CGROUPS},
    sandbox_service::SandboxService,
    task::create_task_service,
};

mod config;
mod container;
mod debug;
mod device;
mod io;
mod mount;
mod netlink;
mod sandbox;
mod sandbox_service;
mod stream;
mod task;
mod util;
mod vsock;

const NAMESPACE: &str = "k8s.io";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StaticMount {
    fstype: &'static str,
    src: &'static str,
    dest: &'static str,
    options: Vec<&'static str>,
}

const ENVS: [(&str, &str); 2] = [
    ("PATH", "/bin:/sbin/:/usr/bin/:/usr/sbin/"),
    ("XDG_RUNTIME_DIR", "/run"),
];

lazy_static! {
    pub static ref VM_ROOTFS_MOUNTS: Vec<StaticMount> = vec![
        StaticMount {
            fstype: "proc",
            src: "proc",
            dest: "/proc",
            options: vec!["nosuid", "nodev", "noexec"]
        },
        StaticMount {
            fstype: "sysfs",
            src: "sysfs",
            dest: "/sys",
            options: vec!["nosuid", "nodev", "noexec"]
        },
        StaticMount {
            fstype: "devtmpfs",
            src: "dev",
            dest: "/dev",
            options: vec!["nosuid"]
        },
        StaticMount {
            fstype: "tmpfs",
            src: "tmpfs",
            dest: "/dev/shm",
            options: vec!["nosuid", "nodev"]
        },
        StaticMount {
            fstype: "devpts",
            src: "devpts",
            dest: "/dev/pts",
            options: vec!["nosuid", "noexec"]
        },
        StaticMount {
            fstype: "tmpfs",
            src: "tmpfs",
            dest: "/run",
            options: vec!["nosuid", "nodev"]
        },
    ];
    pub static ref SHAREFS_9P_MOUNTS: Vec<StaticMount> = vec![StaticMount {
        fstype: "9p",
        src: "kuasar",
        dest: KUASAR_STATE_DIR,
        options: vec!["trans=virtio,version=9p2000.L", "nodev"]
    },];
    pub static ref SHAREFS_VIRTIOFS_MOUNTS: Vec<StaticMount> = vec![StaticMount {
        fstype: "virtiofs",
        src: "kuasar",
        dest: KUASAR_STATE_DIR,
        options: vec!["relatime", "nodev", "sync", "dirsync",]
    },];
    static ref CLONE_FLAG_TABLE: HashMap<String, CloneFlags> = HashMap::from([
        (String::from(IPC_NAMESPACE), CloneFlags::CLONE_NEWIPC),
        (String::from(UTS_NAMESPACE), CloneFlags::CLONE_NEWUTS),
        (String::from(PID_NAMESPACE), CloneFlags::CLONE_NEWPID),
    ]);
}

async fn initialize() -> anyhow::Result<()> {
    early_init_call().await?;

    let config = TaskConfig::new().await?;
    let log_level = LevelFilter::from_str(&config.log_level)?;
    env_logger::Builder::from_default_env()
        .format_timestamp_micros()
        .filter_module("containerd_shim", log_level)
        .filter_module("vmm_task", log_level)
        .init();
    info!("Task server start with config: {:?}", config);

    match &*config.sharefs_type {
        "9p" => {
            mount_static_mounts(SHAREFS_9P_MOUNTS.clone()).await?;
        }
        "virtiofs" => {
            mount_static_mounts(SHAREFS_VIRTIOFS_MOUNTS.clone()).await?;
        }
        _ => {
            warn!("sharefs_type should be either 9p or virtiofs");
        }
    }
    if config.debug {
        debug!("listen vsock port 1025 for debug console");
        if let Err(e) = listen_debug_console("vsock://-1:1025").await {
            error!("failed to listen debug console port, {:?}", e);
        }
    }

    late_init_call(&config).await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = initialize().await {
        error!("failed to do init call: {:?}", e);
        exit(-1);
    }

    // Keep server alive in main function
    let mut server = match create_ttrpc_server().await {
        Ok(s) => s,
        Err(e) => {
            error!("failed to create ttrpc server: {:?}", e);
            exit(-1);
        }
    };
    if let Err(e) = server.start().await {
        error!("failed to start ttrpc server: {:?}", e);
        exit(-1);
    }

    let signals = match Signals::new([libc::SIGTERM, libc::SIGINT, libc::SIGPIPE, libc::SIGCHLD]) {
        Ok(s) => s,
        Err(e) => {
            error!("new signal failed: {:?}", e);
            exit(-1);
        }
    };

    info!("Task server successfully started, waiting for exit signal...");
    handle_signals(signals).await;
}

// Do some initialization before everything starts.
// Such as setting envs, preparing cgroup mounts, setting kernel paras.
async fn early_init_call() -> Result<()> {
    // Set environment variables from ENVS vector(ordered).
    for (k, v) in ENVS.iter() {
        std::env::set_var(k, v);
    }
    init_vm_rootfs().await?;
    Ok(())
}

async fn handle_signals(signals: Signals) {
    let mut signals = signals.fuse();
    while let Some(sig) = signals.next().await {
        match sig {
            libc::SIGTERM | libc::SIGINT => {
                debug!("received {}", sig);
            }
            libc::SIGCHLD => loop {
                // Note: see comment at the counterpart in synchronous/mod.rs for details.
                match asyncify(move || {
                    Ok(wait::waitpid(
                        Some(Pid::from_raw(-1)),
                        Some(WaitPidFlag::WNOHANG),
                    )?)
                })
                .await
                {
                    Ok(WaitStatus::Exited(pid, status)) => {
                        debug!("child {} exit with code ({})", pid, status);
                        monitor_notify_by_pid(pid.as_raw(), status)
                            .await
                            .unwrap_or_else(|e| error!("failed to send exit event {}", e))
                    }
                    Ok(WaitStatus::Signaled(pid, sig, _)) => {
                        debug!("child {} terminated({})", pid, sig);
                        let exit_code = 128 + sig as i32;
                        monitor_notify_by_pid(pid.as_raw(), exit_code)
                            .await
                            .unwrap_or_else(|e| error!("failed to send signal event {}", e))
                    }
                    // StillAlive is returned when the waitpid syscall return 0,
                    // which means there is no more events so we should break the loop.
                    // actually it is not possible to get the ECHILD error,
                    // because only when the pid parameter of waitpid() is a positive value,
                    // and the process specified by this pid does not exist
                    // or not a child of current process, that the error of ECHILD return.
                    // as the pid param is -1 here, it is not possible to return ECHILD.
                    // I don't want to remove this match case because it is from
                    // the open sourced rust-extensions. and I think we can still break the loop
                    // even if it happened unexpectedly, because the error also indicates
                    // that should be no more events happen.
                    Err(Error::Nix(Errno::ECHILD)) | Ok(WaitStatus::StillAlive) => {
                        break;
                    }
                    Err(e) => {
                        // stick until all children will be successfully waited,
                        // even some unexpected error occurs.
                        //
                        // according to the linux man page of waitpid(2)
                        // https://linux.die.net/man/2/waitpid,
                        // waitpid() return three kinds of errors: ECHILD,EINTR,EINVAL
                        // as we assign the parameter pid to -1, ECHILD is impossible,
                        // as we set option to WNOHANG, EINTR is impossible,
                        // as the option is a enum of WaitPidFlag::WNOHANG,
                        // which can not be a invalid value, so the EINVAL is impossible.
                        warn!("unexpected error occurred in signal handler: {}", e);
                    }
                    r => {
                        // as all the errors is handled, other possible return value is
                        // Stopped(Pid, Signal), PtraceEvent(Pid, Signal, c_int)
                        // PtraceSyscall(Pid), Continued(Pid).
                        // these return values can be possible
                        // only when the option of WaitPidFlag::WUNTRACED is set.
                        // as we didn't set this flag, we can not get these values theoretically.
                        // but even if we get these values, we still need to continue
                        // calling waitpid because there is a real pid in these return values,
                        // which indicates that there maybe more child process events.
                        debug!("unexpected waitpid return {:?}", r);
                    } // stick until exit
                }
            },
            _ => {
                if let Ok(sig) = nix::sys::signal::Signal::try_from(sig) {
                    debug!("received {}", sig);
                } else {
                    warn!("received invalid signal {}", sig);
                }
            }
        }
    }
}

async fn init_vm_rootfs() -> Result<()> {
    let mounts = VM_ROOTFS_MOUNTS.clone();
    mount_static_mounts(mounts).await?;
    // has to mount /proc before find cgroup mounts
    let cgroup_mounts = get_cgroup_mounts(PROC_CGROUPS, false).await?;
    mount_static_mounts(cgroup_mounts).await?;
    // Enable memory hierarchical account.
    // For more information see https://www.kernel.org/doc/Documentation/cgroup-v1/memory.txt
    tokio::fs::write("/sys/fs/cgroup/memory/memory.use_hierarchy", "1")
        .await
        .map_err(io_error!(e, "failed to set cgroup hierarchy to 1"))
}

// Continue to do initialization that depend on shared path.
// such as adding guest hook, preparing sandbox files and namespaces.
async fn late_init_call(config: &TaskConfig) -> Result<()> {
    // Setup DNS, bind mount to /etc/resolv.conf
    let dns_file = Path::new(KUASAR_STATE_DIR).join(RESOLV_FILENAME);
    if dns_file.exists() {
        nix::mount::mount(
            Some(&dns_file),
            ETC_RESOLV,
            Some("bind"),
            nix::mount::MsFlags::MS_BIND,
            None::<&str>,
        )?;
    } else {
        warn!("unable to find DNS files in kuasar state dir");
    }

    // Setup sandbox namespace
    setup_sandbox_ns(config.share_pidns).await?;

    Ok(())
}

async fn mount_static_mounts(mounts: Vec<StaticMount>) -> Result<()> {
    for m in mounts {
        tokio::fs::create_dir_all(Path::new(m.dest))
            .await
            .map_err(io_error!(e, "failed to create {}: ", m.dest))?;
        match mount(
            m.fstype.none_if(|x| x.is_empty()),
            m.src.none_if(|x| x.is_empty()),
            &m.options
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
            m.dest,
        ) {
            Ok(_) => {}
            Err(e) => {
                if !e.to_string().contains("Device or resource busy") {
                    // we assume that the "Device or resource busy" means it is already mounted, maybe kernel did the mounts.
                    return Err(other!("failed to mount {:?}, {}", m, e));
                }
            }
        };
    }
    Ok(())
}

// create_ttrpc_server will create all the ttrpc service and register them to a server that
// bind to vsock 1024 port.
async fn create_ttrpc_server() -> anyhow::Result<Server> {
    let (tx, rx) = channel(128);
    let task = create_task_service(tx).await?;
    let task_service = create_task(Arc::new(Box::new(task)));

    let sandbox = SandboxService::new(rx)?;
    sandbox.handle_localhost().await?;
    let sandbox_service = create_sandbox_service(Arc::new(Box::new(sandbox)));

    Ok(Server::new()
        .bind("vsock://-1:1024")?
        .register_service(task_service)
        .register_service(sandbox_service))
}

async fn setup_sandbox_ns(share_pidns: bool) -> Result<()> {
    let mut nss = vec![String::from(IPC_NAMESPACE), String::from(UTS_NAMESPACE)];
    if share_pidns {
        nss.push(String::from(PID_NAMESPACE));
    }
    setup_persistent_ns(nss).await?;
    Ok(())
}

async fn setup_persistent_ns(ns_types: Vec<String>) -> Result<()> {
    if ns_types.is_empty() {
        return Ok(());
    }
    mkdir(SANDBOX_NS_PATH, 0o711).await?;

    let mut clone_type = CloneFlags::empty();

    for ns_type in &ns_types {
        let sandbox_ns_path = format!("{}/{}", SANDBOX_NS_PATH, ns_type);
        File::create(&sandbox_ns_path).await.map_err(io_error!(
            e,
            "failed to create: {}",
            sandbox_ns_path
        ))?;

        clone_type |= *CLONE_FLAG_TABLE
            .get(ns_type)
            .ok_or(other!("bad ns type {}", ns_type))?;
    }

    fork_sandbox(ns_types, clone_type)?;

    Ok(())
}

fn fork_sandbox(ns_types: Vec<String>, clone_type: CloneFlags) -> Result<()> {
    debug!("fork sandbox process {:?}, {:b}", ns_types, clone_type);
    let (r, w) = pipe().map_err(other_error!(e, "create pipe when fork sandbox error"))?;
    match unsafe { fork().map_err(other_error!(e, "failed to fork"))? } {
        ForkResult::Parent { child } => {
            debug!("forked process {} for the sandbox", child);
            drop(w);
            let mut resp = [0u8; 4];
            // just wait the pipe close, do not care the read result
            nix::unistd::read(r.as_raw_fd(), &mut resp).unwrap_or_default();
            Ok(())
        }
        ForkResult::Child => {
            drop(r);
            unshare(clone_type).unwrap();
            if !ns_types.iter().any(|n| n == PID_NAMESPACE) {
                debug!("mount namespaces in child");
                mount_ns(getpid(), &ns_types);
                exit(0);
            }
            // if we need share pid ns, we fork a pause process to act as the pid 1 of the shared pid ns
            match unsafe { fork().unwrap() } {
                ForkResult::Parent { child } => {
                    mount_ns(child, &ns_types);
                    exit(0);
                }
                ForkResult::Child => {
                    debug!("mount namespaces in grand child");
                    drop(w);
                    loop {
                        pause();
                    }
                }
            }
        }
    }
}

fn mount_ns(pid: Pid, ns_types: &Vec<String>) {
    if ns_types.iter().any(|n| n == UTS_NAMESPACE) {
        let hostname = std::fs::read_to_string(Path::new(KUASAR_STATE_DIR).join(HOSTNAME_FILENAME))
            .map(|s| s.trim().to_string())
            .unwrap_or_default();
        if !hostname.is_empty() {
            debug!("set hostname for sandbox: {}", hostname);
            nix::unistd::sethostname(hostname).unwrap();
        }
    }
    for ns_type in ns_types {
        let sandbox_ns_path = format!("{}/{}", SANDBOX_NS_PATH, ns_type);
        let ns_path = format!("/proc/{}/ns/{}", pid, ns_type);
        debug!("mount {} to {}", ns_path, sandbox_ns_path);
        mount(
            Some("none"),
            Some(ns_path.as_str()),
            &["bind".to_string()],
            &sandbox_ns_path,
        )
        .unwrap();
    }
}

---
authors:
- Kuasar Admin
categories:
- General
- Announcements
date: 2023-04-15
draft: false
lastmod: 2023-04-15
summary: Benchmark Test Report on Kuasar
tags:
- Kuasar
- Sandbox
- containerd
- benchmark
- performance
- memory overhead
- startup time
title: Benchmark Test Report on Kuasar
---

# Benchmark Test Report on Kuasar

## Abstract

Kuasar provides an optimized framework to accelerate container startup and reduce unnecessary overheads. We benchmarked Kuasar, and the results demonstrated the superiority of its framework.

## Test Enviroment

|                  | version  |
| ---------------- | -------- |
| Contained        | v1.7.0   |
| Kata             | v2.5.2   |
| Operation System | centOS 8 |
| Cloud-Hypervisor | v28.2    |
| Guest-kernel     | 6.1.6    |

## Variables control

In order to control variables consistently, we use:

- same versions of cloud-hypervisor and guest-kernel

- same container image: docker.io/library/ubuntu:latest

- all pods are created using the hostNetwork mode, which means that CNI plugins are not used

- the container root filesystem storage driver uses overlayfs

## Performance Metrics

- memory overhead:

  Start 1/5/10/20/30/50 pods，measure the Pss of Kuasar's sandboxer process and the total Pss of all `containerd-shim-kata-v2` processes. The Pss is obtaind from the smaps_rollup file under the `/proc` dictionary.

  Measure three times and take the average value.

- startup time:

  The container startup time (including pod startup time) measured end-to-end through CRI. The testing is divided into two groups, one launching a single pod and the other launching 30 pods in parrllel. Every group runs 500 times and obtains CDF data.

## Benchmark Test Result

### boot-serial-1000

![](./images/boot-serial-1000.JPG)

Figure1: Cumulative distribution of wall-clock times for starting container in serial, for Kata and Kuasar.

The boot time of Kata is about 850~950 ms while Kuasar is only 300~360 ms, which less than half of Kata.

### boot-parallel-50

![](./images/boot-parallel-50.JPG)

Figure2: Cumulative distribution of wall-clock times for starting containers in groups of 50 in parallel, for Kata and Kuasar.

The boot time of Kata is about 1600~1800 ms while Kuasar is only 930~1050 ms, which nearly half of Kata.

### mem

![](./images/mem.JPG)

Figure3: Memory overhead (Pss) for Kuasar's sandboxer process and all the processes of Kata-shim.

With the number of pods increases, the memory overhead of Kuasar's sandboxer grows very slowly.When the number of pods comes to 50, Pss is only about 15 MB.

On the other hand, the average Pss of each pods for Kata is about 18 MB, which significantly higher than Kuasar.

## Result analysis

The test results are as shown above, we can see that Kuasar performs much better than KATA in terms of boot times and memory overhead.

Simply put, the better start-up times are due to the resident process mentioned earlier and the removal of the pause container.

And the memory overhead is lower because Kuasar uses the 1:N process management model and is written on Rust.

You can obtain the complete test report and test process from [here](https://github.com/kuasar-io/kuasar/blob/main/tests/benchmark/Benchmark.md).
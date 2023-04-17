---
date: 2023-04-17
draft: false
lastmod: 2023-04-17
linktitle: Overview
menu:
    docs:
        parent: architecture
        weight: 1
title: Overview
toc: true
type: docs
---

## Architecture Overview

Kuasar is a multi-sandbox container runtime. Then what is container runtime? In short, container runtime is the component responsible for running containers.  Generally, container runtime is classified into high-level container runtime and low-level container runtime. 

![container age docker](/img/docs/architecture/kuasar-architecture.png)

### High-level container runtime

High-level container runtime is responsible for CRI implementation and image management, etc. containerd, CRI-O,  docker and iSulad are typical high-level container runtime. 

### Low-level container runtime

And Low-level container runtime is responsible for container & sandbox lifecycle management, etc. So Kuasar is a Low-Level container runtime from a technical landscape.

### Sandboxer plugin and Sandbox API

Kuasar connects with high-level container runtime with sandboxer plugin and sandbox API. The sandboxer plugin introduces the sandbox concept in High-Level runtime, making the sandbox become the "first-class citizen". And the Sandbox API provides efficient and scalable API to manage various sandboxes.

A discussion about the sandboxer plugin has been raised in this [Containerd issue](https://github.com/containerd/containerd/issues/7739), with a community meeting record and slides attached in this [comment](https://github.com/containerd/containerd/issues/7739#issuecomment-1384797825). Now this feature has been put into 2.0 milestone.

## Kuasar Components

As for Kuasar, it consists of two main modules, the Kuasar-sandboxer and Kuasar-Task. 

### Kuasar-sandboxer

The Kuasar-sandboxer, which implements the Sandbox API, is responsible for sandbox lifecycle and resource allocation management. 

### Kuasar-Task

The Kuasar-Task, which implements the Task API,  responds to the requests from high-level container runtime then manages the life cycle and resource allocation of user’s containers.

## Multi-Sandbox Support

Kuasar is a multi-sandbox container runtime, that means it supports mainstream sandboxes. 

As you can see, Kuasar now supports Cloud-Hypervisor, StratoVirt, WasmEdge and Quark sandboxes. And we're planning to support more sandboxes, for details, you can see the [roadmap](../../roadmap). That means Kuasar will have the ability to meet the requirements of different cloud native scenarios.



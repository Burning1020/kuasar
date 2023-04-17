---
date: 2023-04-17
draft: false
lastmod: 2023-04-17
linktitle: Sandbox Rise
menu:
    docs:
        parent: background
        weight: 2
title: Sandbox Rise
toc: true
type: docs
---

## Various cloud-native scenarios, diverse sandboxes emerge

Around 2018, various cloud-native container isolation technologies have gradually emerged. We call this technology "sandbox". 

And the sandbox is a natural POD model. It provides an isolated environment for a group of containers. 

There are some typical sandboxes: microVM sandbox, application-kernel sandbox and WASM sandbox. 

They are born for enterprises' requirements under different cloud-native scenarios, so they have different advantage dimensions. 

![container age docker](/img/docs/background/sandbox-class.png)

As you can see, microVM sandbox is suitable for secure and general multi-tenant situations. Application-kernel sandbox is suitable for efficient and general multi-tenant situations. WASM sandbox is suitable for very light task situations, like function computing.

![container age docker](/img/docs/background/sandbox-vendor-layout.png)

## Pains & Trends

Now, the mainstream cloud vendors are using multiple sandbox technologies in their products. But there is no single container runtime that can directly run the sandboxes mentioned above at the same time.

Therefore, users have to maintain multiple container runtimes to manage different sandboxes, which increases the pressure on O&M (operation and maintenance). 

In addition, because the current container runtime lacks a smooth evolution path, most enterprises are still stuck in place and unable to integrate the industry's latest sandbox technology into their products

In last year, the mainstream container runtime 'containerd', at the voice of the community, introduced the Sandbox API in its preview version. 

![container age docker](/img/docs/background/sandbox-api-containerd.png)

We're pretty sure that the sandbox is going to be, and it should be a new first-class citizen in container runtime. 

We need a multi-sandbox container runtime. 

It makes the sandbox first-class citizen. 

It supports mainstream sandbox techniques naturally. 

It has a plug-in, extensible and evolvable mechanism.

So that is why the Kuasar was born.
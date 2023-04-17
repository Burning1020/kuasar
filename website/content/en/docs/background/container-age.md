---
date: 2023-04-17
draft: false
lastmod: 2023-04-17
linktitle: Container Age
menu:
    docs:
        parent: background
        weight: 1
title: Container Age
toc: true
type: docs
---

## Container is the only first-class citizen in Docker

![container age docker](/img/docs/background/container-age-docker.png)

In 2013, the docker technology emerged, and container technology entered an era of rapid development. Containers utilize the Linux system isolation model, which includes namespace and cgroup. Container is the only first-class citizen in Docker. 

Later, the kubernetes project became mainstream at the container orchestration level. So the "pause" container was introduced to accommodate the first-class citizen "Pod" in Kubernetes.

![container age docker](/img/docs/background/container-age-kubernetes.png)

## Cons

But the "pause" container is actually very confusing. Because a pod is not just a "pause" container that only provides a shared network namespace. Rather, it is a logical and physical collection of multiple containers. 

Moreover, there are many unnecessary and complex logic judgments in the container runtime to differentiate the user's container from the "pause" container, which also increases the difficulty of development virtually.






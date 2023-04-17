---
draft: false
linktitle: Why Kuasar
menu:
  docs:
    parent: architecture
    weight: 5
title: Why Kuasar
toc: true
type: docs
---

In the container world, a sandbox is a technique used to separate container processes from each other, and from the operating system itself. After the introduction of the [Sandbox API](https://github.com/containerd/containerd/issues/4131), sandbox has become the first-class citizen in containerd. With more and more sandbox techniques available in the container world, a management service called "sandboxer" is expected to be proposed.

Kuasar supports various types of sandboxers, making it possible for users to select the most appropriate sandboxer for each application, according to application requirements.

## Overall Advantages

Compared with other container runtimes, Kuasar has the following advantages:

+ **Unified Sandbox Abstraction**

     The sandbox is a first-class citizen in Kuasar as the Kuasar is entirely built upon the Sandbox API, which was previewed by the containerd community in October 2022. Kuasar fully utilizes the advantages of the Sandbox API, providing a unified way for sandbox access and management, and improving sandbox O&M efficiency.

+ **Multi-Sandbox Colocation**
     
     Kuasar has built-in support for mainstream sandboxes, allowing multiple types of sandboxes to run on a single node. Kuasar is able to balance user's demands for security isolation, fast startup, and standardization, and enables a serverless node resource pool to meet various cloud-native scenario requirements.

+ **Optimized Framework**
     
     Optimization has been done in Kuasar via replacing all pause containers and shim processes by a single resident sandboxer process, bringing about a 1:N process management model, which has a better performance than the current 1:1 shim v2 process model. The benchmark test results showed that Kuasar's sandbox startup speed 2x, while the resource overhead for management was reduced by 99%. More details please refer to [Performance](https://github.com/kuasar-io/kuasar#Performance).

+ **Open and Neutral**
     
     Kuasar is committed to building an open and compatible multi-sandbox technology ecosystem. Thanks to the Sandbox API, it is more convenient and time-saving to integrate sandbox technologies. Kuasar keeps an open and neutral attitude towards sandbox technologies, therefore all sandbox technologies are welcome. Currently, the Kuasar project is collaborating with open-source communities and projects such as WasmEdge, openEuler and QuarkContainers.


## Technical Advantages

![vmm-arch](/img/docs/architecture/kuasar-arch-compare.png)

The technical advantages of Kuasar include mainly:

* **1. Flexible plug-in sandbox expansion capability**

     Kuasar has the flexible plug-in sandbox expansion capability. It provides a more developer-friendly sandbox access framework, which makes the continuous integration of mainstream sandbox possible.

* **2. Clear and unified sandbox definition**

     Kuasar has a clear and unified sandbox definition based on the latest Sandbox API. The management logic of the sandbox and container are separated and no longer coupled together as before. And this is going to be native support by upstream containerd community.

* **3. Simplified container API call chain**

     Kuasar has a simplified container API call chain. As you can see, container runtime can manage container without extra conversion in "shim" like before.

* **4. Efficient Kuasar sandboxer**

     Kuasar has efficient sandboxer and optimized framework:
  * Its process is resident, which means a shorter startup time.
  * Its process module is 1:N, which means its resource overhead won't increase as the number of containers increases. 
  * It is written in Rust, which means high performance, high security and low overhead.

* **5. Disappeared  "pause" container**
      
    Kuasar removes the "pause" container for the reason that the secure container is a natural Pod model. This optimization reduces both the resource overhead and boot time introduced by the pause container.

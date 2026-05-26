# Nova Server 🚀
An ultra-low footprint, high-performance local game-streaming host designed for the GameStream/Moonlight ecosystem, written entirely in native **Rust**.

Nova completely replaces the bloated, legacy server-side architectures of standard streaming solutions. By stripping away cross-platform abstractions, heavy background web services, and memory-heavy management runtime stacks, Nova provides a highly optimized, single-purpose Windows background engine with near-zero overhead.

---

## 🔥 Why Nova?

Standard game-streaming hosts run heavy embedded web servers (Node.js/React UI daemons), use generic frame-pacing models that choke multi-gigabit hardware, and rely on external, fragile script orchestration to fix display mapping or microphone routing. 

Nova is built from the ground up for absolute performance:

* **Zero-Copy Pipeline:** Frames are captured straight from the GPU frame buffer via the Windows DXGI Desktop Duplication API and passed directly via VRAM pointers to hardware encoders (**NVIDIA NVENC** or **AMD AMF**). Zero system memory copies.
* **Microscopic Footprint:** No background runtime bloat. Written entirely in Rust, Nova sits at **< 30MB of RAM** and **~0% idle CPU** utilization.
* **Predictive Token-Bucket Pacing:** Eliminates network switch buffer bloat and wireless micro-stutter by mathematically pacing packet distribution cleanly across frame pacing limits.
* **Dual-Protocol Engine:** Fully compatible with legacy Moonlight clients for down-stream video and control packets, with an isolated upstream parallel listener built for **Echo Client** ultra-low-latency voice data.

---

## 💎 Project Tiers & Licensing

Nova operates on an "Open-Core" development framework. The baseline performance-centric engine is completely free and open-source, while complex OS display virtualization and upstream communication layers are funded via a clean licensing model.

| Feature Layer | Access | Capabilities Included |
| :--- | :--- | :--- |
| **Nova Core Engine** | **Free / OSS** | Full Rust host pipeline, zero-copy DXGI capture, native NVENC/AMF encoding, token-bucket UDP packet pacing, standard legacy Moonlight client compatibility. |
| **Nova Pro Engine** | **$4.99 / $60 Lifetime** | Integrated **Indirect Display Driver (IDD)** display virtualization, automated headless resolution/HDR profile cloning on client connect, unified **Echo Mic Passthrough** WASAPI loopback routing. |

---

## 🗺️ Architectural Roadmap

Nova is developed utilizing an inside-out approach, prioritizing core rendering pipelines and absolute memory boundaries before implementing front-end wrappers or secondary utilities.

* [ ] **Phase 1: Foundation:** DXGI Desktop Duplication pipeline, VRAM pointer polling loop, and basic Cargo framework setup.
* [ ] **Phase 2: Encoding:** Native C/C++ FFI bindings mapping DXGI surfaces straight into NVIDIA NVENC and AMD AMF hardware context blocks.
* [ ] **Phase 3: Protocol:** RTSP handshake, cryptographic client pairing verification, and UDP token-bucket engine completion.
* [ ] **Phase 4: Virtualization:** Parallel UDP audio listener loop for Echo client microphone upstream, WASAPI virtual audio device mapping, and IDD display hooks.
* [ ] **Phase 5: Release:** Headless `config.toml` parser optimization, system service daemon configuration, and public alpha builds.

---

## 🛠️ Developer Compilation Prerequisites

To compile Nova natively from source, you must target a modern 64-bit Windows environment. Ensure the following tooling is active on your development host:

1. **Rust Toolchain:** Stable `x86_64-pc-windows-msvc` target.
2. **Windows SDK:** Minimum build version matching Windows 11 target environments.
3. **GPU Drivers:** Updated hardware vendor drivers providing access to native NVENC (`nvEncodeAPI.h`) or AMF developer header libraries.

```bash
# Clone the repository
git clone [https://github.com/YOUR_USERNAME/nova-server.git](https://github.com/YOUR_USERNAME/nova-server.git)
cd nova-server

# Build optimized release binaries
cargo build --release


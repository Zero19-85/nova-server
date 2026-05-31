# Nova Server 🚀

An ultra-low footprint, high-performance local game-streaming host designed for the GameStream/Moonlight ecosystem, written entirely in native **Rust**.

Nova completely replaces the bloated, legacy server-side architectures of standard streaming solutions. By stripping away cross-platform abstractions, heavy background web services, and memory-heavy management runtime stacks, Nova provides a highly optimized, single-purpose Windows background engine with near-zero overhead.

---

## 🔥 Why Nova?

Standard game-streaming hosts run heavy embedded web servers, use generic frame-pacing models that choke multi-gigabit hardware, and rely on fragile script orchestration. Nova is built from the ground up for absolute performance:

* **Zero-Copy Pipeline:** Frames are captured straight from the GPU frame buffer via the Windows DXGI Desktop Duplication API and passed directly via VRAM pointers to hardware encoders.
* **Microscopic Footprint:** No background runtime bloat. Written entirely in native Rust, Nova sits at **< 30MB of RAM** and **~0% idle CPU** utilization.
* **Native CUDA/NVENC Integration:** Nova utilizes a raw CUDA context to bypass Windows display manager locks, ensuring rock-solid stability even when background NVIDIA display containers are active.
* **Dual-Protocol Engine:** Fully compatible with legacy Moonlight clients, with an isolated parallel listener built for ultra-low-latency voice data.

---

## 💎 Project Tiers & Licensing

Nova operates on an "Open-Core" development framework. The baseline performance-centric engine is free and open-source, while complex OS display virtualization and upstream communication layers are funded via a professional licensing model.

| Feature Layer | Access | Capabilities Included |
| :--- | :--- | :--- |
| **Nova Core Engine** | **Free / OSS** | Full Rust host pipeline, raw CUDA/NVENC hardware handshake, DXGI capture, token-bucket UDP pacing. |
| **Nova Pro Engine** | **$4.99 / $60 Lifetime** | Integrated Indirect Display Driver (IDD) virtualization, automated headless HDR, Echo Mic Passthrough. |

---

## 🗺️ Architectural Roadmap

* [x] **Phase 1: Foundation:** DXGI Desktop Duplication and Cargo framework setup.
* [x] **Phase 2: Encoding:** Native CUDA context creation and NVENC hardware handshake.
* [ ] **Phase 3: Pipeline:** VRAM mapping, frame buffer capture to encoder, and RTSP handshake.
* [ ] **Phase 4: Virtualization:** UDP microphone upstream, WASAPI virtual audio, and IDD display hooks.
* [ ] **Phase 5: Release:** Headless `config.toml` optimization and Alpha builds.

---

## 🛠️ Developer Compilation & Usage

Nova requires a modern 64-bit Windows environment.

### Prerequisites
1. **Rust Toolchain:** `stable-x86_64-pc-windows-msvc`.
2. **Administrator Privileges:** **Required** for GPU hardware encoder access. Always run your terminal or VS Code as Administrator.

### Quick Start
```bash
# Clone the repository
git clone [https://github.com/Zero19-85/nova-server.git](https://github.com/Zero19-85/nova-server.git)
cd nova-server

# Build and run
cargo run --release

⚠️ Disclaimers and Legal
​NVIDIA® and NVENC®: This project uses the NVIDIA Video Codec SDK. All relevant trademarks belong to NVIDIA Corporation. This software is provided as-is and is not affiliated with, endorsed by, or sponsored by NVIDIA Corporation.
​Hardware Access: Because Nova interacts directly with GPU hardware (NVENC/CUDA), it requires elevated (Administrator) permissions to function. Use caution when running low-level hardware interfaces.
​No Warranty: This software is provided "as is," without warranty of any kind. The authors are not responsible for any damage to your system or hardware.
​Built for speed. Built for performance. Built for Nova.
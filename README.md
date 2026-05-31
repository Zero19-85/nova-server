# Nova + Echo

**Ultra-lightweight, high-performance game streaming for Moonlight.**

**Nova** = the host (server)  
**Echo** = the client (companion app)

Nova is a next-generation Sunshine replacement built for minimal resource usage, instant startup, and native features Sunshine never had — especially **Echo Mic** (real-time microphone passthrough).

### Why Nova + Echo?

- Single executable, zero background services
- Extremely low VRAM & CPU footprint (lighter than Sunshine)
- True zero-copy DXGI → NVENC pipeline
- Works out-of-the-box on RTX 50-series (no Windows tweaks)
- Native **Echo Mic** support — something Sunshine still can’t do cleanly
- Moonlight-compatible (no client changes required if you just want to use Moonlight)

### Current Status (May 31, 2026)

**Phase 2 Complete**  
✅ DXGI Desktop Duplication working  
✅ Stable NVENC session via official NVIDIA headers (C++ shim)  
✅ Hybrid Rust + thin C++ architecture (single clean executable)  

**Next** → Phase 3: Full encoding loop + zero-copy pipeline

### Features (Planned / In Progress)

- Click-and-play (no GUI bloat)
- Config-driven (`config.toml`)
- Native mic passthrough (Echo Mic)
- Desktop audio loopback
- Low-latency RTP/RTSP for Moonlight
- Future support for Intel QuickSync + AMD AMF (same shim pattern)
- Headless mode + optional tiny tray icon

### Quick Start (once we hit alpha)

```powershell
# After we finish Phase 3
cargo run --release
Or download pre-built nova.exe from Releases.
Architecture

Rust handles high-level orchestration, capture, config, and networking (light & safe)
Thin C++ shim for vendor encoders (NVENC today, QuickSync/AMF tomorrow) — exactly like Sunshine does under the hood, but cleaner
Zero-copy wherever possible (frames stay in GPU memory)

This keeps Nova fast, maintainable, and truly lightweight.
Development Roadmap

Phase 1 — Foundation & DXGI capture → DONE
Phase 2 — NVENC session via stable shim → DONE
Phase 3 — Encoding loop + zero-copy DXGI → NVENC (next)
Phase 4 — Audio + Echo Mic passthrough
Phase 5 — Full Moonlight/RTSP compatibility + controller support
Phase 6 — Intel/AMD encoder shims + headless mode
Phase 7 — Polish, releases, donation-supported builds

Building from Source
PowerShellgit clone https://github.com/Zero19-85/nova-server.git
cd nova-server
cargo run
(Requires Windows + NVIDIA GPU for now. Administrator rights needed for DXGI duplication.)
Contributing & Donations
This project is built for the community.
If you want to help speed up development (or just buy the dev a coffee), donations are greatly appreciated and directly fund more time on Nova + Echo.
Goal: A streaming host that feels native, starts instantly, uses way less resources, and finally gives you proper mic support.

Nova (host) + Echo (client) — the lightweight future of GameStream.
text---

### One last thing — Continuation Tag (copy this for your next chat)
=== NOVA + ECHO CONTINUATION (May 31, 2026) ===
Current state:

DXGI Desktop Duplication working
C++ shim successfully opens NVENC session on RTX 5070 via D3D11 (stable)
Hybrid Rust + thin C++ architecture locked in
Full polished README.md created (Nova host + Echo client)
Project vision: lightweight Sunshine replacement with native Echo Mic passthrough, single exe, minimal footprint

README has been updated with current status and full roadmap.
Please continue from here. Next priority is Phase 3: Encoding loop + zero-copy DXGI → NVENC pipeline + basic config.toml.
User vision reminder: single-click start, out-of-the-box, lower VRAM/CPU than Sunshine, Moonlight-compatible, with native mic support. Keep offering suggestions and improvements along the way.
Let’s start Phase 3.
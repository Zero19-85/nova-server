mod capture;

use windows::core::Result;
use std::ffi::c_void;

// Static FFI to the C++ shim (compiled directly into our binary)
extern "C" {
    fn OpenNvEncSession(d3d11_device: *mut c_void, out_encoder: *mut *mut c_void) -> i32;
}

fn main() -> Result<()> {
    let capturer = capture::DesktopCapturer::new().expect("Failed to start capture");
    println!("✅ DXGI Desktop Duplication READY!");

    unsafe {
        let d3d_device_ptr: *mut c_void = std::mem::transmute(capturer.device);

        let mut h_encoder: *mut c_void = std::ptr::null_mut();
        let status = OpenNvEncSession(d3d_device_ptr, &mut h_encoder);

        if status == 0 {
            println!("✅ NVENC SESSION OPENED SUCCESSFULLY VIA D3D11 SHIM!");
            println!("   Encoder handle acquired — ready for encoding loop + zero-copy");
            // h_encoder is now valid and live
        } else {
            println!("❌ NVENC session failed with code: {}", status);
        }
    }

    Ok(())
}
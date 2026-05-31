mod capture;

use windows::core::Result;
use libloading::{Library, Symbol};

// --- Constants & Helper Macros ---
#[allow(dead_code)]
const NVENCAPI_MAJOR_VERSION: u32 = 13;
#[allow(dead_code)]
const NVENCAPI_MINOR_VERSION: u32 = 0;
const NVENCAPI_VERSION: u32 = NVENCAPI_MAJOR_VERSION | (NVENCAPI_MINOR_VERSION << 24);
#[allow(dead_code)]
const NVENC_FUNCTION_LIST_VER: u32 = 2;

fn nvenc_api_struct_version(ver: u32) -> u32 {
    (ver << 16) | (0x7 << 28) | (1 << 31)
}

#[repr(C)]
#[allow(dead_code)]
pub struct GUID {
    pub data1: u32, pub data2: u16, pub data3: u16, pub data4: [u8; 8],
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct NV_ENC_OPEN_ENCODE_SESSION_EX_PARAMS {
    pub version: u32,
    pub deviceType: u32,
    pub device: *mut std::ffi::c_void,
    pub reserved: *mut std::ffi::c_void,
    pub apiVersion: u32,
    pub reserved1: [u32; 253],
    pub reserved2: [*mut std::ffi::c_void; 64],
}

type CuInitFn = unsafe extern "system" fn(flags: u32) -> i32;
type CuDeviceGetFn = unsafe extern "system" fn(device: *mut i32, ordinal: i32) -> i32;
type CuCtxCreateFn = unsafe extern "system" fn(pctx: *mut *mut std::ffi::c_void, flags: u32, dev: i32) -> i32;

#[repr(C)]
#[allow(non_snake_case)]
pub struct NV_ENCODE_API_FUNCTION_LIST {
    pub version: u32,
    pub reserved: u32,
    pub nvEncOpenEncodeSession: *mut std::ffi::c_void,
    pub nvEncGetEncodeGUIDCount: *mut std::ffi::c_void,
    pub nvEncGetEncodeGUIDs: *mut std::ffi::c_void,
    pub nvEncGetEncodeProfileGUIDCount: *mut std::ffi::c_void,
    pub nvEncGetEncodeProfileGUIDs: *mut std::ffi::c_void,
    pub nvEncGetInputFormatCount: *mut std::ffi::c_void,
    pub nvEncGetInputFormats: *mut std::ffi::c_void,
    pub nvEncGetEncodeCaps: *mut std::ffi::c_void,
    pub nvEncGetEncodePresetCount: *mut std::ffi::c_void,
    pub nvEncGetEncodePresetGUIDs: *mut std::ffi::c_void,
    pub nvEncGetEncodePresetConfig: *mut std::ffi::c_void,
    pub nvEncInitializeEncoder: *mut std::ffi::c_void,
    pub nvEncCreateInputBuffer: *mut std::ffi::c_void,
    pub nvEncDestroyInputBuffer: *mut std::ffi::c_void,
    pub nvEncCreateBitstreamBuffer: *mut std::ffi::c_void,
    pub nvEncDestroyBitstreamBuffer: *mut std::ffi::c_void,
    pub nvEncEncodePicture: *mut std::ffi::c_void,
    pub nvEncLockBitstream: *mut std::ffi::c_void,
    pub nvEncUnlockBitstream: *mut std::ffi::c_void,
    pub nvEncLockInputBuffer: *mut std::ffi::c_void,
    pub nvEncUnlockInputBuffer: *mut std::ffi::c_void,
    pub nvEncGetEncodeStats: *mut std::ffi::c_void,
    pub nvEncGetSequenceParams: *mut std::ffi::c_void,
    pub nvEncRegisterAsyncEvent: *mut std::ffi::c_void,
    pub nvEncUnregisterAsyncEvent: *mut std::ffi::c_void,
    pub nvEncMapInputResource: *mut std::ffi::c_void,
    pub nvEncUnmapInputResource: *mut std::ffi::c_void,
    pub nvEncDestroyEncoder: *mut std::ffi::c_void,
    pub nvEncInvalidateRefFrames: *mut std::ffi::c_void,
    pub nvEncOpenEncodeSessionEx: *mut std::ffi::c_void,
    pub nvEncRegisterResource: *mut std::ffi::c_void,
    pub nvEncUnregisterResource: *mut std::ffi::c_void,
    pub nvEncReconfigureEncoder: *mut std::ffi::c_void,
    pub nvEncReleaseInputBuffer: *mut std::ffi::c_void,
    pub nvEncReleaseBitstreamBuffer: *mut std::ffi::c_void,
    pub reserved1: [u32; 221], 
}

type NvEncodeAPICreateInstanceFn = unsafe extern "system" fn(functions: *mut NV_ENCODE_API_FUNCTION_LIST) -> i32;

fn main() -> Result<()> {
    let _capturer = capture::DesktopCapturer::new().expect("Failed to start capture");
    println!("✅ CAPTURE PIPELINE READY!");

    unsafe {
        let cuda_lib = Library::new("nvcuda.dll").expect("Failed to load nvcuda.dll");
        let cu_init: Symbol<CuInitFn> = cuda_lib.get(b"cuInit\0").unwrap();
        let cu_device_get: Symbol<CuDeviceGetFn> = cuda_lib.get(b"cuDeviceGet\0").unwrap();
        let cu_ctx_create: Symbol<CuCtxCreateFn> = cuda_lib.get(b"cuCtxCreate_v2\0").unwrap();

        cu_init(0);
        let mut cu_device: i32 = 0;
        cu_device_get(&mut cu_device, 0);
        let mut cu_context: *mut std::ffi::c_void = std::ptr::null_mut();
        cu_ctx_create(&mut cu_context, 0, cu_device);
        println!("✅ RAW CUDA CONTEXT CREATED!");

        let nvenc_lib = Library::new("nvEncodeAPI64.dll").expect("Failed to load nvEncodeAPI64.dll");
        let create_instance: Symbol<NvEncodeAPICreateInstanceFn> = nvenc_lib.get(b"NvEncodeAPICreateInstance\0").unwrap();

        let mut functions: NV_ENCODE_API_FUNCTION_LIST = std::mem::zeroed();
        functions.version = nvenc_api_struct_version(NVENC_FUNCTION_LIST_VER);
        
        if create_instance(&mut functions) == 0 {
            println!("✅ NVENC HANDSHAKE SUCCESS!");
            let mut h_encoder: *mut std::ffi::c_void = std::ptr::null_mut();
            let mut session_params: NV_ENC_OPEN_ENCODE_SESSION_EX_PARAMS = std::mem::zeroed();
            session_params.version = nvenc_api_struct_version(1);
            session_params.deviceType = 0;
            session_params.device = cu_context;
            session_params.apiVersion = NVENCAPI_VERSION;

            let open_session_fn: unsafe extern "system" fn(*mut std::ffi::c_void, *mut NV_ENC_OPEN_ENCODE_SESSION_EX_PARAMS, *mut *mut std::ffi::c_void) -> i32 = 
                std::mem::transmute(functions.nvEncOpenEncodeSessionEx);
            
            let status = open_session_fn(std::ptr::null_mut(), &mut session_params, &mut h_encoder);
            if status == 0 { println!("🚀 SESSION OPENED SUCCESSFULLY VIA CUDA!"); } 
            else { println!("❌ Could not open session. Code: {}", status); }
        }
    }
    Ok(())
}

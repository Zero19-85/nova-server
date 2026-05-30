use windows::core::{Result, Interface};
use windows::Win32::Foundation::HMODULE; 
use windows::Win32::Graphics::Direct3D::{D3D_DRIVER_TYPE_UNKNOWN, D3D_FEATURE_LEVEL_11_1};
use windows::Win32::Graphics::Direct3D11::{
    D3D11CreateDevice, D3D11_CREATE_DEVICE_BGRA_SUPPORT, D3D11_SDK_VERSION, ID3D11Device,
    ID3D11DeviceContext,
};
use windows::Win32::Graphics::Dxgi::{
    CreateDXGIFactory1, IDXGIAdapter1, IDXGIFactory1, IDXGIOutput1, IDXGIOutputDuplication,
    DXGI_OUTDUPL_FRAME_INFO
};
use libloading::{Library, Symbol};

type NvEncodeAPICreateInstanceFn = unsafe extern "system" fn(functions: *mut NV_ENCODE_API_FUNCTION_LIST) -> i32;
type NvEncOpenEncodeSessionExFn = unsafe extern "system" fn(params: *mut NV_ENC_OPEN_ENCODE_SESSION_EX_PARAMS, session: *mut *mut std::ffi::c_void) -> i32;
type NvEncDestroyEncodeSessionFn = unsafe extern "system" fn(session: *mut std::ffi::c_void) -> i32;

#[repr(C)]
pub struct NV_ENC_OPEN_ENCODE_SESSION_EX_PARAMS {
    pub version: u32,
    pub device_type: u32,
    pub device: *mut std::ffi::c_void,
    pub reserved: *mut std::ffi::c_void,
    pub api_version: u32,
    pub reserved1: [u32; 253],
    pub reserved2: [*mut std::ffi::c_void; 64],
}

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
    pub nvEncDestroyEncoder: Option<NvEncDestroyEncodeSessionFn>,
    pub nvEncInvalidateRefFrames: *mut std::ffi::c_void,
    pub nvEncOpenEncodeSessionEx: Option<NvEncOpenEncodeSessionExFn>,
    pub nvEncRegisterResource: *mut std::ffi::c_void,
    pub nvEncUnregisterResource: *mut std::ffi::c_void,
    pub nvEncReconfigureEncoder: *mut std::ffi::c_void,
    pub padding: [*mut std::ffi::c_void; 64],
}

fn main() -> Result<()> {
    println!("🚀 Nova Silicon Core: Initializing Hardware Pipelines...");
    
    unsafe {
        // 1. Hunt for the NVIDIA GPU
        let factory: IDXGIFactory1 = CreateDXGIFactory1()?;
        let mut selected_adapter: Option<IDXGIAdapter1> = None;
        
        println!("🔎 Scanning PCIe lanes for NVIDIA Silicon...");
        for i in 0..10 {
            if let Ok(adapter) = factory.EnumAdapters1(i) {
                if let Ok(desc) = adapter.GetDesc1() {
                    if desc.VendorId == 0x10DE {
                        println!("🎮 Found NVIDIA GPU at Adapter Index {}!", i);
                        selected_adapter = Some(adapter);
                        break;
                    }
                }
            } else { break; }
        }

        let adapter = selected_adapter.expect("❌ CRITICAL: Could not find an NVIDIA GPU!");

        // 2. Build Direct3D11 Engine
        let mut device: Option<ID3D11Device> = None;
        let mut context: Option<ID3D11DeviceContext> = None;
        let mut feature_level = D3D_FEATURE_LEVEL_11_1;

        D3D11CreateDevice(
            &adapter, D3D_DRIVER_TYPE_UNKNOWN, HMODULE::default(), D3D11_CREATE_DEVICE_BGRA_SUPPORT, 
            Some(&[D3D_FEATURE_LEVEL_11_1]), D3D11_SDK_VERSION, Some(&mut device), Some(&mut feature_level), Some(&mut context)
        )?;

        let device = device.expect("Failed to bind Direct3D11 render context");

        // 3. New Screen Capture Subsystem (DXGI Desktop Duplication)
        println!("🎬 Attaching to primary desktop output monitor display...");
        
        // Grab the primary output screen hooked up to our card (Index 0 is your main monitor)
        let output = adapter.EnumOutputs(0)?;
        
        // Typecast the output interface to an IDXGIOutput1 wrapper required for desktop duplication
        let output1: IDXGIOutput1 = output.cast()?;
        
        // Activate the Windows desktop duplication stream on our Direct3D device context
        let desk_dupl: IDXGIOutputDuplication = output1.DuplicateOutput(&device)?;
        println!("✅ Screen Capture Engine initialized successfully!");

        // 4. Load NVIDIA NVENC Driver Binary Interfaces
        let nvenc_lib = Library::new("nvEncodeAPI64.dll").expect("NVIDIA Graphics Driver not found");
        
        let major_version: u32 = 12;
        let minor_version: u32 = 1;
        let api_version: u32 = major_version | (minor_version << 24); 

        let create_instance: Symbol<NvEncodeAPICreateInstanceFn> = nvenc_lib.get(b"NvEncodeAPICreateInstance\0").unwrap();
        let mut function_list: NV_ENCODE_API_FUNCTION_LIST = std::mem::zeroed();
        
        function_list.version = api_version | (2 << 16) | (0x7 << 28); 
        
        if create_instance(&mut function_list) != 0 {
            println!("❌ Function List Handshake rejected."); 
            return Ok(());
        }
        
        // 5. Request Silicon Session Allocation
        if let (Some(open_session_ex), Some(destroy_encoder)) = 
            (function_list.nvEncOpenEncodeSessionEx, function_list.nvEncDestroyEncoder) 
        {
            let mut session_handle: *mut std::ffi::c_void = std::ptr::null_mut();
            let mut session_params: NV_ENC_OPEN_ENCODE_SESSION_EX_PARAMS = std::mem::zeroed();
            
            session_params.version = api_version | (1 << 16) | (0x7 << 28); 
            session_params.device_type = 0; // Verified working Direct3D11 Index
            session_params.device = device.as_raw() as *mut std::ffi::c_void; 
            session_params.api_version = api_version;

            let status = open_session_ex(&mut session_params, &mut session_handle);
            
            if status == 0 {
                println!("✅ NVENC Session Verified and Active! (RTX 5070 Silicon Locked)");
                
                // --- TEST FRAME GRAB ---
                println!("📡 Testing active video capture frame grab loop...");
                let mut frame_info: DXGI_OUTDUPL_FRAME_INFO = std::mem::zeroed();
                let mut resource = None;
                
                // Wait for up to 250 milliseconds for a screen frame change to occur
                if desk_dupl.AcquireNextFrame(250, &mut frame_info, &mut resource).is_ok() {
                    println!("📸 Successfully pulled a live screen texture frame out of Windows display engine!");
                    let _ = desk_dupl.ReleaseFrame(); // Safely return the frame back to Windows
                } else {
                    println!("⚠️ Frame grab timeout (no screen updates occurred during test initialization).");
                }
                
                destroy_encoder(session_handle);
                println!("🛑 Session safely recycled. Pipelines clear.");
            } else {
                println!("❌ Handshake rejected. NVENC Error Code: {}", status);
            }
        }
    }
    Ok(())
}
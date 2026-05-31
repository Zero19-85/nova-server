#include <windows.h>
#include "nvEncodeAPI.h"
#include <stdio.h>

// Correct typedef using NVENCAPI calling convention (exactly as defined in the header)
typedef NVENCSTATUS (NVENCAPI *PFN_NvEncodeAPICreateInstance)(NV_ENCODE_API_FUNCTION_LIST* functionList);

extern "C" __declspec(dllexport) int OpenNvEncSession(void* d3d11_device, void** out_encoder) {
    if (!d3d11_device || !out_encoder) {
        printf("❌ Invalid parameters passed to shim\n");
        return -1;
    }

    // Dynamically load the NVIDIA NVENC DLL (no extra SDK or .lib files needed)
    HMODULE nvenc_dll = LoadLibraryA("nvEncodeAPI64.dll");
    if (!nvenc_dll) {
        printf("❌ Failed to load nvEncodeAPI64.dll\n");
        return -1;
    }

    PFN_NvEncodeAPICreateInstance create_instance = 
        (PFN_NvEncodeAPICreateInstance)GetProcAddress(nvenc_dll, "NvEncodeAPICreateInstance");
    if (!create_instance) {
        printf("❌ Failed to get NvEncodeAPICreateInstance function pointer\n");
        FreeLibrary(nvenc_dll);
        return -1;
    }

    NV_ENCODE_API_FUNCTION_LIST functionList = { 0 };
    functionList.version = NV_ENCODE_API_FUNCTION_LIST_VER;

    NVENCSTATUS status = create_instance(&functionList);
    if (status != NV_ENC_SUCCESS) {
        printf("❌ NvEncodeAPICreateInstance failed: %d\n", status);
        FreeLibrary(nvenc_dll);
        return status;
    }

    NV_ENC_OPEN_ENCODE_SESSION_EX_PARAMS params = { 0 };
    params.version     = NV_ENC_OPEN_ENCODE_SESSION_EX_PARAMS_VER;
    params.deviceType  = NV_ENC_DEVICE_TYPE_DIRECTX;   // D3D11 — works out-of-the-box
    params.device      = d3d11_device;
    params.apiVersion  = NVENCAPI_VERSION;

    status = functionList.nvEncOpenEncodeSessionEx(&params, out_encoder);
    if (status != NV_ENC_SUCCESS) {
        printf("❌ nvEncOpenEncodeSessionEx failed: %d\n", status);
    } else {
        printf("✅ NVENC SESSION OPENED SUCCESSFULLY VIA D3D11 SHIM!\n");
        printf("   Encoder handle acquired — ready for encoding loop + zero-copy\n");
    }

    FreeLibrary(nvenc_dll);
    return status;
}
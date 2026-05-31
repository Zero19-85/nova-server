#include <windows.h>
#include <nvEncodeAPI.h>

// This exports the function so Rust can see it
extern "C" __declspec(dllexport) int InitializeNvEncSession(void** session) {
    NV_ENCODE_API_FUNCTION_LIST functionList = {0};
    functionList.version = NV_ENCODE_API_FUNCTION_LIST_VER;
    
    // Attempt to create the instance using the official SDK call
    // This is the call that was failing in Rust due to ABI alignment issues
    int status = NvEncodeAPICreateInstance(&functionList);
    
    if (status == 0) {
        // If successful, we pass the session back to Rust
        // For now, we return 0 (Success)
        return 0; 
    }
    
    return status;
}
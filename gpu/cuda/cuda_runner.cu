#include "cuda_runner.h"
#include <cuda_runtime.h>
#include <string.h>
#include <stdio.h>

extern "C" CudaInitResult cuda_init(void) {
    CudaInitResult result;
    memset(&result, 0, sizeof(CudaInitResult));
    
    int deviceCount = 0;
    cudaError_t error = cudaGetDeviceCount(&deviceCount);
    
    if (error != cudaSuccess) {
        result.device_available = 0;
        snprintf(result.error_message, sizeof(result.error_message),
                "CUDA initialization failed: %s", cudaGetErrorString(error));
        return result;
    }
    
    if (deviceCount == 0) {
        result.device_available = 0;
        snprintf(result.error_message, sizeof(result.error_message),
                "No CUDA-capable devices found");
        return result;
    }
    
    // Get device properties
    cudaDeviceProp prop;
    cudaGetDeviceProperties(&prop, 0);
    
    result.device_available = 1;
    snprintf(result.error_message, sizeof(result.error_message),
            "CUDA initialized: %s", prop.name);
    
    return result;
}

extern "C" int cuda_is_available(void) {
    int deviceCount = 0;
    cudaError_t error = cudaGetDeviceCount(&deviceCount);
    return (error == cudaSuccess && deviceCount > 0) ? 1 : 0;
}

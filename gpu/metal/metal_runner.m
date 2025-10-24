#import <Metal/Metal.h>
#import <Foundation/Foundation.h>
#import "metal_runner.h"

static id<MTLDevice> device = nil;
static id<MTLCommandQueue> commandQueue = nil;

MetalInitResult metal_init(void) {
    MetalInitResult result = {0};
    
    fprintf(stderr, "[Metal C] Calling MTLCreateSystemDefaultDevice()...\n");
    fflush(stderr);
    
    device = MTLCreateSystemDefaultDevice();
    
    fprintf(stderr, "[Metal C] Device pointer: %p\n", (__bridge void*)device);
    fflush(stderr);
    
    if (!device) {
        result.device_available = 0;
        fprintf(stderr, "[Metal C] ERROR: Device is NULL\n");
        fflush(stderr);
        snprintf(result.error_message, sizeof(result.error_message), 
                "Metal is not supported on this device");
        return result;
    }
    
    NSString* deviceName = [device name];
    fprintf(stderr, "[Metal C] Device name: %s\n", [deviceName UTF8String]);
    fflush(stderr);
    
    commandQueue = [device newCommandQueue];
    if (!commandQueue) {
        result.device_available = 0;
        fprintf(stderr, "[Metal C] ERROR: Failed to create command queue\n");
        fflush(stderr);
        snprintf(result.error_message, sizeof(result.error_message), 
                "Failed to create Metal command queue");
        return result;
    }
    
    fprintf(stderr, "[Metal C] SUCCESS: Metal fully initialized!\n");
    fflush(stderr);
    
    result.device_available = 1;
    snprintf(result.error_message, sizeof(result.error_message), 
            "Metal initialized successfully");
    return result;
}

void metal_cleanup(void) {
    commandQueue = nil;
    device = nil;
}

int metal_is_available(void) {
    return device != nil ? 1 : 0;
}

MetalExecuteResult metal_execute_kernel(const char* kernel_source, float* input_data, int data_size) {
    MetalExecuteResult result = {NULL, 0};
    
    if (!device || !commandQueue) {
        return result;
    }
    
    @autoreleasepool {
        NSError* error = nil;
        NSString* sourceString = [NSString stringWithUTF8String:kernel_source];
        
        // Compile the Metal shader
        id<MTLLibrary> library = [device newLibraryWithSource:sourceString 
                                                      options:nil 
                                                        error:&error];
        if (!library) {
            NSLog(@"[Metal] Failed to create library: %@", error);
            return result;
        }
        
        id<MTLFunction> kernelFunction = [library newFunctionWithName:@"compute_kernel"];
        if (!kernelFunction) {
            NSLog(@"[Metal] Failed to find kernel function 'compute_kernel'");
            return result;
        }
        
        id<MTLComputePipelineState> pipelineState = [device newComputePipelineStateWithFunction:kernelFunction 
                                                                                          error:&error];
        if (!pipelineState) {
            NSLog(@"[Metal] Failed to create pipeline state: %@", error);
            return result;
        }
        
        // Create input and output buffers
        id<MTLBuffer> inputBuffer = [device newBufferWithBytes:input_data
                                                        length:data_size * sizeof(float)
                                                       options:MTLResourceStorageModeShared];
        
        id<MTLBuffer> outputBuffer = [device newBufferWithLength:data_size * sizeof(float)
                                                         options:MTLResourceStorageModeShared];
        
        // Create command buffer and encoder
        id<MTLCommandBuffer> commandBuffer = [commandQueue commandBuffer];
        id<MTLComputeCommandEncoder> encoder = [commandBuffer computeCommandEncoder];
        
        [encoder setComputePipelineState:pipelineState];
        [encoder setBuffer:inputBuffer offset:0 atIndex:0];
        [encoder setBuffer:outputBuffer offset:0 atIndex:1];
        
        // Calculate thread groups
        NSUInteger threadGroupSize = pipelineState.maxTotalThreadsPerThreadgroup;
        if (threadGroupSize > 256) threadGroupSize = 256;
        
        MTLSize threadgroupSize = MTLSizeMake(threadGroupSize, 1, 1);
        MTLSize gridSize = MTLSizeMake(data_size, 1, 1);
        
        [encoder dispatchThreads:gridSize threadsPerThreadgroup:threadgroupSize];
        [encoder endEncoding];
        
        [commandBuffer commit];
        [commandBuffer waitUntilCompleted];
        
        // Copy output data
        float* output_data = (float*)malloc(data_size * sizeof(float));
        if (output_data) {
            memcpy(output_data, [outputBuffer contents], data_size * sizeof(float));
            result.data = output_data;
            result.size = data_size;
            printf("[Metal] ✓ Kernel executed on GPU with %d elements\n", data_size);
        }
        
        return result;
    }
}

void metal_free_result(MetalExecuteResult result) {
    if (result.data) {
        free(result.data);
    }
}

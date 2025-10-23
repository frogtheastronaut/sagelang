#import <Metal/Metal.h>
#import <Foundation/Foundation.h>
#import "metal_runner.h"

static id<MTLDevice> device = nil;
static id<MTLCommandQueue> commandQueue = nil;

MetalInitResult metal_init(void) {
    MetalInitResult result = {0};
    
    device = MTLCreateSystemDefaultDevice();
    if (!device) {
        result.device_available = 0;
        snprintf(result.error_message, sizeof(result.error_message), 
                "Metal is not supported on this device");
        return result;
    }
    
    commandQueue = [device newCommandQueue];
    if (!commandQueue) {
        result.device_available = 0;
        snprintf(result.error_message, sizeof(result.error_message), 
                "Failed to create Metal command queue");
        return result;
    }
    
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

int metal_execute_kernel(const char* kernel_source, MetalBuffer* buffers, size_t buffer_count) {
    if (!device || !commandQueue) {
        return -1;
    }
    
    @autoreleasepool {
        NSError* error = nil;
        NSString* sourceString = [NSString stringWithUTF8String:kernel_source];
        
        id<MTLLibrary> library = [device newLibraryWithSource:sourceString 
                                                      options:nil 
                                                        error:&error];
        if (!library) {
            NSLog(@"Failed to create library: %@", error);
            return -2;
        }
        
        id<MTLFunction> kernelFunction = [library newFunctionWithName:@"compute_kernel"];
        if (!kernelFunction) {
            NSLog(@"Failed to find kernel function");
            return -3;
        }
        
        id<MTLComputePipelineState> pipelineState = [device newComputePipelineStateWithFunction:kernelFunction 
                                                                                          error:&error];
        if (!pipelineState) {
            NSLog(@"Failed to create pipeline state: %@", error);
            return -4;
        }
        
        id<MTLCommandBuffer> commandBuffer = [commandQueue commandBuffer];
        id<MTLComputeCommandEncoder> encoder = [commandBuffer computeCommandEncoder];
        
        [encoder setComputePipelineState:pipelineState];
        
        for (size_t i = 0; i < buffer_count; i++) {
            if (buffers[i].input_data && buffers[i].input_size > 0) {
                id<MTLBuffer> inputBuffer = [device newBufferWithBytes:buffers[i].input_data
                                                               length:buffers[i].input_size * sizeof(float)
                                                              options:MTLResourceStorageModeShared];
                [encoder setBuffer:inputBuffer offset:0 atIndex:i];
            }
        }
        
        NSUInteger threadGroupSize = pipelineState.maxTotalThreadsPerThreadgroup;
        if (threadGroupSize > 256) threadGroupSize = 256;
        
        MTLSize threadgroupSize = MTLSizeMake(threadGroupSize, 1, 1);
        MTLSize gridSize = MTLSizeMake(buffers[0].input_size, 1, 1);
        
        [encoder dispatchThreads:gridSize threadsPerThreadgroup:threadgroupSize];
        [encoder endEncoding];
        
        [commandBuffer commit];
        [commandBuffer waitUntilCompleted];
        
        return 0;
    }
}

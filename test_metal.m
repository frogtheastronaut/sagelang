#import <Metal/Metal.h>
#import <Foundation/Foundation.h>
#import <stdio.h>

int main() {
    @autoreleasepool {
        printf("Testing Metal device creation...\n");
        
        id<MTLDevice> device = MTLCreateSystemDefaultDevice();
        
        if (device) {
            NSString* name = [device name];
            printf("SUCCESS: Metal device found: %s\n", [name UTF8String]);
            printf("Device supports: family Apple %ld\n", (long)[device supportsFamily:MTLGPUFamilyApple7]);
            return 0;
        } else {
            printf("ERROR: MTLCreateSystemDefaultDevice() returned nil\n");
            printf("This should not happen on M1 Mac\n");
            return 1;
        }
    }
}

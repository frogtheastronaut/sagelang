#ifndef METAL_RUNNER_H
#define METAL_RUNNER_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    int device_available;
    char error_message[256];
} MetalInitResult;

typedef struct {
    float* input_data;
    size_t input_size;
    float* output_data;
    size_t output_size;
} MetalBuffer;

MetalInitResult metal_init(void);
void metal_cleanup(void);
int metal_execute_kernel(const char* kernel_source, MetalBuffer* buffers, size_t buffer_count);
int metal_is_available(void);

#ifdef __cplusplus
}
#endif

#endif

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
    float* data;
    int size;
} MetalExecuteResult;

MetalInitResult metal_init(void);
int metal_is_available(void);
MetalExecuteResult metal_execute_kernel(const char* kernel_source, float* input_data, int data_size);
void metal_free_result(MetalExecuteResult result);

#ifdef __cplusplus
}
#endif

#endif

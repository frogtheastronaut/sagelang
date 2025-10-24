#ifndef CUDA_RUNNER_H
#define CUDA_RUNNER_H

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    int device_available;
    char error_message[256];
} CudaInitResult;

CudaInitResult cuda_init(void);
int cuda_is_available(void);

#ifdef __cplusplus
}
#endif

#endif // CUDA_RUNNER_H

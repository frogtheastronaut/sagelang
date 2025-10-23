# Metal GPU Support in SageLang

This guide explains how to use Metal GPU acceleration in your SageLang code.

## Prerequisites

- macOS (Metal is only available on Apple platforms)
- Xcode Command Line Tools installed

## Syntax

```sage
use_metal {
    "METAL_KERNEL_CODE_AS_STRING";
    
    // Your code that will benefit from GPU acceleration
    // ...
}
```

## Example: Simple Vector Doubling

```sage
use_metal {
    "#include <metal_stdlib>
    using namespace metal;
    
    kernel void compute_kernel(device float* input [[buffer(0)]],
                              device float* output [[buffer(1)]],
                              uint id [[thread_position_in_grid]]) {
        output[id] = input[id] * 2.0;
    }";
    
    let data = [1, 2, 3, 4, 5];
    print data;
}
```

## How It Works

1. **Kernel Definition**: The Metal shader code is provided as a string literal
2. **Compilation**: The Metal kernel is compiled at runtime by the Metal framework
3. **Execution**: Data from your SageLang code is transferred to GPU memory
4. **Processing**: The GPU executes the kernel in parallel across all data elements
5. **Return**: Results are transferred back to CPU memory

## Metal Kernel Structure

```metal
#include <metal_stdlib>
using namespace metal;

kernel void compute_kernel(
    device float* input [[buffer(0)]],   // Input buffer
    device float* output [[buffer(1)]],  // Output buffer
    uint id [[thread_position_in_grid]]  // Thread ID
) {
    // Your GPU code here
    output[id] = input[id] * 2.0;
}
```

## Limitations

- Currently supports only numeric data (floats/numbers)
- Kernel must be named `compute_kernel`
- Data transfer happens implicitly
- Only available on macOS

## Performance Tips

1. **Use for large datasets**: GPU acceleration is most beneficial for arrays with 1000+ elements
2. **Minimize data transfers**: Keep data on GPU as long as possible
3. **Parallelize operations**: Design kernels that work independently on each element
4. **Profile your code**: Use `--debug` flag to see execution details

## Compilation

The Metal GPU support requires additional build dependencies. The project automatically links against the Metal framework on macOS.

## Future Enhancements

- Support for multiple kernel functions
- Explicit buffer management
- Support for compute/Vulkan on other platforms
- Automatic data type conversion
- GPU memory pooling

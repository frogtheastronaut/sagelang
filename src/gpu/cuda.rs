#[cfg(all(unix, not(target_os = "macos")))]
use cuda_driver_sys::*;
#[cfg(all(unix, not(target_os = "macos")))]
use std::ffi::CString;

#[cfg(all(unix, not(target_os = "macos")))]
pub struct CudaContext {
    context: CUcontext,
    module: Option<CUmodule>,
}

#[cfg(all(unix, not(target_os = "macos")))]
impl CudaContext {
    pub fn new() -> Result<Self, String> {
        unsafe {
            // Initialize CUDA
            let result = cuInit(0);
            if result != CUresult::CUDA_SUCCESS {
                return Err(format!("Failed to initialize CUDA: error code {}", result as i32));
            }

            // Get device
            let mut device: CUdevice = 0;
            let result = cuDeviceGet(&mut device, 0);
            if result != CUresult::CUDA_SUCCESS {
                return Err(format!("Failed to get CUDA device: error code {}", result as i32));
            }

            // Create context
            let mut context: CUcontext = std::ptr::null_mut();
            let result = cuCtxCreate_v2(&mut context, 0, device);
            if result != CUresult::CUDA_SUCCESS {
                return Err(format!("Failed to create CUDA context: error code {}", result as i32));
            }

            Ok(CudaContext {
                context,
                module: None,
            })
        }
    }

    pub fn load_kernel(&mut self, source: &str) -> Result<(), String> {
        unsafe {
            let source_cstr = CString::new(source)
                .map_err(|_| "Invalid kernel source".to_string())?;

            let mut module: CUmodule = std::ptr::null_mut();
            let result = cuModuleLoadData(&mut module, source_cstr.as_ptr() as *const _);
            if result != CUresult::CUDA_SUCCESS {
                return Err(format!("Failed to load CUDA module: error code {}", result as i32));
            }

            self.module = Some(module);
            Ok(())
        }
    }

    pub fn execute(&mut self, input_data: &[f32]) -> Result<Vec<f32>, String> {
        let module = self.module
            .ok_or_else(|| "No kernel loaded".to_string())?;

        unsafe {
            // Get kernel function
            let kernel_name = CString::new("sage_kernel")
                .map_err(|_| "Invalid kernel name".to_string())?;
            let mut kernel: CUfunction = std::ptr::null_mut();
            let result = cuModuleGetFunction(&mut kernel, module, kernel_name.as_ptr());
            if result != CUresult::CUDA_SUCCESS {
                return Err(format!("Failed to get kernel function: error code {}", result as i32));
            }

            // Allocate device memory
            let size = (input_data.len() * std::mem::size_of::<f32>()) as usize;
            let mut d_input: CUdeviceptr = 0;
            let mut d_output: CUdeviceptr = 0;

            let result = cuMemAlloc_v2(&mut d_input, size);
            if result != CUresult::CUDA_SUCCESS {
                return Err(format!("Failed to allocate input memory: error code {}", result as i32));
            }

            let result = cuMemAlloc_v2(&mut d_output, size);
            if result != CUresult::CUDA_SUCCESS {
                cuMemFree_v2(d_input);
                return Err(format!("Failed to allocate output memory: error code {}", result as i32));
            }

            // Copy input to device
            let result = cuMemcpyHtoD_v2(d_input, input_data.as_ptr() as *const _, size);
            if result != CUresult::CUDA_SUCCESS {
                cuMemFree_v2(d_input);
                cuMemFree_v2(d_output);
                return Err(format!("Failed to copy input to device: error code {}", result as i32));
            }

            // Launch kernel
            let mut args = [
                &d_input as *const _ as *mut std::ffi::c_void,
                &d_output as *const _ as *mut std::ffi::c_void,
            ];

            let block_size = 256;
            let grid_size = (input_data.len() + block_size - 1) / block_size;

            let result = cuLaunchKernel(
                kernel,
                grid_size as u32, 1, 1,  // grid dim
                block_size as u32, 1, 1,  // block dim
                0,  // shared memory
                std::ptr::null_mut(),  // stream
                args.as_mut_ptr(),
                std::ptr::null_mut(),
            );

            if result != CUresult::CUDA_SUCCESS {
                cuMemFree_v2(d_input);
                cuMemFree_v2(d_output);
                return Err(format!("Failed to launch kernel: error code {}", result as i32));
            }

            // Wait for completion
            let result = cuCtxSynchronize();
            if result != CUresult::CUDA_SUCCESS {
                cuMemFree_v2(d_input);
                cuMemFree_v2(d_output);
                return Err(format!("Kernel execution failed: error code {}", result as i32));
            }

            // Copy output back
            let mut output = vec![0.0f32; input_data.len()];
            let result = cuMemcpyDtoH_v2(output.as_mut_ptr() as *mut _, d_output, size);
            
            // Free device memory
            cuMemFree_v2(d_input);
            cuMemFree_v2(d_output);

            if result != CUresult::CUDA_SUCCESS {
                return Err(format!("Failed to copy output from device: error code {}", result as i32));
            }

            Ok(output)
        }
    }
}

#[cfg(all(unix, not(target_os = "macos")))]
impl Drop for CudaContext {
    fn drop(&mut self) {
        unsafe {
            if !self.context.is_null() {
                cuCtxDestroy_v2(self.context);
            }
        }
    }
}

#[cfg(not(all(unix, not(target_os = "macos"))))]
pub struct CudaContext;

#[cfg(not(all(unix, not(target_os = "macos"))))]
impl CudaContext {
    pub fn new() -> Result<Self, String> {
        Err("CUDA is only available on Linux/Unix systems (not macOS)".to_string())
    }

    pub fn load_kernel(&mut self, _source: &str) -> Result<(), String> {
        Err("CUDA is only available on Linux/Unix systems (not macOS)".to_string())
    }

    pub fn execute(&mut self, _input_data: &[f32]) -> Result<Vec<f32>, String> {
        Err("CUDA is only available on Linux/Unix systems (not macOS)".to_string())
    }
}

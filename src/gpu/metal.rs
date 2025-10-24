#[cfg(target_os = "macos")]
use metal::*;

#[cfg(target_os = "macos")]
pub struct MetalContext {
    device: Device,
    command_queue: CommandQueue,
    pipeline_state: Option<ComputePipelineState>,
}

#[cfg(target_os = "macos")]
impl MetalContext {
    pub fn new() -> Result<Self, String> {
        let device = Device::system_default()
            .ok_or_else(|| {
                "Failed to get default Metal device. This could be due to:\n\
                 1. No GPU/Metal support on this system\n\
                 2. Missing entitlements (macOS sandboxing)\n\
                 3. GPU driver issues".to_string()
            })?;
        
        let command_queue = device.new_command_queue();
        
        Ok(MetalContext {
            device,
            command_queue,
            pipeline_state: None,
        })
    }

    pub fn load_kernel(&mut self, source: &str) -> Result<(), String> {
        let options = CompileOptions::new();
        let library = self.device
            .new_library_with_source(source, &options)
            .map_err(|e| format!("Failed to compile Metal shader: {}", e))?;
        
        let kernel_function = library
            .get_function("compute_kernel", None)
            .map_err(|e| format!("Failed to get kernel function: {}", e))?;
        
        let pipeline_state = self.device
            .new_compute_pipeline_state_with_function(&kernel_function)
            .map_err(|e| format!("Failed to create pipeline state: {}", e))?;
        
        self.pipeline_state = Some(pipeline_state);
        Ok(())
    }

    pub fn execute(&mut self, input_data: &[f32]) -> Result<Vec<f32>, String> {
        let pipeline_state = self.pipeline_state
            .as_ref()
            .ok_or_else(|| "No kernel loaded".to_string())?;
        
        // Create input buffer
        let input_buffer = self.device.new_buffer_with_data(
            input_data.as_ptr() as *const _,
            (input_data.len() * std::mem::size_of::<f32>()) as u64,
            MTLResourceOptions::StorageModeShared,
        );
        
        // Create output buffer (same size as input)
        let output_buffer = self.device.new_buffer(
            (input_data.len() * std::mem::size_of::<f32>()) as u64,
            MTLResourceOptions::StorageModeShared,
        );
        
        // Create command buffer and encoder
        let command_buffer = self.command_queue.new_command_buffer();
        let compute_encoder = command_buffer.new_compute_command_encoder();
        
        compute_encoder.set_compute_pipeline_state(pipeline_state);
        compute_encoder.set_buffer(0, Some(&input_buffer), 0);
        compute_encoder.set_buffer(1, Some(&output_buffer), 0);
        
        // Determine thread group sizes
        let thread_execution_width = pipeline_state.thread_execution_width();
        let max_total_threads = pipeline_state.max_total_threads_per_threadgroup();
        
        let threadgroup_size = MTLSize {
            width: thread_execution_width.min(max_total_threads),
            height: 1,
            depth: 1,
        };
        
        let threadgroup_count = MTLSize {
            width: (input_data.len() as u64 + threadgroup_size.width - 1) / threadgroup_size.width,
            height: 1,
            depth: 1,
        };
        
        compute_encoder.dispatch_thread_groups(threadgroup_count, threadgroup_size);
        compute_encoder.end_encoding();
        
        command_buffer.commit();
        command_buffer.wait_until_completed();
        
        // Read results
        let output_ptr = output_buffer.contents() as *const f32;
        let output = unsafe {
            std::slice::from_raw_parts(output_ptr, input_data.len()).to_vec()
        };
        
        Ok(output)
    }
}

#[cfg(not(target_os = "macos"))]
pub struct MetalContext;

#[cfg(not(target_os = "macos"))]
impl MetalContext {
    pub fn new() -> Result<Self, String> {
        Err("Metal is only available on macOS".to_string())
    }

    pub fn load_kernel(&mut self, _source: &str) -> Result<(), String> {
        Err("Metal is only available on macOS".to_string())
    }

    pub fn execute(&mut self, _input_data: &[f32]) -> Result<Vec<f32>, String> {
        Err("Metal is only available on macOS".to_string())
    }
}

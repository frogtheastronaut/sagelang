pub mod metal;
pub mod cuda;

use metal::MetalContext;
use cuda::CudaContext;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GpuBackend {
    Metal,
    Cuda,
    None,
}

pub enum GpuContext {
    Metal(MetalContext),
    Cuda(CudaContext),
    None,
}

pub struct GpuState {
    pub initialized: bool,
    pub backend: GpuBackend,
    context: GpuContext,
}

impl GpuState {
    pub fn new() -> Self {
        Self {
            initialized: false,
            backend: GpuBackend::None,
            context: GpuContext::None,
        }
    }
    
    pub fn init(&mut self) -> Result<(), String> {
        // Try Metal first (macOS)
        #[cfg(target_os = "macos")]
        {
            match MetalContext::new() {
                Ok(ctx) => {
                    self.initialized = true;
                    self.backend = GpuBackend::Metal;
                    self.context = GpuContext::Metal(ctx);
                    return Ok(());
                }
                Err(e) => {
                    println!("[GPU] Metal init failed: {}", e);
                }
            }
        }
        
        // Try CUDA
        #[cfg(all(unix, not(target_os = "macos")))]
        {
            match CudaContext::new() {
                Ok(ctx) => {
                    self.initialized = true;
                    self.backend = GpuBackend::Cuda;
                    self.context = GpuContext::Cuda(ctx);
                    return Ok(());
                }
                Err(e) => {
                    println!("[GPU] CUDA init failed: {}", e);
                }
            }
        }
        
        Err("No GPU backend available".to_string())
    }
    
    pub fn load_kernel(&mut self, kernel: String) -> Result<(), String> {
        match &mut self.context {
            GpuContext::Metal(ctx) => ctx.load_kernel(&kernel),
            GpuContext::Cuda(ctx) => ctx.load_kernel(&kernel),
            GpuContext::None => Err("No GPU backend available".to_string()),
        }
    }
    
    pub fn execute(&mut self, input_data: &[f32]) -> Result<Vec<f32>, String> {
        if !self.initialized {
            return Err("GPU not initialized".to_string());
        }
        
        match &mut self.context {
            GpuContext::Metal(ctx) => ctx.execute(input_data),
            GpuContext::Cuda(ctx) => ctx.execute(input_data),
            GpuContext::None => Err("No GPU backend available".to_string()),
        }
    }
}

pub mod metal;

pub struct GpuState {
    pub initialized: bool,
    pub kernel_code: Option<String>,
}

impl GpuState {
    pub fn new() -> Self {
        Self {
            initialized: false,
            kernel_code: None,
        }
    }
    
    pub fn init(&mut self) -> Result<(), String> {
        #[cfg(target_os = "macos")]
        {
            metal::init()?;
            self.initialized = true;
            Ok(())
        }
        
        #[cfg(not(target_os = "macos"))]
        Err("Metal is only available on macOS".to_string())
    }
    
    pub fn load_kernel(&mut self, kernel: String) {
        self.kernel_code = Some(kernel);
    }
}

#[cfg(target_os = "macos")]
#[repr(C)]
struct MetalInitResult {
    device_available: i32,
    error_message: [u8; 256],
}

#[cfg(target_os = "macos")]
unsafe extern "C" {
    fn metal_init() -> MetalInitResult;
}

#[cfg(target_os = "macos")]
pub fn init() -> Result<(), String> {
    unsafe {
        let result = metal_init();
        if result.device_available != 0 {
            Ok(())
        } else {
            let msg = std::ffi::CStr::from_ptr(result.error_message.as_ptr() as *const i8)
                .to_string_lossy()
                .to_string();
            Err(msg)
        }
    }
}

#[cfg(not(target_os = "macos"))]
pub fn init() -> Result<(), String> {
    Err("Metal is only available on macOS".to_string())
}

fn main() {
    #[cfg(target_os = "macos")]
    {
        cc::Build::new()
            .file("gpu/metal/metal_runner.m")
            .flag("-fobjc-arc")
            .compile("metal_runner");
        
        println!("cargo:rustc-link-lib=framework=Metal");
        println!("cargo:rustc-link-lib=framework=Foundation");
        println!("cargo:rerun-if-changed=gpu/metal/metal_runner.m");
        println!("cargo:rerun-if-changed=gpu/metal/metal_runner.h");
    }
}

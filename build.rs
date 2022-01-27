use std::env;

fn main() {
    if cfg!(target_os = "linux") {
        match env::var("VULKAN_SDK") {
            Ok(path) => {
                println!("cargo:rustc-link-search={}/lib", path);
            }
            Err(_) => (),
        };
        println!("cargo:rustc-link-lib=vulkan");
    }
}

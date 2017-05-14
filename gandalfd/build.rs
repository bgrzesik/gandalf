
use std::env;


fn main() {
    if let Ok(profile) = env::var("PROFILE") {
        println!("cargo:rustc-cfg=build={:?}", profile);
    }
    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=static=Packet");
        println!("cargo:rustc-link-search=static={}\\lib",
                 env!("CARGO_MANIFEST_DIR"));
    }
}

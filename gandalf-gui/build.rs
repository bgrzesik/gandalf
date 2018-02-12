

fn main() {
    if let Ok(profile) = std::env::var("PROFILE") {
        println!("cargo:rustc-cfg=build={:?}", profile);
    }
    println!("cargo:rustc-link-lib=static=legacy_stdio_definitions");
}

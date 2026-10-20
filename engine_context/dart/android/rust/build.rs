fn main() {
    let target_system = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    // 16 KB page alignment for Android arm64 and x86_64.
    // https://developer.android.com/guide/practices/page-sizes#other-build-systems
    if target_system == "android"
        && (target_arch == "aarch64" || target_arch == "x86_64")
    {
        println!("cargo:rustc-link-arg=-Wl,-z,max-page-size=16384");
    }
}

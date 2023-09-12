fn main() {
    println!("cargo:rustc-link-search=libvomid");
    println!("cargo:rustc-link-lib=asound");
}

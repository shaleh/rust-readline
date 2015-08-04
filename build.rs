#[cfg(target_os = "macos")]
fn main() {
    println!("cargo:rustc-flags=-l edit");
}

#[cfg(target_os = "linux")]
fn main() {
    println!("cargo:rustc-flags=-l readline");
}

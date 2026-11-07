// the face's stack: an import walks a whole game by recursion and the tick's frames are a debug build's, so more than the linker's 1 MB
fn main() {
    println!("cargo:rustc-cdylib-link-arg=-zstack-size=67108864");
    println!("cargo:rerun-if-changed=build.rs");
}

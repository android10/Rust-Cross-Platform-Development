// Cargo targets: https://doc.rust-lang.org/cargo/reference/cargo-targets.html#binaries

#[path="../../build.rs"] mod build;

fn main() {
    println!("PUBLISH: {}", &build::ANDROID_TARGETS.len());
    println!("rust-library/target/armv7-linux-androideabi/release/libcryptor_jni.so");
}
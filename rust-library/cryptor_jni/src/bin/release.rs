// Cargo targets: https://doc.rust-lang.org/cargo/reference/cargo-targets.html#binaries

#[path="../../build.rs"] mod build;

use cryptor_global::console;

fn main() {
    console::out("HERE RELEASE!!!");
    console::out(&build::ANDROID_TARGETS.len().to_string());
    console::out("rust-library/target/armv7-linux-androideabi/release/libcryptor_jni.so");
}


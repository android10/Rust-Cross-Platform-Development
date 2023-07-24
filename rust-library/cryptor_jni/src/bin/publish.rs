// Cargo targets: https://doc.rust-lang.org/cargo/reference/cargo-targets.html#binaries

// https://doc.rust-lang.org/reference/items/modules.html
#[path="../../build.rs"] 
mod build;

use std::env;
use std::path::PathBuf;
use std::fs::File;
use std::io::Result;
use std::io::ErrorKind;

use cryptor_global::console;

static LIB_CRYPTOR_JNI_FILENAME: &str = "libcryptor_jni.so";


fn main() {
    println!("PUBLISH: {}", &build::ANDROID_TARGETS_CONFIG.len());
    println!("rust-library/target/armv7-linux-androideabi/release/libcryptor_jni.so");

    // https://github.com/rust-lang/cargo/issues/6100
    // For release then it is easier to change the directory

    //Check 
    //https://stackoverflow.com/questions/46749360/how-to-get-only-the-directory-portion-of-the-current-executables-path

    let target_dir = env::var("PWD").unwrap();
    let file_path = PathBuf::from("../../").into_os_string().into_string().unwrap();

    for target in build::ANDROID_TARGETS_CONFIG.keys() {

        console::print(format!("Android Target --> {}", &target));
        console::print(format!("Release Directory --> {}", &target_dir));
        // let command_args = build_command_args_for_target(&target.to_owned());
    }
}
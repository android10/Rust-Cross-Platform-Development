// Cargo targets: https://doc.rust-lang.org/cargo/reference/cargo-targets.html#binaries

// https://doc.rust-lang.org/reference/items/modules.html
#[path="../../build.rs"] 
mod build;

use std::env;
use std::path::PathBuf;
use std::path::MAIN_SEPARATOR_STR;

use cryptor_global::{console, io};

///
/// Returns the directory path where the release 
/// version of this crate is placed.
/// 
/// ## Example
/// 
/// `$ rust-library/target`
/// 
fn release_target_dir_path() -> String {
    let current_dir_path = env::current_dir().expect(
        "Cannot read current directory"
    );
    let target_dir_path = current_dir_path.parent().expect(
        "Cannot find/read 'target' directory"
    );
    
    target_dir_path.as_os_str().to_str().expect(
        "Cannot validate 'target' directory"
    ).to_owned()
}

///
/// Returns the jni directory path in the android project 
/// where the release version of this crate should be 
/// placed.
/// 
/// ## Example
/// 
/// `$ android-sample/app/src/main/jniLibs`
///
fn android_jni_file_path_for_target(android_target: &str) -> String {
    "".to_owned()
}

///
/// Returns the file path where the 
/// release version of this crate 
/// is placed ('libcryptor_jni.so').
/// 
/// ## Example
/// 
/// `$ rust-library/target/x86_64-linux-android/release/libcryptor_jni.so`
/// 
fn crate_file_path_for_target(target_release_path: &str, android_target: &str) -> String {
    let mut crate_lib_file_path = target_release_path.to_owned();

    crate_lib_file_path.push_str(MAIN_SEPARATOR_STR);
    crate_lib_file_path.push_str("target");
    crate_lib_file_path.push_str(MAIN_SEPARATOR_STR);
    crate_lib_file_path.push_str(&android_target);
    crate_lib_file_path.push_str(MAIN_SEPARATOR_STR);
    crate_lib_file_path.push_str("release");
    crate_lib_file_path.push_str(MAIN_SEPARATOR_STR);
    crate_lib_file_path.push_str("libcryptor_jni.so");

    crate_lib_file_path
}

fn publish_jni_lib_to_android_project() {
    let release_target_path = release_target_dir_path();

    for android_target in build::ANDROID_TARGETS_CONFIG.keys() {
        let crate_lib_file_path = crate_file_path_for_target(&release_target_path, &android_target);
        let android_lib_file_path = android_jni_file_path_for_target(&android_target);

        if PathBuf::from(&crate_lib_file_path).exists() && 
        PathBuf::from(&android_lib_file_path).exists() {
            console::out("it exists!!!");
            console::print(format!("Android Target {} succesfully copied!!!", &android_target));
            console::print(format!("File: {}", &crate_lib_file_path));
        } else {
            println!("ERROR!!!");
        }
    }
}

fn main() {
    publish_jni_lib_to_android_project();
}
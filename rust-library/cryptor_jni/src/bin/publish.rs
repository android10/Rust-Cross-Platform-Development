// Cargo targets: https://doc.rust-lang.org/cargo/reference/cargo-targets.html#binaries

// https://doc.rust-lang.org/reference/items/modules.html
#[path="../../build.rs"] 
mod build;

use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::path::MAIN_SEPARATOR_STR;

use cryptor_global::{console, io};

// Represents the crate/lib file name generated
static JNI_LIB_FILE_NAME: &str = "libcryptor_jni.so";

///
/// Returns the project directory path.
/// 
/// ## Examples
/// 
/// `$ rust-library/`
/// 
fn project_dir_path() -> String {
    let current_dir_path = env::current_dir().expect(
        "Cannot read current directory"
    );
    let target_dir_path = current_dir_path.parent().expect(
        "Cannot find/read 'rust-library' directory"
    );
    
    target_dir_path.as_os_str().to_str().expect(
        "Cannot validate 'rust-library' directory"
    ).to_owned()
}

///
/// Returns the jni directory path in the android project 
/// where the release version of this crate should be 
/// placed.
/// 
/// ## Arguments
///
/// * `android_jni_lib_folder` - A string slice that holds the name of the android target.
/// 
/// ## Examples
/// 
/// `$ android-sample/app/src/main/jniLibs`
///
fn android_jni_dir_path(android_jni_lib_folder: &str) -> String {
    let project_dir = PathBuf::from(project_dir_path());
    let android_project_dir_path = project_dir.parent().expect(
        "Cannot find/read 'android-sample' directory"
    );
    
    let mut android_jni_file_path = android_project_dir_path.as_os_str().to_str().expect(
        "Cannot validate 'android-sample' directory"
    ).to_owned();

    android_jni_file_path.push_str(MAIN_SEPARATOR_STR);
    android_jni_file_path.push_str("android-sample");
    android_jni_file_path.push_str(MAIN_SEPARATOR_STR);
    android_jni_file_path.push_str("app");
    android_jni_file_path.push_str(MAIN_SEPARATOR_STR);
    android_jni_file_path.push_str("src");
    android_jni_file_path.push_str(MAIN_SEPARATOR_STR);
    android_jni_file_path.push_str("main");
    android_jni_file_path.push_str(MAIN_SEPARATOR_STR);
    android_jni_file_path.push_str("jniLibs");
    android_jni_file_path.push_str(MAIN_SEPARATOR_STR);
    android_jni_file_path.push_str(&android_jni_lib_folder);
    android_jni_file_path.push_str(MAIN_SEPARATOR_STR);
    android_jni_file_path.push_str(JNI_LIB_FILE_NAME);
    
    android_jni_file_path
}

///
/// Returns the file path where the 
/// release version of this crate 
/// is placed.
/// 
/// ## Arguments
///
/// * `project_dir_path` - A string slice that holds thsi project directory path.
/// * `android_target` - A string slice that holds the name of the android target.
/// 
/// ## Examples
/// 
/// `$ rust-library/target/x86_64-linux-android/release/JNI_LIB_FILE_NAME`
/// 
fn crate_file_path_for_target(project_dir_path: &str, android_target: &str) -> String {
    let mut crate_lib_file_path = project_dir_path.to_owned();

    crate_lib_file_path.push_str(MAIN_SEPARATOR_STR);
    crate_lib_file_path.push_str("target");
    crate_lib_file_path.push_str(MAIN_SEPARATOR_STR);
    crate_lib_file_path.push_str(&android_target);
    crate_lib_file_path.push_str(MAIN_SEPARATOR_STR);
    crate_lib_file_path.push_str("release");
    crate_lib_file_path.push_str(MAIN_SEPARATOR_STR);
    crate_lib_file_path.push_str(JNI_LIB_FILE_NAME);

    crate_lib_file_path
}

fn publish_jni_lib_to_android_project() -> Result<String, Box<dyn Error>> {
    let project_dir_path = project_dir_path();

    // we loop through all android targets
    for android_target in build::ANDROID_TARGETS_CONFIG.keys() {
        // get the path of the 'JNI_LIB_FILE_NAME' file.
        let crate_lib_file_path = crate_file_path_for_target(&project_dir_path, &android_target);
        
        // get the jni android folder name to place our 'JNI_LIB_FILE_NAME' file.
        let android_jni_lib_folder = build::ANDROID_TARGETS_CONFIG.get(&android_target).expect(
            "Cannot find 'jniLib' folder in 'android-sample' project."
        ).2;

        // build the entire jniLib based on the current android target
        let android_lib_file_path = android_jni_dir_path(&android_jni_lib_folder);

        if PathBuf::from(&crate_lib_file_path).exists() {
            io::copy_file(&crate_lib_file_path, &android_lib_file_path)?;
        } else {
            return Err("Error copying library file".into())
        }
    }

    Ok("".to_owned())
}

fn main() {
    match publish_jni_lib_to_android_project() {
        Ok(success_message) => console::print(success_message),
        Err(error) => console::print(error.to_string()),
    }
}
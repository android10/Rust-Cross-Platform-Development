// Cargo targets: https://doc.rust-lang.org/cargo/reference/cargo-targets.html#binaries

#[path="../../build.rs"] mod build;

use phf::phf_map;

use std::env;
use std::path::PathBuf;
use std::fs::File;
use std::io::Result;
use std::io::ErrorKind;

use cryptor_global::console;

/**
 * This map is necessary for copying each generated target `libcryptor_jni.so` and `libcryptor.so` to 
 * their specific directory inside the `jniLibs` in the android project.  
 * 
 * Each generated Target relates to an ABI (Application Binary Interface, which is a combination of 
 * CPU type and instruction set). According to the official android documentation, we map each
 * target with its corresponding directory (ABI): 
 *
 * -------------------------------------------------------------------------------------
 *  ANDROID TARGET                ABI (folder in the android project inside `jniLibs`)
 *  ------------------------------------------------------------------------------------
 *  armv7a-linux-androideabi ---> armeabi-v7a	  
 *  aarch64-linux-android    ---> arm64-v8a    
 *  i686-linux-android       ---> x86	        
 *  x86_64-linux-android     ---> x86-64       
 * -------------------------------------------------------------------------------------
 * 
 * For more information, check the Official Android documentation: 
 *  - https://developer.android.com/ndk/guides/other_build_systems
 *  - https://developer.android.com/ndk/guides/abis
 */
static TARGETS_ABI: phf::Map<&'static str, &'static str> = phf_map! {
    "armv7a-linux-androideabi" => "armeabi-v7a",
    "aarch64-linux-android" => "arm64-v8a",
    "i686-linux-android" => "x86",
    "x86_64-linux-android" => "x86-64"
};

static LIB_CRYPTOR_JNI_FILENAME: &str = "libcryptor_jni.so";
static LIB_CRYPTOR_FILENAME: &str = "libcryptor.so";


fn main() {
    println!("PUBLISH: {}", &build::ANDROID_TARGETS.len());
    println!("rust-library/target/armv7-linux-androideabi/release/libcryptor_jni.so");

    // https://github.com/rust-lang/cargo/issues/6100
    // For release then it is easier to change the directory

    //Check 
    //https://stackoverflow.com/questions/46749360/how-to-get-only-the-directory-portion-of-the-current-executables-path

    let target_dir = env::var("PWD").unwrap();
    let file_path = PathBuf::from("../../").into_os_string().into_string().unwrap();

    for target in build::ANDROID_TARGETS.keys() {

        console::print(format!("Android Target --> {}", &target));
        console::print(format!("Release Directory --> {}", &target_dir));
        // let command_args = build_command_args_for_target(&target.to_owned());
    }
}
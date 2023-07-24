// @See Build Scripts: 
//  - https://doc.rust-lang.org/cargo/reference/build-scripts.html
//  - https://doc.rust-lang.org/cargo/reference/build-script-examples.html
//  - https://rust-lang.github.io/rust-bindgen/tutorial-3.html

// Avoid false positive warning due to 
// calling members from another module.
#![allow(dead_code)] 

use std::env;
use std::collections::HashMap;
use std::io::Write;

use phf::phf_map;

use serde::Serialize;
use toml;

use cryptor_global::io;
use cryptor_global::console;
use cryptor_global::system;


// -----------------------------------------------------------------------------------------------
// C O N F I G U R A T I O N
// -----------------------------------------------------------------------------------------------
// Cargo Config:
//  - https://doc.rust-lang.org/cargo/reference/config.html
//
// Andrid NDK Configuration:
//  - https://developer.android.com/studio/projects/install-ndk#specific-version
// -----------------------------------------------------------------------------------------------
static ANDROID_NDK_VERSION: &str = "25.2.9519653";
static ANDROID_TOOLCHAINS_PATH: &str = "/toolchains/llvm/prebuilt/linux-x86_64/bin/";

//
// Due to Rust limitations on generating a static Map with a custom type, a tuple is 
// needed with the following value representation:
// 
//  - Tuple.0 = Archiver to be used to assemble static libraries compiled from C/C++ (Rust) code. 
//  - Tuple.1 = Linker to be used to link Rust code.
//  - Tuple.2 = ABI (Application Binary Interface) related to Tuple.0 target.
//
// ABI (Application Binary Interface) explanation:
// 
// Each generated Target relates to an ABI (Application Binary Interface, which is a combination of 
// CPU type and instruction set). According to the official android documentation, we map each
// target with its corresponding directory (ABI) in our Android client as following: 
//
// -------------------------------------------------------------------------------------
//  ANDROID TARGET                ABI (folder in the android project inside `jniLibs`)
//  ------------------------------------------------------------------------------------
//  armv7a-linux-androideabi ---> armeabi-v7a  
//  aarch64-linux-android    ---> arm64-v8a    
//  i686-linux-android       ---> x86	        
//  x86_64-linux-android     ---> x86-64       
// -------------------------------------------------------------------------------------
// 
// For more information, check the Official Android documentation: 
//  - https://developer.android.com/ndk/guides/other_build_systems
//  - https://developer.android.com/ndk/guides/abis
// 
// And the Rust Cross Compilation documentation:
// - https://rust-lang.github.io/rustup/cross-compilation.html
//
pub static ANDROID_TARGETS_CONFIG: phf::Map<&'static str, (&'static str, &'static str, &'static str)> = phf_map! {
    "armv7-linux-androideabi" => ("arm-linux-androideabi-ar", "armv7a-linux-androideabi21-clang", "armeabi-v7a"),
    "aarch64-linux-android" => ("aarch64-linux-android-ar", "aarch64-linux-android21-clang", "arm64-v8a"),
    "i686-linux-android" => ("i686-linux-android-ar", "i686-linux-android21-clang", "x86"),
    "x86_64-linux-android" => ("x86_64-linux-android-ar", "x86_64-linux-android21-clang", "x86-64"),
};
// -----------------------------------------------------------------------------------------------

struct AndroidConfig;
impl AndroidConfig {
    fn ndk_dir() -> String {
        format!(
            "{android_sdk_dir}/ndk/{android_ndk_version}",
            android_sdk_dir = env::var("ANDROID_HOME").unwrap(), 
            android_ndk_version = ANDROID_NDK_VERSION
        )
    }

    fn toolchains_dir() -> String {
        format!(
            "{ndk}{toolchains}", 
            ndk = Self::ndk_dir(), 
            toolchains = ANDROID_TOOLCHAINS_PATH
        ) 
    }
}

#[derive(Serialize)]
struct AndroidTargets<'a> {
    #[serde(rename(serialize = "target"))]
    targets: HashMap<&'a str, AndroidTargetConfig>,
}

#[derive(Serialize)]
struct AndroidTargetConfig {
    // Archiver to be used to assemble static 
    // libraries compiled from C/C++ (Rust) code.
    ar: String,

    //Linker to be used to link Rust code.
    linker: String,
}

fn build_archiver(archiver_path: &str) -> String {
    format!(
        "{toolchain}{ar}", 
        toolchain=AndroidConfig::toolchains_dir(), 
        ar=archiver_path
    )
}

fn build_linker(linker_path: &str) -> String {
    format!(
        "{toolchain}{linker}", 
        toolchain=AndroidConfig::toolchains_dir(), 
        linker=linker_path
    )
} 

fn android_targets<'a>() -> AndroidTargets<'a> {
    let mut android_targets = AndroidTargets { 
        targets: HashMap::with_capacity(ANDROID_TARGETS_CONFIG.len()) 
    };

    for (target, config) in ANDROID_TARGETS_CONFIG.entries() {
        let target_config = AndroidTargetConfig { 
            ar: build_archiver(config.0), 
            linker: build_linker(config.1) 
        };

        android_targets.targets.insert(target, target_config);
    }

    AndroidTargets { targets: android_targets.targets }
}

///
/// The file `.cargo/config` is necessary to be able to allow 
/// `cargo` to create our android targets. 
/// 
/// - https://doc.rust-lang.org/cargo/reference/config.html 
/// 
/// The config file should look like this, as 
/// an example for Linux and Android NDK 25.2.9519653: 
/// 
/// ```
/// [target.armv7-linux-androideabi]
/// ar = "$ANDROID_HOME/ndk/25.2.9519653/toolchains/llvm/prebuilt/linux-x86_64/bin/arm-linux-androideabi-ar"
/// linker = "$ANDROID_HOME/ndk/25.2.9519653/toolchains/llvm/prebuilt/linux-x86_64/bin/armv7a-linux-androideabi21-clang"

/// [target.aarch64-linux-android]
/// ar = "$ANDROID_HOME/ndk/25.2.9519653/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android-ar"
/// linker = "ANDROID_HOME/ndk/25.2.9519653/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android21-clang"

/// [target.i686-linux-android]
/// ar = "$ANDROID_HOME/ndk/25.2.9519653/toolchains/llvm/prebuilt/linux-x86_64/bin/i686-linux-android-ar"
/// linker = "$ANDROID_HOME/ndk/25.2.9519653/toolchains/llvm/prebuilt/linux-x86_64/bin/i686-linux-android21-clang"

/// [target.x86_64-linux-android]
/// ar = "$ANDROID_HOME/ndk/25.2.9519653/toolchains/llvm/prebuilt/linux-x86_64/bin/x86_64-linux-android-ar"
/// linker = "$ANDROID_HOME/ndk/25.2.9519653/toolchains/llvm/prebuilt/linux-x86_64/bin/x86_64-linux-android21-clang"
/// ```
fn create_android_targets_config_file() {
    let targets_config = android_targets();
    let mut config_file = io::create_cargo_config_file(&env::current_dir().unwrap());
    let toml = toml::to_string(&targets_config).unwrap();
    
    match config_file.write_all(toml.as_bytes()) {
        Ok(_) => println!("Successfully wrote Android Configuration File."),
        Err(why) => panic!("Couldn't write Android Configuration: {}", why),
    };
}

///
/// Install android targets each for each toolchaing using 
/// `rustup`, like so:
/// 
/// `rustup target add --toolchain <toolchain> <target>...`
/// 
/// Cross-compilation: 
///  - https://rust-lang.github.io/rustup/cross-compilation.html
/// 
fn add_android_targets_to_toolchain() {
    let mut command_args = Vec::new();
    command_args.push("target");
    command_args.push("add");
    
    for target in ANDROID_TARGETS_CONFIG.keys() {
        command_args.push(target)
    }

    console::run_command("rustup", &command_args.as_slice());
}

fn main() {
    system::rerun_if_changed("build.rs");

    create_android_targets_config_file();
    add_android_targets_to_toolchain();
}

//
// T E S T S 
//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn android_targets_and_config_same_size() {
        let android_targets = android_targets().targets; 
        assert_eq!(&android_targets.len() , &ANDROID_TARGETS_CONFIG.len());
    }

    #[test]
    fn android_targets_links_to_proper_target_config() {
        for target_config in ANDROID_TARGETS_CONFIG.entries() {
            let target_config_key = target_config.0;
            let target_config_abi = target_config.1.2;

            if target_config_key.starts_with("armv7") { assert_eq!(target_config_abi, "armeabi-v7a") }
            if target_config_key.starts_with("aarch64") { assert_eq!(target_config_abi, "arm64-v8a") }
            if target_config_key.starts_with("i686") { assert_eq!(target_config_abi, "x86") }
            if target_config_key.starts_with("x86_64") { assert_eq!(target_config_abi, "x86-64") }
        }
    }
}
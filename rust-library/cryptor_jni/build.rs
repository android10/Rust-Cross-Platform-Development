// @See Build Scripts: 
//  - https://doc.rust-lang.org/cargo/reference/build-scripts.html
//  - https://doc.rust-lang.org/cargo/reference/build-script-examples.html

#![allow(dead_code)] // Avoid false positive warning due to calling members from another module

use std::env;
use std::{collections::HashMap};
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
// @See Cargo Config
//  - https://doc.rust-lang.org/cargo/reference/config.html
// -----------------------------------------------------------------------------------------------
static ANDROID_NDK_ENV_VAR: &str = "ANDROID_NDK_HOME";
static ANDROID_TOOLCHAINS_PATH: &str = "/toolchains/llvm/prebuilt/linux-x86_64/bin/";

/**
 * Due to Rust limitations on generating a static Map with a custom type, a tuple was 
 * needed with the following value representation:
 *  - Tuple.0 = Archiver to be used to assemble static libraries compiled from C/C++ code. 
 *  - Tuple.1 = Linker to be used to link Rust code.
 */
pub static ANDROID_TARGETS: phf::Map<&'static str, (&'static str, &'static str)> = phf_map! {
    "armv7-linux-androideabi" => ("arm-linux-androideabi-ar", "armv7a-linux-androideabi21-clang"),
    "aarch64-linux-android" => ("aarch64-linux-android-ar", "aarch64-linux-android21-clang"),
    "i686-linux-android" => ("i686-linux-android-ar", "i686-linux-android21-clang"),
    "x86_64-linux-android" => ("x86_64-linux-android-ar", "x86_64-linux-android21-clang"),
};
// -----------------------------------------------------------------------------------------------

struct AndroidConfig;
impl AndroidConfig {
    fn ndk_dir() -> String {
        env::var(ANDROID_NDK_ENV_VAR).unwrap()
    }

    fn toolchains_dir() -> String {
        format!("{ndk}{toolchains}", ndk=Self::ndk_dir(), toolchains=ANDROID_TOOLCHAINS_PATH) 
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
    // libraries compiled from C/C++ code.
    ar: String,

    //Linker to be used to link Rust code.
    linker: String,
}

fn build_archiver(archiver_path: &str) -> String {
    format!("{toolchain}{ar}", toolchain=AndroidConfig::toolchains_dir(), ar=archiver_path)
}

fn build_linker(linker_path: &str) -> String {
    format!("{toolchain}{linker}", toolchain=AndroidConfig::toolchains_dir(), linker=linker_path)
} 

fn android_targets<'a>() -> AndroidTargets<'a> {
    let mut android_targets = AndroidTargets { targets: HashMap::with_capacity(ANDROID_TARGETS.len()) };

    for (target, config) in ANDROID_TARGETS.entries() {
        let target_config = AndroidTargetConfig { ar: build_archiver(config.0), linker: build_linker(config.1) };
        android_targets.targets.insert(target, target_config);
    }

    AndroidTargets { targets: android_targets.targets }
}

fn create_android_targets_config_file() {
    let targets_config = android_targets();
    let mut config_file = io::create_cargo_config_file(&env::current_dir().unwrap());
    let toml = toml::to_string(&targets_config).unwrap();
    
    match config_file.write_all(toml.as_bytes()) {
        Err(why) => panic!("Couldn't write Android Configuration: {}", why),
        Ok(_) => println!("Successfully wrote Android Configuration File."),
    };
}

fn add_android_targets_to_toolchain() {
    let mut command_args = Vec::new();
    command_args.push("target");
    command_args.push("add");
    
    for target in ANDROID_TARGETS.keys() {
        command_args.push(target)
    }

    console::run_command("rustup", &command_args.as_slice());
}

fn main() {
    system::rerun_if_changed("build.rs");

    create_android_targets_config_file();
    add_android_targets_to_toolchain();
}
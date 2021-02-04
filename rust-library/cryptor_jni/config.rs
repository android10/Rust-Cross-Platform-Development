// @See Cargo Config
//  - https://doc.rust-lang.org/cargo/reference/config.html

use std::env;
use std::{collections::HashMap};
use std::io::Write;

use phf::phf_map;
use serde::Serialize;
use toml;

use cryptor_global::io;
use cryptor_global::console;

static ANDROID_NDK_ENV_VAR: &str = "ANDROID_NDK_HOME";
static ANDROID_TOOLCHAINS_DIR: &str = "/toolchains/llvm/prebuilt";

static ANDROID_TARGETS: phf::Map<&'static str, &'static str> = phf_map! {
    "ARMV7" => "armv7-linux-androideabi",
    "AARCH64" => "aarch64-linux-android",
    "I686" => "i686-linux-android",
    "X86_64" => "x86_64-linux-android",
};

struct AndroidConfig;
impl AndroidConfig {
    fn ndk_dir() -> String {
        env::var(ANDROID_NDK_ENV_VAR).unwrap()
    }

    fn toolchains_dir() -> String {
        format!("{ndk}{toolchains}", ndk=Self::ndk_dir(), toolchains=ANDROID_TOOLCHAINS_DIR) 
    }
}

#[derive(Serialize)]
struct AndroidTargets<'a> {
    #[serde(rename(serialize = "target"))]
    targets: HashMap<&'a str, AndroidTargetConfig<'a>>,
}

#[derive(Serialize)]
struct AndroidTargetConfig<'a> {
    // Target identifier name to be used by Rust compiler 
    // to generate the specific artifact.
    #[serde(skip_serializing)]
    name: &'a str,

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
    let mut android_targets = AndroidTargets { targets: HashMap::with_capacity(4) };

    let armv7 = AndroidTargetConfig { 
        name: ANDROID_TARGETS["ARMV7"],
        ar: build_archiver("/linux-x86_64/bin/arm-linux-androideabi-ar"),
        linker: build_linker("/linux-x86_64/bin/armv7a-linux-androideabi21-clang"), 
    };

    let aarch64 = AndroidTargetConfig { 
        name: ANDROID_TARGETS["AARCH64"],
        ar: build_archiver("/linux-x86_64/bin/aarch64-linux-android-ar"),
        linker: build_linker("/linux-x86_64/bin/aarch64-linux-android21-clang"), 
    };

    let i686 = AndroidTargetConfig { 
        name: ANDROID_TARGETS["I686"],
        ar: build_archiver("/linux-x86_64/bin/i686-linux-android-ar"),
        linker: build_linker("/linux-x86_64/bin/i686-linux-android21-clang"), 
    };

    let x86_64 = AndroidTargetConfig { 
        name: ANDROID_TARGETS["X86_64"],
        ar: build_archiver("/linux-x86_64/bin/x86_64-linux-android-ar"),
        linker: build_linker("/linux-x86_64/bin/x86_64-linux-android21-clang"), 
    };

    android_targets.targets.insert(armv7.name, armv7);
    android_targets.targets.insert(aarch64.name, aarch64);
    android_targets.targets.insert(i686.name, i686);
    android_targets.targets.insert(x86_64.name, x86_64);

    //Being defensive to make sure we do not forget to add any target
    assert!(android_targets.targets.len() == ANDROID_TARGETS.len());

    AndroidTargets { targets: android_targets.targets }
}

pub fn create_android_targets_config_file() {

    let targets_config = android_targets();

    let mut config_file = io::create_cargo_config_file(&env::current_dir().unwrap());
    
    let toml = toml::to_string(&targets_config).unwrap();
    
    match config_file.write_all(toml.as_bytes()) {
        Err(why) => panic!("Couldn't write Android Configuration: {}", why),
        Ok(_) => println!("Successfully wrote Android Configuration File."),
    };
}

pub fn add_android_targets_to_toolchain() {
    //TODO
    let command_args = ["target", "add", "armv7-linux-androideabi", "aarch64-linux-android", "i686-linux-android", "x86_64-linux-android"];
    console::run_command("rustup", &command_args);
}

#[cfg(test)]
mod tests {
    use super::config;

    #[test]
    fn test_android_number_of_targets() {
        assert_eq!(ANDROID_TARGETS.len(), 4);
    }
}







// @See Build Scripts: 
//  - https://doc.rust-lang.org/cargo/reference/build-scripts.html
//  - https://doc.rust-lang.org/cargo/reference/build-script-examples.html

use std::env;
use std::{collections::HashMap};
use std::io::Write;

use serde::Serialize;
use toml;

use cryptor_global::io;
use cryptor_global::console;
use cryptor_global::system;

mod config;

struct AndroidConfig;
impl AndroidConfig {
    fn ndk_dir() -> String {
        env::var(config::ANDROID_NDK_ENV_VAR).unwrap()
    }

    fn toolchains_dir() -> String {
        format!("{ndk}{toolchains}", ndk=Self::ndk_dir(), toolchains=config::ANDROID_TOOLCHAINS_PATH) 
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
    let mut android_targets = AndroidTargets { targets: HashMap::with_capacity(config::ANDROID_TARGETS.len()) };

    for (target, config) in config::ANDROID_TARGETS.entries() {
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
    
    for target in config::ANDROID_TARGETS.keys() {
        command_args.push(target)
    }

    console::run_command("rustup", &command_args.as_slice());
}

fn main() {
    // system::rerun_if_changed("build.rs");

    create_android_targets_config_file();
    add_android_targets_to_toolchain();
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_test() {
        // TODO: It does not work
        assert_eq!(true, false);
    }
}

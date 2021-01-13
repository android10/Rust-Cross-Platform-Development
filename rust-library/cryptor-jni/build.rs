// @See Build Scripts: 
//  - https://doc.rust-lang.org/cargo/reference/build-scripts.html
//  - https://doc.rust-lang.org/cargo/reference/build-script-examples.html

// @See Cargo Config
//  - https://doc.rust-lang.org/cargo/reference/config.html


use std::env;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::Write;
use serde::Serialize;
use std::{collections::BTreeMap};
use toml;

mod config;
mod release;

#[derive(Default, Serialize)]
struct Targets<'a> {
    #[serde(rename(serialize = "target"))]
    targets: BTreeMap<&'a str, TargetConfig<'a>>,
}

#[derive(Serialize)]
struct TargetConfig<'a> {
    // Archiver to be used to assemble static 
    // libraries compiled from C/C++ code.
    ar: &'a str,

    //Linker to be used to link Rust code.
    linker: &'a str,
}

fn main() {
    // Tell Cargo that if the given file changes, 
    // to rerun this build script. 
    //
    // We communicate with cargo from within 
    // the script by writing to stdout.
    // println!("cargo:rerun-if-changed=build.rs");

    let android_ndk_dir = env::var("ANDROID_NDK_HOME").unwrap();
    let android_toolchains_dir = format!("{ndk}{toolchains}", ndk=android_ndk_dir, toolchains="/toolchains/llvm/prebuilt"); 

    let mut targets_config = Targets::default();

    let armv7_name = "armv7-linux-androideabi";
    let armv7_ar = format!("{toolchain}{ar}", toolchain=android_toolchains_dir, ar="/linux-x86_64/bin/arm-linux-androideabi-ar"); 
    let armv7_linker = format!("{toolchain}{linker}", toolchain=android_toolchains_dir, linker="/linux-x86_64/bin/armv7a-linux-androideabi21-clang");
    targets_config.targets.insert(
        &armv7_name,
        TargetConfig { 
            ar: &armv7_ar, 
            linker: &armv7_linker 
        },
    );

    let aarch64_name = "aarch64-linux-android";
    let aarch64_ar = format!("{toolchain}{ar}", toolchain=android_toolchains_dir, ar="/linux-x86_64/bin/aarch64-linux-android-ar");
    let aarch64_linker = format!("{toolchain}{linker}", toolchain=android_toolchains_dir, linker="/linux-x86_64/bin/aarch64-linux-android21-clang");
    targets_config.targets.insert(
        &aarch64_name,
        TargetConfig {
            ar: &aarch64_ar,
            linker: &aarch64_linker,
        },
    );

    let i686_name = "i686-linux-android";
    let i686_ar = format!("{toolchain}{ar}", toolchain=android_toolchains_dir, ar="/linux-x86_64/bin/i686-linux-android-ar");
    let i686_linker = format!("{toolchain}{linker}", toolchain=android_toolchains_dir, linker="/linux-x86_64/bin/i686-linux-android21-clang");
    targets_config.targets.insert(
        &i686_name,
        TargetConfig {
            ar: &i686_ar,
            linker: &i686_linker,
        },
    );

    let x86_64_name = "x86_64-linux-android";
    let x86_64_ar = format!("{toolchain}{ar}", toolchain=android_toolchains_dir, ar="/linux-x86_64/bin/x86_64-linux-android-ar");
    let x86_64_linker = format!("{toolchain}{linker}", toolchain=android_toolchains_dir, linker="/linux-x86_64/bin/x86_64-linux-android21-clang");
    targets_config.targets.insert(
        &x86_64_name,
        TargetConfig {
            ar: &x86_64_ar,
            linker: &x86_64_linker,
        },
    );

    let toml = toml::to_string(&targets_config).unwrap();

    match create_cargo_config_dir() {
        Err(_) => println!("Directory already exists, avoiding step..."),
        Ok(_) => println!("Successfully created configuration dir!"),
    };

    let config_file_path = format!("{dir}/{file}", dir=".cargo", file="config");
    let path = Path::new(&config_file_path);
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // Write content to `file`, returns `io::Result<()>`
    match file.write_all(toml.as_bytes()) {
        Err(why) => panic!("Couldn't write to {}: {}", display, why),
        Ok(_) => println!("Successfully wrote to {}", display),
    };

    //TODO: uncommnet println!("cargo:rerun-if-changed=build.rs")
    //TODO: Refactor and extract global stuff: Global Configuration in Rust?
    //TODO: Write tests
    //TODO: Add toolchains to cargo via command: rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android
    //TODO: post release execution: deploy task to copy all the mentioned architectures to the folders. 

    release::deploy_artifacts();
}

// TODO: This could also be global
fn create_cargo_config_dir() -> std::io::Result<()> {
    fs::create_dir(".cargo")?;
    Ok(())
}

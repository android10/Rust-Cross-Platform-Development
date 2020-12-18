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
use std::io::prelude::*;


use serde::Serialize;
use std::{collections::BTreeMap};
use toml;


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
    let android_toolchains_dir = format!("{ndk}/{toolchains}", ndk=android_ndk_dir, toolchains="/toolchains/llvm/prebuilt"); 

    let mut targets_config = Targets::default();
    targets_config.targets.insert(
        "armv7-linux-androideabi",
        TargetConfig {
            ar: format!("{toolchain}{ar}", toolchain=android_toolchains_dir, ar="/linux-x86_64/bin/arm-linux-androideabi-ar"),
            linker: format!("{toolchain}{linker}", toolchain=android_toolchains_dir, linker="/linux-x86_64/bin/armv7a-linux-androideabi21-clang"),
        },
    );
    targets_config.targets.insert(
        "aarch64-linux-android",
        TargetConfig {
            ar: format!("{toolchain}{ar}", toolchain=android_toolchains_dir, ar="/linux-x86_64/bin/aarch64-linux-android-ar"),
            linker: format!("{toolchain}{linker}", toolchain=android_toolchains_dir, linker="/linux-x86_64/bin/aarch64-linux-android21-clang"),
        },
    );
    targets_config.targets.insert(
        "i686-linux-android",
        TargetConfig {
            ar: format!("{toolchain}{ar}", toolchain=android_toolchains_dir, ar="/linux-x86_64/bin/i686-linux-android-ar"),
            linker: format!("{toolchain}{linker}", toolchain=android_toolchains_dir, linker="/linux-x86_64/bin/i686-linux-android21-clang"),
        },
    );
    targets_config.targets.insert(
        "x86_64-linux-android",
        TargetConfig {
            ar: format!("{toolchain}{ar}", toolchain=android_toolchains_dir, ar="/linux-x86_64/bin/x86_64-linux-android-ar"),
            linker: format!("{toolchain}{linker}", toolchain=android_toolchains_dir, linker="/linux-x86_64/bin/x86_64-linux-android21-clang"),
        },
    );
    let toml = toml::to_string(&targets_config).unwrap();



    create_cargo_config_dir();
    // match create_cargo_config_dir() {
    //     Err(why) => panic!("Couldn't create {}: {}", display, why),
    //     Ok(())
    // }

    let config_file_path = format!("{dir}/{file}", dir=".cargo", file="config1");
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

    //TODO: Refactor and extract global stuff
    //TODO: Write tests
    //TODO: Add toolchains to cargo via command
}

fn create_cargo_config_dir() -> std::io::Result<()> {
    fs::create_dir(".cargo")?;
    Ok(())
}

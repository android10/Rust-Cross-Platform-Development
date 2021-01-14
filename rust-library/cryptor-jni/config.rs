// @See Cargo Config
//  - https://doc.rust-lang.org/cargo/reference/config.html

use std::env;
use std::{collections::HashMap};
use std::io::Result;

use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::Write;

use serde::Serialize;
use toml;

static ANDROID_NDK_ENV_VAR: &str = "ANDROID_NDK_HOME";
static ANDROID_TOOLCHAINS_DIR: &str = "/toolchains/llvm/prebuilt";

struct AndroidConfig;
impl AndroidConfig {
    fn ndk_dir() -> String {
        env::var(ANDROID_NDK_ENV_VAR).unwrap()
    }

    fn toolchains_dir() -> String {
        format!("{ndk}{toolchains}", ndk=Self::ndk_dir(), toolchains=ANDROID_TOOLCHAINS_DIR) 
    }
}

#[derive(Default, Serialize)]
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
    // let mut android_targets = AndroidTargets::default();
    let mut android_targets = AndroidTargets { targets: HashMap::with_capacity(4) };

    let armv7 = AndroidTargetConfig { 
        name: "armv7-linux-androideabi",
        ar: build_archiver("/linux-x86_64/bin/arm-linux-androideabi-ar"),
        linker: build_linker("/linux-x86_64/bin/armv7a-linux-androideabi21-clang"), 
    };

    let aarch64 = AndroidTargetConfig { 
        name: "aarch64-linux-android",
        ar: build_archiver("/linux-x86_64/bin/aarch64-linux-android-ar"),
        linker: build_linker("/linux-x86_64/bin/aarch64-linux-android21-clang"), 
    };

    let i686 = AndroidTargetConfig { 
        name: "i686-linux-android",
        ar: build_archiver("/linux-x86_64/bin/i686-linux-android-ar"),
        linker: build_linker("/linux-x86_64/bin/i686-linux-android21-clang"), 
    };

    let x86_64 = AndroidTargetConfig { 
        name: "x86_64-linux-android",
        ar: build_archiver("/linux-x86_64/bin/x86_64-linux-android-ar"),
        linker: build_linker("/linux-x86_64/bin/x86_64-linux-android21-clang"), 
    };

    android_targets.targets.insert(armv7.name, armv7);
    android_targets.targets.insert(aarch64.name, aarch64);
    android_targets.targets.insert(i686.name, i686);
    android_targets.targets.insert(x86_64.name, x86_64);

    AndroidTargets { targets: android_targets.targets }
}

pub fn create_android_targets_config_file() -> Result<()> {

    let targets_config = android_targets();

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

    Ok(())
}

// TODO: This could also be global
fn create_cargo_config_dir() -> std::io::Result<()> {
    fs::create_dir(".cargo")?;
    Ok(())
}







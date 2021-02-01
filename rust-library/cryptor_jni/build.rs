// @See Build Scripts: 
//  - https://doc.rust-lang.org/cargo/reference/build-scripts.html
//  - https://doc.rust-lang.org/cargo/reference/build-script-examples.html

mod config;
mod release;

fn main() {
    // Tell Cargo that if the given file changes, 
    // to rerun this build script. 
    //
    // We communicate with cargo from within 
    // the script by writing to stdout.
    // println!("cargo:rerun-if-changed=build.rs");

    config::create_android_targets_config_file();
    config::add_android_targets_to_toolchain();

    //TODO: uncommnet println!("cargo:rerun-if-changed=build.rs")
    //TODO: Write tests
    //TODO: Add toolchains to cargo via command: rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android
    //TODO: post release execution: deploy task to copy all the mentioned architectures to the folders. 

    release::deploy_android_artifacts();
}

// @See Build Scripts: 
//  - https://doc.rust-lang.org/cargo/reference/build-scripts.html
//  - https://doc.rust-lang.org/cargo/reference/build-script-examples.html

use cryptor_global::system;

mod config;

fn main() {
    system::rerun_if_changed("build.rs");
    
    config::create_android_targets_config_file();
    config::add_android_targets_to_toolchain();
}

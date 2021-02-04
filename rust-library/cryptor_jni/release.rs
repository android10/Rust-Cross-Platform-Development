use cryptor_global::system;
use cryptor_global::console;

pub fn build_android_artifacts() {
    if system::is_release() {
        //TODO
        // console::run_command(comand: &str, args: &[&str])
    }
}

pub fn deploy_android_artifacts() {
    if system::is_release() {
        //TODO: copy the artifacts to its corresponding directories
    }
}
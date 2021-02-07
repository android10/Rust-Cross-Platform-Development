use cryptor_global::system;
use cryptor_global::console;

pub fn build_android_artifacts() {
    if system::is_release() {
        //TODO: Please refactor this
        let command_base = "cargo";
        let command_args = ["build", "--target", "armv7-linux-androideabi", "--release"];
        console::run_command(&command_base, &command_args);
        let command_args = ["build", "--target", "aarch64-linux-android", "--release"];
        console::run_command(&command_base, &command_args);
        let command_args = ["build", "--target", "i686-linux-android", "--release"];
        console::run_command(&command_base, &command_args);
        let command_args = ["build", "--target", "armv7-linux-androideabi", "--release"];
        console::run_command(&command_base, &command_args);
    }
}

pub fn deploy_android_artifacts() {
    if system::is_release() {
        //TODO: copy the artifacts to its corresponding directories
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //TODO: Write tests
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
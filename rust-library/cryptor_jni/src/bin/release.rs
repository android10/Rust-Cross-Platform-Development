// Cargo targets: https://doc.rust-lang.org/cargo/reference/cargo-targets.html#binaries

#[path="../../build.rs"] mod build;

use cryptor_global::console;

fn main() {
    console::out("Releasing Android Targets...");
    release_android_targets();
}

/// Release android targets from Configuration.
/// Check out ['build.rs'] File.
/// 
/// Example of executed command:
/// ```
/// cargo build --target armv7-linux-androideabi --release
/// ```
fn release_android_targets() {
    for target in build::ANDROID_TARGETS.keys() {
        let command_args = build_command_args_for_target(&target);
        console::run_command("cargo", &command_args.as_slice());
    }
}

fn build_command_args_for_target(target: &str) -> Vec<&str> {
    let mut command_args = Vec::new();
    command_args.push("build");
    command_args.push("--target");
    command_args.push(target);
    command_args.push("--release");

    command_args
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_android_release_arguments() {
        let command_args = build_command_args_for_target("armv7-linux-androideabi");
        let expected_result = "build --target armv7-linux-androideabi --release";
        
        assert_eq!(command_args.join(" "), expected_result.trim());
    }
}
/// This is a binary targets, which is an executable program 
/// that can be run after crate compilation.
/// 
/// It will basically build release versions of 
/// each android target. 
/// 
/// ## Examples
/// ```
/// $ cd cryptor_jni/
/// $ cargo run --bin release 
/// ``` 
/// 
/// For more information, refer to the official doc:
///  - https://doc.rust-lang.org/cargo/reference/cargo-targets.html#binaries


// https://doc.rust-lang.org/reference/items/modules.html
#[path="../../build.rs"] 
mod build;

use cryptor_global::console;

///
/// Release android targets from Configuration.
/// Check ['build.rs'] File.
/// 
/// ## Example
/// 
/// ```
/// cargo build --target armv7-linux-androideabi --release
/// cargo build --target aarch64-linux-android --release
/// cargo build --target i686-linux-android --release
/// cargo build --target x86_64-linux-android --release
/// ```
fn release_android_targets() {
    for target in build::ANDROID_TARGET_ABI_CONFIG.keys() {
        console::print(format!("Building Android Target --> {}", &target));

        let command_args = build_command_args_for_target(&target);
        console::run_command("cargo", &command_args);
    }
}

/// 
/// Build `cargo` arguments for building android targets.
/// 
/// ## Example
/// 
/// ```
/// cargo build --target armv7-linux-androideabi --release
/// ```
fn build_command_args_for_target(target: &str) -> Vec<&str> {
    let mut command_args = Vec::new();

    command_args.push("build");
    command_args.push("--target");
    command_args.push(target);
    command_args.push("--release");

    command_args
}

fn main() {
    console::out("Releasing Android Targets...Be patient... :)");

    release_android_targets();
}

//
// T E S T S 
//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_android_release_argument() {
        let command_args = build_command_args_for_target("armv7-linux-androideabi");
        let expected_result = "build --target armv7-linux-androideabi --release";
        
        assert_eq!(command_args.join(" "), expected_result.trim());
    }

    #[test]
    fn test_build_android_release_arguments_for_all_targets() {
        for target in build::ANDROID_TARGET_ABI_CONFIG.keys() {
            let command_args = build_command_args_for_target(&target);
            let expected_result = format!("build --target {target} --release", target = &target);

            assert_eq!(command_args.join(" "), expected_result.trim());
        }
    }
}
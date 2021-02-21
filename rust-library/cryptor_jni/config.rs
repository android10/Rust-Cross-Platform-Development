// @See Cargo Config
//  - https://doc.rust-lang.org/cargo/reference/config.html

use phf::phf_map;

pub static ANDROID_NDK_ENV_VAR: &str = "ANDROID_NDK_HOME";
pub static ANDROID_TOOLCHAINS_PATH: &str = "/toolchains/llvm/prebuilt/linux-x86_64/bin/";

/**
 * Due to Rust limitations on generating a static Map with a custom type, a tuple was 
 * needed which the following value representation:
 *  - Tuple.0 = Archiver to be used to assemble static libraries compiled from C/C++ code. 
 *  - Tuple.1 = Linker to be used to link Rust code. 
 */
pub static ANDROID_TARGETS: phf::Map<&'static str, (&'static str, &'static str)> = phf_map! {
    "armv7-linux-androideabi" => ("arm-linux-androideabi-ar", "armv7a-linux-androideabi21-clang"),
    "aarch64-linux-android" => ("aarch64-linux-android-ar", "aarch64-linux-android21-clang"),
    "i686-linux-android" => ("i686-linux-android-ar", "i686-linux-android21-clang"),
    "x86_64-linux-android" => ("x86_64-linux-android-ar", "x86_64-linux-android21-clang"),
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_android_number_of_targets() {
        // TODO: It does not work
        assert_eq!(super::ANDROID_TARGETS.len(), 8);
    }
}
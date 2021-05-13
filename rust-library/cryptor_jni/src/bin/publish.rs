// Cargo targets: https://doc.rust-lang.org/cargo/reference/cargo-targets.html#binaries

#[path="../../build.rs"] mod build;

use phf::phf_map;

/**
 * This map is necessary for copying each generated target `libcryptor_jni.so` and `libcryptor.so` to 
 * their specific directory inside the `jniLibs` in the android project.  
 * 
 * Each generated Target relates to an ABI (Application Binary Interface, which is a combination of 
 * CPU type and instruction set). According to the official android documentation, we map each
 * target with its corresponding directory (ABI): 
 *
 * -------------------------------------------
 *  ABI               TARGET
 *  ------------------------------------------
 *  armeabi-v7a	 ---> armv7a-linux-androideabi
 *  arm64-v8a    ---> aarch64-linux-android
 *  x86	         ---> i686-linux-android
 *  x86-64       ---> x86_64-linux-android
 * -------------------------------------------
 * 
 * For more information, check the Official Android documentation: 
 *  - https://developer.android.com/ndk/guides/other_build_systems
 *  - https://developer.android.com/ndk/guides/abis
 */
static ABI_TARGETS: phf::Map<&'static str, &'static str> = phf_map! {
    "armeabi-v7a" => "armv7a-linux-androideabi",
    "arm64-v8a" => "aarch64-linux-android",
    "x86" => "i686-linux-android",
    "x86-64" => "x86_64-linux-android",
};

static LIB_CRYPTOR_JNI_FILENAME: &str = "libcryptor_jni.so";
static LIB_CRYPTOR_FILENAME: &str = "libcryptor.so";


fn main() {
    println!("PUBLISH: {}", &build::ANDROID_TARGETS.len());
    println!("rust-library/target/armv7-linux-androideabi/release/libcryptor_jni.so");
}
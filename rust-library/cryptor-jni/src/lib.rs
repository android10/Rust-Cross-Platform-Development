/// Expose the JNI interface for Android
#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;

    use self::jni::objects::{JClass, JString};
    use self::jni::sys::jstring;
    use self::jni::JNIEnv;
    use super::*;

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_fernandocejas_cryptor_encrypt(
        env: JNIEnv,
        _: JClass,
        java_pattern: JString,
    ) -> jstring {
        let string = encrypt(
            env.get_string(java_pattern)
                .expect("invalid pattern string")
                .as_ptr(),
        );
        // Retake pointer so that we can use it below and allow memory to be freed when it goes out of scope.
        let string_ptr = CString::from_raw(string);
        let output = env
            .new_string(string_ptr.to_str().unwrap())
            .expect("Couldn't create java string!");

        output.into_inner()
    }
}

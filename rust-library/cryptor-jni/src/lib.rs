/// [cfg(target_os = "android")]: Compiler flag ("cfg") which exposes
/// the JNI interface for targeting Android in this case
/// 
/// [allow(non_snake_case)]: Tells the compiler not to warn if
/// we are not using snake_case for a variable or function names.
/// For Android Development we want to be consistent with code style. 
#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;

    use self::jni::objects::{JClass, JString};
    use self::jni::sys::jstring;
    use self::jni::JNIEnv;
    use super::*;

    /**
     * Encrypt a String.
     */
    #[no_mangle]
    pub unsafe extern "C" fn Java_com_fernandocejas_cryptor_encrypt(
        env: JNIEnv,
        _: JClass,
        java_string: JString,
    ) -> jstring {

        // Call the Rust Library for encryption
        let encrypted_str = encrypt(get_string(env, java_string));

        // Retake pointer so that we can use it below and allow 
        // memory to be freed when it goes out of scope.
        let encrypted_str_ptr = CString::from_raw(encrypted_str);
        let output = get_string(encrypted_str_ptr.to_str().unwrap());

        output.into_inner()
    }

    /**
     * Decrypt a String.
     */
    #[no_mangle]
    pub unsafe extern "C" fn Java_com_fernandocejas_cryptor_decrypt(
        env: JNIEnv,
        _: JClass,
        java_string: JString,
    ) -> jstring {

        // Call the Rust Library for decryption
        let decrypted_str = decrypt(get_string(env, java_string));

        // Retake pointer so that we can use it below and allow 
        // memory to be freed when it goes out of scope.
        let decrypted_str_ptr = CString::from_raw(decrypted_str);
        let output = get_string(decrypted_str_ptr.to_str().unwrap());

        output.into_inner()
    }    

    /**
     * Get and check a valid Java String
     */
    fn get_string(
        env: JNIEnv, 
        java_string: JString) {

        env.get_string(java_string)
        .expect("invalid Pattern String")
        .as_ptr()
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_cryptor_encrypt() {
        // TODO
        assert_eq!(true, true);
    }

    #[test]
    fn test_cryptor_decrypt() {
        // TODO
        assert_eq!(true, true);
    }
}

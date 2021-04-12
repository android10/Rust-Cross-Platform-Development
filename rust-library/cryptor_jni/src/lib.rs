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

    use self::jni::JNIEnv;
    use self::jni::objects::{JClass, JString};
    use self::jni::sys::{jstring};
    use std::ffi::{CStr};
    use std::os::raw::c_char;
    
    use cryptor::encrypt;
    use cryptor::decrypt;

    /// Encrypts a String.
    #[no_mangle]
    pub unsafe extern fn Java_com_fernandocejas_rust_Cryptor_encrypt(
        env: JNIEnv,
        _: JClass,
        java_string: JString,
    ) -> jstring {

        // Let's call the Rust Library for encryption
        let to = get_string(&env, java_string);
        let to_encrypt = CStr::from_ptr(to).to_str().unwrap();

        let encrypted_str = encrypt(to_encrypt);
        let output = env.new_string(&encrypted_str).expect("Couldn't create Java String!");

        output.into_inner()
    }
    
    /// Decrypts a String.
    #[no_mangle]
    pub unsafe extern fn Java_com_fernandocejas_rust_Cryptor_decrypt(
        env: JNIEnv, 
        _: JClass, 
        java_string: JString
    ) -> jstring {

        // Let's call the Rust Library for decryption
        let to = get_string(&env, java_string);
        let to_decrypt = CStr::from_ptr(to).to_str().unwrap();

        let decrypted_str = decrypt(to_decrypt);
        let output = env.new_string(&decrypted_str).expect("Couldn't create Java String!");

        output.into_inner()
    }

    /// Get and check a valid Java String
    fn get_string(
        env: &JNIEnv, 
        java_string: JString
    ) -> *const c_char {

        env.get_string(java_string)
        .expect("Invalid Pattern String")
        .as_ptr()
    }
}

#[cfg(test)]
mod tests {

    // use super::*;

    #[test]
    fn test_cryptor_encrypt() {
        
        let _str_encoded_b64 = "aGVsbG9fd29ybGRfZnJvbV9qbmk=";
        let _to_encrypt = "hello_world_from_jni";
        
        // TODO
        assert_eq!(true, true);
    }

    #[test]
    fn test_cryptor_decrypt() {

        let _str_decoded_b64 = "hello_world_from_jni";
        let _to_decrypt_b64 = "aGVsbG9fd29ybGRfZnJvbV9qbmk=";

        // TODO
        assert_eq!(true, true);
    }
}

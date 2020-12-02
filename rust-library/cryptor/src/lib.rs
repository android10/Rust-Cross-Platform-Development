use std::os::raw::{c_char};
use std::ffi::{CString, CStr};

#[no_mangle]
pub extern fn encrypt(to: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(to) };
    let recipient = match c_str.to_str() {
        Err(_) => "There was an error",
        Ok(string) => string,
    };

    CString::new("sdflsjdfsv094".to_owned()).unwrap().into_raw()
}

/// Expose the JNI interface for Android
#[cfg(target_os="android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;

    use super::*;
    use self::jni::JNIEnv;
    use self::jni::objects::{JClass, JString};
    use self::jni::sys::{jstring};

    #[no_mangle]
    pub unsafe extern fn Java_com_fernandocejas_cryptor_encrypt(env: JNIEnv, _: JClass, java_pattern: JString) -> jstring {
        let string = encrypt(env.get_string(java_pattern).expect("invalid pattern string").as_ptr());
        // Retake pointer so that we can use it below and allow memory to be freed when it goes out of scope.
        let string_ptr = CString::from_raw(string);
        let output = env.new_string(string_ptr.to_str().unwrap()).expect("Couldn't create java string!");

        output.into_inner()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::encrypt;

//     #[test]
//     fn test_encrypt_string() {
//         assert_eq!("sdflsjdfsv094", encrypt("hello".as_ptr() as *const i8));
//     }
// }
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// Encrypts a String.
#[no_mangle] 
pub extern "C" fn encrypt(to: *const c_char) -> *mut c_char {
    let to = unsafe { CStr::from_ptr(to).to_str().unwrap() };
    let result = cryptor::encrypt(to);

    CString::new(result).unwrap().into_raw()
}

/// Decrypts a String.
#[no_mangle] 
pub extern "C" fn decrypt(from: *const c_char) -> *mut c_char {
    let from = unsafe { CStr::from_ptr(from).to_str().unwrap() };
    let result = cryptor::decrypt(from);

    CString::new(result).unwrap().into_raw()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_string() {
        let str_encoded_b64 = "aGVsbG9fd29ybGRfZnJvbV9j";
        let to_encrypt = "hello_world_from_c";
        let to_encrypt_cstring = CString::new(to_encrypt).unwrap();
        
        let encrypted_result = encrypt(to_encrypt_cstring.as_ptr());
        let encrypted_result = unsafe { CStr::from_ptr(encrypted_result).to_string_lossy().into_owned() };

        assert_eq!(encrypted_result, str_encoded_b64);
    }

    #[test]
    fn test_decrypt_string() {
        let str_decoded_b64 = "hello_world_from_c";
        let to_decrypt_b64 = "aGVsbG9fd29ybGRfZnJvbV9j";
        let to_decrypt_b64_cstring = CString::new(to_decrypt_b64).unwrap();
        
        let decrypted_result = decrypt(to_decrypt_b64_cstring.as_ptr());
        let decrypted_result = unsafe { CStr::from_ptr(decrypted_result).to_string_lossy().into_owned() };

        assert_eq!(decrypted_result, str_decoded_b64);
    }    
}

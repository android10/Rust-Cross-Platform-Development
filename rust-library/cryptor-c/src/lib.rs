use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn encrypt(to: *const c_char) -> *mut c_char {
    let to = unsafe { CStr::from_ptr(to).to_str().unwrap() };

    let result = cryptor::encrypt(to);

    CString::new(result).unwrap().into_raw()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_string() {
        let recipient = "hello";
        let recipient_cstring = CString::new(recipient).unwrap();
        let result = encrypt(recipient_cstring.as_ptr());
        let result = unsafe { CStr::from_ptr(result).to_string_lossy().into_owned() };

        assert_eq!("hello:sdflsjdfsv094", result);
    }
}

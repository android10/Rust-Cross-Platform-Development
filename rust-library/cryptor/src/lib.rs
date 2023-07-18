use base64::{Engine as _, engine::general_purpose};

/// Encrypts a String.
/// 
/// Example:
/// ```
/// use cryptor::encrypt;
/// assert_eq!(encrypt("hello"), "aGVsbG8=");
/// ```
pub fn encrypt(to: &str) -> String {
    let b64 = general_purpose::STANDARD.encode(String::from(to));
    
    b64.to_owned()
}

/// Decrypts a String.
/// 
/// Example:
/// ```
/// use cryptor::decrypt;
/// assert_eq!(decrypt("aGVsbG8="), "hello");
/// ```
pub fn decrypt(from: &str) -> String {
    let b64_bytes = general_purpose::STANDARD.decode(String::from(from)).unwrap();
    let str_result = String::from_utf8(b64_bytes).unwrap();

    str_result.to_owned()
}
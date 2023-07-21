use base64::{
    Engine as _, 
    engine::general_purpose::STANDARD as base64Engine
};

/// Encrypts a String.
/// 
/// Example:
/// ```
/// use cryptor::encrypt;
/// assert_eq!(encrypt("hello"), "aGVsbG8=");
/// assert_eq!(encrypt(""), "");
/// ```
pub fn encrypt(to: &str) -> String {
    base64Engine.encode(String::from(to))
}

/// Decrypts a String.
/// 
/// Example:
/// ```
/// use cryptor::decrypt;
/// assert_eq!(decrypt("aGVsbG8="), "hello");
/// assert_eq!(decrypt(""), "");
/// ```
pub fn decrypt(from: &str) -> String {
    let base64_bytes = base64Engine.decode(
        String::from(from)
    ).unwrap_or(vec![]);

    match String::from_utf8(base64_bytes) {
        Ok(result) => result,
        Err(_) => "".to_owned()
    }
}
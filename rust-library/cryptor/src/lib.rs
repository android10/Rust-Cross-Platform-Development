extern crate base64;

use base64::{encode, decode};

/**
 * Encrypt a String.
 */
pub fn encrypt(to: &str) -> String {
    let b64 = encode(String::from(to));
    
    b64.to_owned()
}

/**
 * Decrypt a String.
 */
pub fn decrypt(from: &str) -> String {
    let b64_bytes = decode(String::from(from)).unwrap();
    let str_result = String::from_utf8(b64_bytes).unwrap();

    str_result.to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_string() {
        let to_encrypt = "hello_world_from_rust";
        let str_encoded_b64 = "aGVsbG9fd29ybGRfZnJvbV9ydXN0";

        let encrypted_result = encrypt(&to_encrypt);
        
        assert_eq!(str_encoded_b64, encrypted_result);
    }

    #[test]
    fn test_decrypt_string() {
        let str_encoded_b64 = "aGVsbG9fd29ybGRfZnJvbV9ydXN0";
        let str_decoded_b64 = "hello_world_from_rust";

        let decrypted_result = decrypt(&str_encoded_b64);
        
        assert_eq!(str_decoded_b64, decrypted_result);
    }
}
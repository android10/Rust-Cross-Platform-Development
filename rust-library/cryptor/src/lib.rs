extern crate base64;

/**
 * Encrypt a String.
 */
pub fn encrypt(to: &str) -> String {
    let b64 = base64::encode(to);
    
    b64.to_owned()
}

/**
 * Decrypt a String.
 */
pub fn decrypt(from: &str) -> String {
    let bytes = base64::decode(from).unwrap();
    let str_result = match String::from_utf8(bytes) {
        Ok(value) => value,
        Err(error) => panic!("Invalid UTF-8 sequence: {}", error),
    };

    str_result.to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_string() {
        let to_encrypt = "hello_world";
        let encrypted = "aGVsbG9fd29ybGQ=";

        let result = encrypt(&to_encrypt);
        
        assert_eq!(encrypted, result);
    }

    #[test]
    fn test_decrypt_string() {
        let to_decrypt = "aGVsbG9fd29ybGQ=";
        let decrypted = "hello_world";

        let result = decrypt(&to_decrypt);
        
        assert_eq!(decrypted, result);
    }
}

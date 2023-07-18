use cryptor;

#[test]
fn test_encrypt_string() {
    let to_encrypt = "hello_world_from_rust";
    let str_encoded_b64 = "aGVsbG9fd29ybGRfZnJvbV9ydXN0";

    let encrypted_result = cryptor::encrypt(&to_encrypt);
    
    assert_eq!(str_encoded_b64, encrypted_result);
}

#[test]
fn test_decrypt_string() {
    let str_encoded_b64 = "aGVsbG9fd29ybGRfZnJvbV9ydXN0";
    let str_decoded_b64 = "hello_world_from_rust";

    let decrypted_result = cryptor::decrypt(&str_encoded_b64);
    
    assert_eq!(str_decoded_b64, decrypted_result);
}
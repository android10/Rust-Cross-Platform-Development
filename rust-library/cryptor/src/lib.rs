pub fn encrypt(to: &str) -> String {
    to.to_owned() + ":sdflsjdfsv094"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_string() {
        let recipient = "hello";
        let result = encrypt(&recipient);
        assert_eq!("hello:sdflsjdfsv094", result);
    }
}

use std::env;

///
/// Returns whehter the current build is Release. 
/// 
pub fn is_release() -> bool {
    Ok("release".to_owned()) == env::var("PROFILE")
}

///
/// Tells Cargo that if the given file changes, 
/// to rerun the build script file passed as a
/// parameter.
///
/// We communicate with cargo from within 
/// the script by writing to stdout.
/// 
/// Example:
/// ```
/// use cryptor_global::system::rerun_if_changed;
/// rerun_if_changed("build.rs");
/// ```
pub fn rerun_if_changed(file_name: &str) {
    println!("cargo:rerun-if-changed={}", file_name);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_debug_profile_then_release_returns_false() {
        assert_eq!(
            is_release(), false, 
            "'is_release()' fn is not returning the right value'",
        );
    }
}
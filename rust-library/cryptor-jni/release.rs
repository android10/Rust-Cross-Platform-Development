use std::env;

pub fn deploy_artifacts() {
    if is_release() {
        panic!("I'm only panicking in release mode deploying artifacts");
    }
}

// Move this to a global helper with another function name, also publish or deploy artifact function, in this case I need to copy
// only the artifacts to their corresponding android directories
fn is_release() -> bool {
    Ok("release".to_owned()) == env::var("PROFILE")
}
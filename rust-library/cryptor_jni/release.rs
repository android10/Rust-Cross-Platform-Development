use std::env;

pub fn deploy_android_artifacts() {
    if is_release() {
        //TODO: copy the artifacts to its corresponding directories
    }
}

// Move this to a global helper with another function name, also publish or deploy artifact function, in this case I need to copy
// only the artifacts to their corresponding android directories
fn is_release() -> bool {
    Ok("release".to_owned()) == env::var("PROFILE")
}
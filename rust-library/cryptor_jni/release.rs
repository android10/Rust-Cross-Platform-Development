use cryptor_global::system;

pub fn build_android_artifacts() {
    if system::is_release() {
        //TODO: build every android artifact
    }
}

pub fn deploy_android_artifacts() {
    if system::is_release() {
        //TODO: copy the artifacts to its corresponding directories
    }
}
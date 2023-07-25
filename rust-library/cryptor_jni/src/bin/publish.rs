// Cargo targets: https://doc.rust-lang.org/cargo/reference/cargo-targets.html#binaries

// https://doc.rust-lang.org/reference/items/modules.html
#[path="../../build.rs"] 
mod build;

use std::env;
use std::path::PathBuf;
use std::path::MAIN_SEPARATOR_STR;

use cryptor_global::{console, io};


fn main() {
    let current_dir = env::current_dir().expect("Cannot read current directory");
    let target_dir = current_dir.parent().expect("Cannot find/read 'target' directory");
    let target_dir_str = target_dir.as_os_str().to_str().expect("Cannot validate 'target' directory");

    for android_target in build::ANDROID_TARGETS_CONFIG.keys() {
        let mut current_target_lib_file_path =


        // fn dotfiles_dir(&self) -> Result<String, DotyError> {
        //     let mut dotfiles_dir = env::var(USSER_HOME_ENV)
        //         .map_err(|_| DotyError::DotfilesInvalidDir)?;
    
        //     dotfiles_dir.push_str(MAIN_SEPARATOR_STR);
        //     dotfiles_dir.push_str(DOTFILES_DIR_NAME);
    
        //     metadata(&dotfiles_dir)
        //         .map_err(|_| DotyError::DotfilesInvalidDir)?;
    
        //     Ok(dotfiles_dir)
        // }


        // let current_target_lib_file = PathBuf::from(
        //     format!("{crate_dir}/target/{target}/release/libcryptor_jni.so", crate_dir = target_dir_str, target = android_target)
        // );

        // // console::out(&current_target_lib_file.into_os_string().to_str().unwrap());

        // if current_target_lib_file.exists() {
        //     console::out("it exists!!!");
        //     console::print(format!("Android Target {} succesfully copied!!!", &android_target));
        // }
    }
}
static CARGO_CONFIG_DIR_NAME: &str = ".cargo";
static CARGO_CONFIG_FILE_NAME: &str = "config";


use std::fs;
use std::path::PathBuf;
use std::fs::File;
use std::io::Result;
use std::io::ErrorKind;

///
/// Creates a cargo config file in the in the directory
/// passed as a parameter.
/// 
/// Example:
/// ```
/// use std::env;
/// use cryptor_global::io;
/// let mut config_file = io::create_cargo_config_file(&env::current_dir().unwrap());
/// ```
pub fn create_cargo_config_file(dir_path: &PathBuf) -> File {
    let config_dir_path = dir_path.join(CARGO_CONFIG_DIR_NAME);
    create_config_dir(&config_dir_path).unwrap_or_else(|error| {
        if error.kind() != ErrorKind::AlreadyExists {
            panic!("Could not create cargo configuration directory: {:?}", error);
        }
    });
    
    let config_file_path = config_dir_path.join(CARGO_CONFIG_FILE_NAME);
    create_config_file(&config_file_path).expect("Could not create cargo configuration file.")
}

fn create_config_dir(dir_path: &PathBuf) -> Result<()> {
    fs::create_dir(&dir_path)
}

fn create_config_file(file_path: &PathBuf) -> Result<File> {
    File::create(&file_path)
}

///
/// Copies the contents of one file to another. 
/// 
///  - https://doc.rust-lang.org/std/fs/fn.copy.html
/// 
pub fn copy_file(from: &str, to: &str)-> std::io::Result<()> {
    fs::copy(from, to)?;
    Ok(())
}
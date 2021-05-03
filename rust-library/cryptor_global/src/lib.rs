static CARGO_CONFIG_DIR_NAME: &str = ".cargo";
static CARGO_CONFIG_FILE_NAME: &str = "config";

pub mod io {
    use std::fs;
    use std::path::PathBuf;
    use std::fs::File;
    use std::io::Result;
    use std::io::ErrorKind;

    /// Creates a cargo config file in the in the directory
    /// passed as a parameter.
    /// 
    /// Example:
    /// ```
    /// use std::env;
    /// use cryptor_global::io;
    /// let mut config_file = io::create_cargo_config_file(&env::current_dir().unwrap());
    /// ```
    ///  
    pub fn create_cargo_config_file(dir_path: &PathBuf) -> File {
        let config_dir_path = dir_path.join(super::CARGO_CONFIG_DIR_NAME);
        create_config_dir(&config_dir_path).unwrap_or_else(|error| {
            if error.kind() != ErrorKind::AlreadyExists {
                panic!("Could not create cargo configuration directory: {:?}", error);
            }
        });
        
        let config_file_path = config_dir_path.join(super::CARGO_CONFIG_FILE_NAME);
        create_config_file(&config_file_path).expect("Could not create cargo configuration file.")
    }

    fn create_config_dir(dir_path: &PathBuf) -> Result<()> {
        fs::create_dir(&dir_path)
    }

    fn create_config_file(file_path: &PathBuf) -> Result<File> {
        File::create(&file_path)
    }
}

pub mod console {
    use std::process::Command;
    use std::io::{self, Write};


    /// Runs a command line command.
    /// 
    /// Example:
    /// ```
    /// use cryptor_global::console::run_command;
    /// 
    /// let mut command_args = Vec::new();
    /// command_args.push("ls");
    /// command_args.push("a");
    /// 
    /// run_command("ls", &command_args.as_slice());
    /// ```
    pub fn run_command(comand: &str, args: &[&str]) {
        let mut command_with_args = Command::new(comand);

        for arg in args.iter() {
            command_with_args.arg(arg);
        };

        let output = command_with_args.output().expect("Failed to execute command");
        
        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
    }

    /// Prints a message to the standard output.
    /// 
    /// Example:
    /// ```
    /// use cryptor_global::console;
    /// console::out("hello");
    /// ```
    pub fn out(message: &str) {
        println!("Message: {}", &message);
    }
}

pub mod system {
    use std::env;

    /// Returns whehter the current build is Release. 
    pub fn is_release() -> bool {
        Ok("release".to_owned()) == env::var("PROFILE")
    }

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
}

#[cfg(test)]
mod tests {
    use super::system::*;

    #[test]
    fn test_given_debug_profile_then_release_returns_false() {
        assert_eq!(is_release(), false);
    }
}
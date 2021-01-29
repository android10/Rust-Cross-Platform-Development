static CARGO_CONFIG_DIR_NAME: &str = ".cargo";
static CARGO_CONFIG_FILE_NAME: &str = "config";

pub mod io {
    use std::fs;
    use std::path::PathBuf;

    pub fn create_cargo_config_file(dir_path: &PathBuf) -> std::io::Result<()> {
        let target = dir_path.join(super::CARGO_CONFIG_DIR_NAME);

        match fs::create_dir(target) {
            Err(error) => panic!("Error creating cargo configuration directory: {}", error),
            Ok(current_dir) => {
                // TODO: create config file and return path
            }
        }
        Ok(())
    }
}

pub mod console {
    pub fn out(message: &str) {
        println!("{}", message);
    }
}
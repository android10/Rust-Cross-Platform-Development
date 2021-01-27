static CARGO_CONFIG_DIR_NAME: &str = ".cargo";
static CARGO_CONFIG_FILE_PATH: &str = concat!("config/", ".cargo");

pub mod io {
    use std::fs;

    pub fn create_cargo_config_file()-> std::io::Result<()> {
        //TODO: pass the current path of the directory
        Ok(())
    }

    pub fn create_cargo_config_dir(parent_dir: &str) -> std::io::Result<()> {
        let mut full_dir_path: String = String::from(parent_dir);
        full_dir_path.push_str(super::CARGO_CONFIG_DIR_NAME); 

        fs::create_dir(full_dir_path)?;
        Ok(())
    }
}

pub mod console {
    pub fn out(message: &str) {
        println!("{}", message);
    }
}
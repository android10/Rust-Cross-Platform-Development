static CARGO_CONFIG_DIR_NAME: &str = ".cargo";
static CARGO_CONFIG_FILE_PATH: &str = concat!("config/", ".cargo");

pub mod io {
    use std::fs;

    pub fn create_cargo_config_file()-> std::io::Result<()> {
        //TODO: pass the current path of the directory
        Ok(())
    }

    pub fn create_cargo_config_dir() -> std::io::Result<()> {
        //TODO: pass the current path of the directory
        fs::create_dir(super::CARGO_CONFIG_DIR_NAME)?;
        Ok(())
    }
}

pub mod console {
    pub fn out(message: &str) {
        println!("{}", message);
    }
}
use crate::{cli::Cli, Result};

use once_cell::sync::OnceCell;

static CONFIG: OnceCell<Config> = OnceCell::new();

pub fn get_config() -> Result<&'static Config> {
    match CONFIG.get() {
        Some(s) => Ok(s),
        None => panic!("Config is not loaded, panicking."),
    }
}

pub fn load_config(args: &Cli) -> Result<()> {
    if CONFIG.get().is_some() {
        panic!("Config is not loaded, panicking.")
    }
    let config = Config::new(args);
    CONFIG.set(config).unwrap();
    Ok(())
}

#[derive(Debug)]
pub struct Config {
    pub verbose: bool,
    pub json: bool,
    pub unattended: bool,
}
impl Config {
    fn new(args: &Cli) -> Self {
        let verbose = args.verbose;
        let json = args.json;
        let unattended = args.unattended;
        let config_path = &args.config;
        Self {
            verbose,
            json,
            unattended,
        }
    }
}

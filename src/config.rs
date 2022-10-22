use crate::{cli::Cli, Result};

use once_cell::sync::OnceCell;

static CONFIG: OnceCell<Config> = OnceCell::new();

pub fn get_config() -> Result<&'static Config> {
    match CONFIG.get() {
        Some(s) => Ok(s),
        None => panic!("Config is not loaded, panicking."),
    }
}

/// Load configuration. Also, it initializes logger.
pub fn load_config(args: &Cli) -> Result<()> {
    if CONFIG.get().is_some() {
        panic!("Config is not loaded, panicking.")
    }

    // Create Config, initialize & set logger.
    let config = Config::new(args);
    config.init_logger();
    CONFIG.set(config).unwrap();

    log::debug!("Loaded configuration.");
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
    fn init_logger(&self) {
        env_logger::init();
        log::debug!("Initialized logger.");
    }
}

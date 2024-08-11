use crate::config::cli_config::CliConfig;

pub struct CliInjestor {
    config: CliConfig,
}

impl CliInjestor {
    pub fn new(config: CliConfig) -> Self {
        Self { config }
    }

    pub fn run(&self) {
        println!("CliInjestor running with config: {:?}", self.config);
    }
}

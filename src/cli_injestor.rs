use config::Config;

pub struct CliInjestor {
    config: Config,
}

impl CliInjestor {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn run(&self) {
        println!("CliInjestor running with config: {:?}", self.config);
    }
}

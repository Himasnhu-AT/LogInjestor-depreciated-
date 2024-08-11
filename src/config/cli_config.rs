use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct CliConfig {
    #[structopt(long, default_value = "info")]
    pub log_level: String,
}

impl CliConfig {
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();
        Self {
            log_level: std::env::var("LOG_LEVEL").expect("LOG_LEVEL must be set"),
        }
    }

    pub fn from_args(log_level: &str) -> Self {
        Self {
            log_level: log_level.to_string(),
        }
    }

    pub fn default() -> Self {
        Self {
            log_level: "info".to_string(),
        }
    }
}

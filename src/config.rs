use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Config {
    #[structopt(long, default_value = "8080")]
    pub api_port: u16,

    #[structopt(long, default_value = "info")]
    pub log_level: String,

    // can be empty
    #[structopt(long, default_value = "")]
    pub database_url_sql: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();
        Self {
            api_port: std::env::var("API_PORT")
                .expect("API_PORT must be set")
                .parse()
                .expect("API_PORT must be a number"),
            log_level: std::env::var("LOG_LEVEL").expect("LOG_LEVEL must be set"),
            database_url_sql: std::env::var("DATABASE_URL_SQL").unwrap_or_default(),
        }
    }

    pub fn default() -> Self {
        Self {
            api_port: 8080,
            log_level: "info".to_string(),
            database_url_sql: "".to_string(),
        }
    }
}

use structopt::StructOpt;

// Add defaults based on docker file, or respective file

#[derive(Debug, StructOpt)]
pub struct DbConfig {
    #[structopt(long, default_value = "8080")]
    pub api_port: u16,

    #[structopt(long, default_value = "")]
    pub database_url_sql: String,

    #[structopt(long)]
    pub auth_key: String,
}

impl DbConfig {
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();
        Self {
            api_port: std::env::var("API_PORT")
                .expect("API_PORT must be set")
                .parse()
                .expect("API_PORT must be a number"),
            database_url_sql: std::env::var("DATABASE_URL_SQL").expect("DataBase URL must be Set"),
            auth_key: std::env::var("AUTH_KEY").expect("AUTH_KEY must be set"),
        }
    }

    pub fn default(auth_key: &str) -> Self {
        Self {
            api_port: "8080".parse().unwrap(),
            database_url_sql: "".to_string(),
            auth_key: auth_key.to_string(),
        }
    }
}

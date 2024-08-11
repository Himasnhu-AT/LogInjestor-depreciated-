pub mod cli_injestor;
pub mod logger;

// Inside the config folder
pub mod config {
    pub mod cli_config;
    pub mod db_config;
}

// LogIngestor divided into two parts:
//  1. CliInjestor: Shows logs in the terminal
//  2. DbIngestor: Stores logs in the database

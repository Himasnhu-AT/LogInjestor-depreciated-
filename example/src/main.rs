use log_injestor::cli_injestor::CliInjestor;
use log_injestor::config::cli_config::CliConfig;
use log_injestor::logger::Logger;

fn main() {
    println!("Hello World");

    // Initialize the CLI configuration
    //
    // ALLOWED LOG LEVELS: error, warn, info, log, debug
    //
    // CLI configuration can be loaded using functions:
    // 1. from_env: Loads the configuration from the environment
    // 2. from_args: Loads the configuration from the arguments {log_level: <log_level>}
    // 3. default: Loads the default configuration {log_level: "info"}
    let cli_config = CliConfig::from_args("error");
    println!("{:?}", cli_config);

    // Create a new CliInjestor with the logger
    let cli_injestor = CliInjestor::new(CliConfig::default());
    cli_injestor.run();

    // Initialize the logger with the configuration
    let logger = Logger::new(cli_config);
    logger.log_info("Logger initialized");
    logger.log_warn("This is a warning");
    logger.log_error("This is an error");
    logger.log_debug("This is a debug message");
    logger.log("This is a log message");
}

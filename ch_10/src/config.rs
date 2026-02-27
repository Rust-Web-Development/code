use clap::Parser;
use std::env;

/// Q&A web service API
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// Which errors we want to log (info, warn or error)
    #[arg(short, long, default_value = "warn")]
    pub log_level: String,
    /// Which PORT the server is listening to
    #[arg(short, long, default_value = "8080")]
    pub port: u16,
    /// Database user
    #[arg(long, default_value = "username")]
    pub db_user: String,
    /// Database user
    #[arg(long)]
    pub db_password: String,
    /// URL for the postgres database
    #[arg(long, default_value = "localhost")]
    pub db_host: String,
    /// PORT number for the database connection
    #[arg(long, default_value = "5432")]
    pub db_port: u16,
    /// Database name
    #[arg(long, default_value = "rustwebdev")]
    pub db_name: String,
}

impl Config {
    pub fn new() -> Result<Config, handle_errors::Error> {
        dotenvy::dotenv().ok();
        let config = Config::parse();

        if env::var("BAD_WORDS_API_KEY").is_err() {
            panic!("BadWords API key not set");
        }

        if env::var("PASETO_KEY").is_err() {
            panic!("PASETO_KEY not set");
        }

        let port = std::env::var("PORT")
            .ok()
            .map(|val| val.parse::<u16>())
            .unwrap_or(Ok(config.port))
            .map_err(handle_errors::Error::ParseError)?;

        let db_user =
            env::var("POSTGRES_USER").unwrap_or(config.db_user.to_owned());
        let db_password = env::var("POSTGRES_PASSWORD").unwrap();
        let db_host =
            env::var("POSTGRES_HOST").unwrap_or(config.db_host.to_owned());
        let db_port = env::var("POSTGRES_PORT")
            .unwrap_or(config.db_port.to_string());
        let db_name =
            env::var("POSTGRES_DB").unwrap_or(config.db_name.to_owned());

        Ok(Config {
            log_level: config.log_level,
            port,
            db_user,
            db_password,
            db_host,
            db_port: db_port
                .parse::<u16>()
                .map_err(handle_errors::Error::ParseError)?,
            db_name,
        })
    }
}

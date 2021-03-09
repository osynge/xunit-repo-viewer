use std::env;

pub(super) fn cli_env() -> super::configuration::Config {
    let mut out = super::configuration::Config::new();
    let config_file = env::var("XRV_CONFIG").ok();
    let database_url = env::var("XRV_DATABASE").ok();
    let database_migrate = match env::var("XRV_DATABASE_MIGRATE") {
        Ok(migrate) => match &*migrate {
            "true" | "1" | "True" | "TRUE" => Some(true),
            "false" | "0" | "False" | "FALSE" => Some(true),
            _ => None,
        },
        Err(_) => None,
    };
    let host = env::var("XRV_HOST").ok();
    let port = match env::var("XRV_PORT") {
        Ok(p) => Some(
            p.parse()
                .expect("Environment variable XRV_PORT is not an integer"),
        ),
        Err(_) => None,
    };
    let log_level = match env::var("XRV_LOG_LEVEL") {
        Ok(p) => Some(
            p.parse()
                .expect("Environment variable XRV_LOG_LEVEL is not an integer"),
        ),
        Err(_) => None,
    };
    super::configuration::Config {
        log_level,
        config_file,
        database_url,
        database_migrate,
        host,
        port,
    }
}

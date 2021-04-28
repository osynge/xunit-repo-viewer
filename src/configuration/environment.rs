use std::env;

fn env_to_bool(env_result: Result<String, env::VarError>) -> Option<bool> {
    match env_result {
        Ok(env_value) => match &*env_value {
            "true" | "1" | "True" | "TRUE" => Some(true),
            "false" | "0" | "False" | "FALSE" => Some(false),
            _ => None,
        },
        Err(_) => None,
    }
}

pub(super) fn cli_env() -> super::configuration::Config {
    let config_file = env::var("XRV_CONFIG").ok();
    let database_url = env::var("XRV_DATABASE").ok();
    let database_migrate = env_to_bool(env::var("XRV_DATABASE_MIGRATE"));
    let host = env::var("XRV_HOST").ok();
    let port = match env::var("XRV_PORT") {
        Ok(p) => Some(
            p.parse()
                .expect("Environment variable XRV_PORT is not an integer"),
        ),
        Err(_) => None,
    };
    let log_in_json = env_to_bool(env::var("XRV_LOG_JSON"));
    let log_level = match env::var("XRV_LOG_LEVEL") {
        Ok(p) => Some(
            p.parse()
                .expect("Environment variable XRV_LOG_LEVEL is not an integer"),
        ),
        Err(_) => None,
    };
    super::configuration::Config {
        log_level,
        log_in_json,
        config_file,
        database_url,
        database_migrate,
        host,
        port,
    }
}

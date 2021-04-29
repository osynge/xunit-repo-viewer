mod configuration;
#[macro_use]
extern crate log;
mod model;
mod routes;
use actix_web_prom::PrometheusMetrics;
use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;
use tracing_actix_web::TracingLogger;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};
use xunit_repo_db;
use xunit_repo_db::db;
use xunit_repo_db::schema;
mod plumbing;
pub type DbConnection = SqliteConnection;
pub type Pool = r2d2::Pool<ConnectionManager<DbConnection>>;
use actix_files::Files;
use actix_web::http::header;
use actix_web::{guard, web, App, HttpRequest, HttpResponse, HttpServer};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

fn level_to_tracing_level(level: &Option<i8>) -> tracing::Level {
    let default = Level::INFO;
    match level {
        Some(p) => {
            if *p < -1 {
                Level::ERROR
            } else if *p == -1 {
                Level::WARN
            } else if *p == 0 {
                Level::INFO
            } else if *p == 1 {
                Level::DEBUG
            } else if *p >= 2 {
                Level::TRACE
            } else {
                default
            }
        }
        None => default,
    }
}

fn log_level_to_env_filter(level: &Option<i8>) -> EnvFilter {
    let default = EnvFilter::new("INFO");
    match level {
        Some(p) => {
            if *p < -1 {
                EnvFilter::new("ERROR")
            } else if *p == -1 {
                EnvFilter::new("WARN")
            } else if *p == 0 {
                EnvFilter::new("INFO")
            } else if *p == 1 {
                EnvFilter::new("DEBUG")
            } else if *p >= 2 {
                EnvFilter::new("TRACE")
            } else {
                default
            }
        }
        None => default,
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let app_cfg = configuration::configure().unwrap();
    let json_logging = match app_cfg.log_in_json {
        Some(log_in_json) => log_in_json,
        None => false,
    };
    let app_name = concat!(env!("CARGO_PKG_NAME"), "-", env!("CARGO_PKG_VERSION")).to_string();
    let (non_blocking_writer, _guard) = tracing_appender::non_blocking(std::io::stdout());
    LogTracer::init().expect("Unable to setup log tracer!");
    match json_logging {
        false => {
            let subscriber = FmtSubscriber::builder()
                // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
                // will be written to stdout.
                .with_max_level(level_to_tracing_level(&app_cfg.log_level))
                .with_writer(non_blocking_writer)
                // completes the builder.
                .finish();
            tracing::subscriber::set_global_default(subscriber).unwrap();
        }
        true => {
            let bunyan_formatting_layer = BunyanFormattingLayer::new(app_name, non_blocking_writer);
            let subscriber = Registry::default()
                .with(log_level_to_env_filter(&app_cfg.log_level))
                .with(JsonStorageLayer)
                .with(bunyan_formatting_layer);
            tracing::subscriber::set_global_default(subscriber).unwrap();
        }
    }
    info!("{:?}", app_cfg);

    println!("{:?}", app_cfg);
    let database_url = match app_cfg.database_url {
        Some(url) => url,
        None => {
            let custom_error =
                std::io::Error::new(std::io::ErrorKind::Other, "No database_url specified");
            return Err(custom_error);
        }
    };
    let host = match app_cfg.host {
        Some(host) => host,
        None => {
            let custom_error = std::io::Error::new(std::io::ErrorKind::Other, "No host specified");
            return Err(custom_error);
        }
    };
    let port = match app_cfg.port {
        Some(port) => port,
        None => {
            let custom_error = std::io::Error::new(std::io::ErrorKind::Other, "No port specified");
            return Err(custom_error);
        }
    };

    let bind = format!("{}:{}", host, port);

    let migrate = match app_cfg.database_migrate {
        Some(database_migrate) => database_migrate,
        None => false,
    };

    let database_pool = match db::establish_connection_pool(&database_url, migrate) {
        Ok(pool) => pool,
        Err(err) => {
            let custom_error = std::io::Error::new(std::io::ErrorKind::Other, err);
            return Err(custom_error);
        }
    };
    let prometheus = PrometheusMetrics::new("api", Some("/metrics"), None);
    HttpServer::new(move || {
        App::new()
            .wrap(prometheus.clone())
            .wrap(TracingLogger)
            // static files
            .data(web::JsonConfig::default().limit(1024 * 1024 * 50))
            .data(database_pool.clone())
            .service(Files::new("/static", "./static/").index_file("index.html"))
            // redirect
            .service(web::resource("/").route(web::get().to(|req: HttpRequest| {
                println!("{:?}", req);
                HttpResponse::Found()
                    .header(header::LOCATION, "static/welcome.html")
                    .finish()
            })))
            .route("/v1/project/all", web::get().to(routes::project_get_all))
            .route(
                "/v1/run_identifer",
                web::get().to(routes::run_identifer_get_all),
            )
            .route("/v1/test_run", web::get().to(routes::test_run_get_all))
            .route(
                "/v1/environment_for_test_run",
                web::get().to(routes::environment_get),
            )
            .route(
                "/v1/environment_details",
                web::get().to(routes::environment_details),
            )
            .route(
                "/v1/test_file_for_test_run",
                web::get().to(routes::test_file_run_get),
            )
            .route(
                "/v1/test_case_pass_from_test_file_run",
                web::get().to(routes::test_case_pass_from_test_file_run),
            )
            .route(
                "/v1/test_case_failure_from_test_file_run",
                web::get().to(routes::test_case_failure_from_test_file_run),
            )
            .route(
                "/v1/test_case_error_from_test_file_run",
                web::get().to(routes::test_case_error_from_test_file_run),
            )
            .route(
                "/v1/test_case_skip_from_test_file_run",
                web::get().to(routes::test_case_skip_from_test_file_run),
            )
            .route(
                "/v1/test_case_class_suite_from_test_case",
                web::get().to(routes::test_case_class_suite_from_test_case),
            )
            .route(
                "/v1/test_case_class_suite_list_from_test_case_list",
                web::post().to(routes::test_case_class_suite_list_from_test_case_list),
            )
            // register favicon
            .service(routes::favicon)
            // default
            .default_service(
                // 404 for GET request
                web::resource("")
                    .route(web::get().to(routes::p404))
                    // all requests that are not `GET`
                    .route(
                        web::route()
                            .guard(guard::Not(guard::Get()))
                            .to(HttpResponse::MethodNotAllowed),
                    ),
            )
    })
    .bind(&bind)?
    .run()
    .await
}

mod configuration;
mod model;
mod routes;
use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;
use xunit_repo_db;
use xunit_repo_db::db;
use xunit_repo_db::schema;
mod plumbing;
pub type DbConnection = SqliteConnection;
pub type Pool = r2d2::Pool<ConnectionManager<DbConnection>>;
use actix_files::Files;
use actix_web::http::{header, Method, StatusCode};
use actix_web::{
    error, get, guard, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer,
    Responder, Result,
};
use serde::{Deserialize, Serialize};
#[derive(Deserialize)]
struct Info {
    username: String,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let app_cfg = configuration::configure().unwrap();
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
    HttpServer::new(move || {
        App::new()
            // static files
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
            ).route(
                "/v1/test_run",
                web::get().to(routes::test_run_get_all),
            ).route(
                "/v1/environment_for_test_run",
                web::get().to(routes::environment_get),
            ).route(
                "/v1/environment_details",
                web::get().to(routes::environment_details),
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
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

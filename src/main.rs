#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate xunit_repo_db;
mod plumbing;
mod routes;
use actix_web::{get, web, HttpRequest, HttpResponse, Result};
use actix_web::{App, HttpServer};
use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;

use xunit_repo_db::db;
use xunit_repo_db::model;
use xunit_repo_db::schema;

pub type DbConnection = SqliteConnection;
pub type Pool = r2d2::Pool<ConnectionManager<DbConnection>>;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let database_url = std::env::var("DATABASE_URL").expect("Database not found");
    let migrate = std::env::var("DATABASE_MIGRATE").is_ok();

    let database_pool = db::establish_connection_pool(&database_url, migrate);
    HttpServer::new(move || {
        App::new()
            .data(database_pool.clone())
            /*.service(crate::routes::index)*/
            .route("/", web::get().to(routes::home))
            .route("/index.js", web::get().to(routes::index_js))
            .route("/v1/project/all", web::get().to(routes::project_get_all))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

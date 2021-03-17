use crate::plumbing::environment::{get_environment_with_test_run, get_environments_details};
use crate::plumbing::project::get_all_project;
use crate::plumbing::run_identifier::get_run_identifier_with_project;
use crate::plumbing::test_run::get_test_run_with_run_identifier;
use crate::Pool;
use actix_web::http::StatusCode;
use actix_web::{get, web, Error, HttpResponse, Result};

use serde::Deserialize;

#[get("/favicon.ico")]
pub(crate) async fn favicon() -> Result<actix_files::NamedFile> {
    Ok(actix_files::NamedFile::open("static/favicon.ico")?)
}

/// 404 handler
pub(crate) async fn p404() -> Result<actix_files::NamedFile> {
    Ok(actix_files::NamedFile::open("static/404.html")?.set_status_code(StatusCode::NOT_FOUND))
}

pub async fn project_get_all(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let conn = pool.get().unwrap();
    Ok(web::block(move || get_all_project(&conn))
        .await
        .map(|project| HttpResponse::Created().json(project))
        .map_err(|_| HttpResponse::InternalServerError())?)
}
#[derive(Deserialize)]
pub struct RunIdentifierParameters {
    pub project_sk: String,
}
pub async fn run_identifer_get_all(
    pool: web::Data<Pool>,
    parameters: web::Query<RunIdentifierParameters>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().unwrap();
    Ok(
        web::block(move || get_run_identifier_with_project(&conn, &parameters.project_sk))
            .await
            .map(|project| HttpResponse::Created().json(project))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

#[derive(Deserialize)]
pub struct TestRunParameters {
    pub run_identifer_sk: String,
}
pub async fn test_run_get_all(
    pool: web::Data<Pool>,
    parameters: web::Query<TestRunParameters>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().unwrap();
    Ok(
        web::block(move || get_test_run_with_run_identifier(&conn, &parameters.run_identifer_sk))
            .await
            .map(|project| HttpResponse::Created().json(project))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

#[derive(Deserialize)]
pub struct EnvironmentParameters {
    pub test_run_sk: String,
}

pub async fn environment_get(
    pool: web::Data<Pool>,
    parameters: web::Query<EnvironmentParameters>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().unwrap();
    Ok(
        web::block(move || get_environment_with_test_run(&conn, &parameters.test_run_sk))
            .await
            .map(|project| HttpResponse::Created().json(project))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

#[derive(Deserialize)]
pub struct EnvironmentDetailsParameters {
    pub environment_sk: String,
}
pub async fn environment_details(
    pool: web::Data<Pool>,
    parameters: web::Query<EnvironmentDetailsParameters>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().unwrap();
    Ok(
        web::block(move || get_environments_details(&conn, &parameters.environment_sk))
            .await
            .map(|project| HttpResponse::Created().json(project))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

#[derive(Deserialize)]
pub struct TestFileRunGetParameters {
    pub test_run_sk: String,
}

pub async fn test_file_run_get(
    pool: web::Data<Pool>,
    parameters: web::Query<TestFileRunGetParameters>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().unwrap();
    Ok(web::block(move || {
        crate::plumbing::test_file_run::get_test_file_run_with_test_run(
            &conn,
            &parameters.test_run_sk,
        )
    })
    .await
    .map(|project| HttpResponse::Created().json(project))
    .map_err(|_| HttpResponse::InternalServerError())?)
}

#[derive(Deserialize)]
pub struct TestCasePassFromTestFileRunGetParameters {
    pub test_file_run_sk: String,
}

pub async fn test_case_pass_from_test_file_run(
    pool: web::Data<Pool>,
    parameters: web::Query<TestCasePassFromTestFileRunGetParameters>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().unwrap();
    Ok(web::block(move || {
        crate::plumbing::test_case_pass::get_test_case_pass_with_test_file_run(
            &conn,
            &parameters.test_file_run_sk,
        )
    })
    .await
    .map(|project| HttpResponse::Created().json(project))
    .map_err(|_| HttpResponse::InternalServerError())?)
}

#[derive(Deserialize)]
pub struct TestCaseFailureFromTestFileRunGetParameters {
    pub test_file_run_sk: String,
}

pub async fn test_case_failure_from_test_file_run(
    pool: web::Data<Pool>,
    parameters: web::Query<TestCaseFailureFromTestFileRunGetParameters>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().unwrap();
    Ok(web::block(move || {
        crate::plumbing::test_case_failure::get_test_case_failure_with_test_file_run(
            &conn,
            &parameters.test_file_run_sk,
        )
    })
    .await
    .map(|project| HttpResponse::Created().json(project))
    .map_err(|_| HttpResponse::InternalServerError())?)
}

/*
#[get("/a/{name}")]
async fn index(
    request: HttpRequest,
    pool: web::Data<Pool>,
    obj: web::Path<MyObj>,
) -> Result<HttpResponse> {
    let headers = get_content_type(&request)?;
    let project_sk = match headers.get("project_sk") {
        Some(p) => p.clone(),
        None => {
            return Ok(HttpResponse::Ok()
                .status(http::StatusCode::BAD_REQUEST)
                .json("No project header"));
        }
    };
    let voo = match headers.get("project_sk") {
        Some(p) => p.clone(),
        None => {
            return Ok(HttpResponse::Ok()
                .status(http::StatusCode::BAD_REQUEST)
                .json("No project header"));
        }
    };
    let conn = pool.get().unwrap();
    Ok(
        web::block(move || get_all_environments_for_project(&conn, &project_sk))
            .await
            .map(|project| HttpResponse::Created().json(project))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}
*/

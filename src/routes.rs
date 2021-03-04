use crate::plumbing::project::get_all_project;
use crate::Pool;
use actix_web::http::StatusCode;
use actix_web::{get, http, web, Error, HttpRequest, HttpResponse, Result};
use std::str::FromStr;

use serde::{Deserialize, Serialize};
pub async fn home() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/index.html")))
}

pub async fn index_js() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/index.js")))
}

pub async fn project_get_all(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let conn = pool.get().unwrap();
    Ok(web::block(move || get_all_project(&conn))
        .await
        .map(|project| HttpResponse::Created().json(project))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

#[derive(Serialize, Deserialize)]
struct MyObj {
    name: String,
}
fn get_content_type<'a>(
    req: &'a HttpRequest,
) -> Result<std::collections::HashMap<String, String>, ()> {
    let mut output = std::collections::HashMap::new();
    for (headername, headervalue) in req.headers().iter() {
        let hn = headername.to_string();
        let hv = match headervalue.to_str() {
            Ok(p) => p,
            Err(p) => continue,
        };
        output.insert(hn.to_string(), hv.to_string());
    }
    Ok(output)
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

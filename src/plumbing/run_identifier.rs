use crate::model::RunIdentifierJson;
use crate::schema::project::dsl::project;
use crate::xunit_repo_db::model::run_identifier;
use crate::DbConnection;
use diesel::prelude::*;
use diesel::query_dsl::QueryDsl;
use diesel::RunQueryDsl;

pub fn get_run_identifier_with_project(
    conn: &DbConnection,
    project_sk: &str,
) -> Result<Vec<RunIdentifierJson>, diesel::result::Error> {
    let tmp = crate::schema::project::dsl::project
        .inner_join(crate::schema::run_identifier::dsl::run_identifier)
        .filter(crate::schema::project::dsl::sk.eq(project_sk))
        .select((
            crate::schema::run_identifier::dsl::sk,
            crate::schema::run_identifier::dsl::client_identifier,
            crate::schema::run_identifier::dsl::created,
        ))
        .load::<(String, String, i64)>(conn)?;
    let result = tmp
        .into_iter()
        .map(
            |(new_sk, new_client_identifier, new_created)| RunIdentifierJson {
                sk: new_sk,
                client_identifier: new_client_identifier,
                created: new_created,
            },
        )
        .collect();
    Ok(result)
}

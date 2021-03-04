use crate::DbConnection;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use diesel::{dsl::insert_into, query_builder::nodes::Identifier};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use xunit_repo_db::schema::environment;

pub fn get_all_environments_for_project(
    conn: &DbConnection,
    project_sk: &str,
) -> Result<Vec<String>, diesel::result::Error> {
    let result = crate::schema::environment::dsl::environment
        .inner_join(crate::schema::project::dsl::project)
        .select(crate::schema::environment::dsl::sk)
        .filter(crate::schema::project::dsl::sk.eq(project_sk))
        .load::<String>(conn)?;
    Ok(result)
}

pub fn get_environments_details(
    conn: &DbConnection,
    env_sk: &str,
) -> Result<Vec<(String, String)>, diesel::result::Error> {
    let result = crate::schema::bind_environment_keyvalue::dsl::bind_environment_keyvalue
        .inner_join(crate::schema::environment::dsl::environment)
        .inner_join(crate::schema::keyvalue::dsl::keyvalue)
        .filter(crate::schema::environment::dsl::sk.eq(env_sk))
        .select((
            crate::schema::keyvalue::dsl::key,
            crate::schema::keyvalue::dsl::value,
        ))
        .load::<(String, String)>(conn)?;
    Ok(result)
}

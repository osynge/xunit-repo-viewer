use crate::DbConnection;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectJson {
    pub sk: String,
    pub identifier: String,
    pub human_name: String,
}

pub fn get_all_project(conn: &DbConnection) -> Result<Vec<ProjectJson>, diesel::result::Error> {
    use crate::schema::project::dsl::*;
    let tmp = project
        .select((sk, identifier, human_name))
        .order(id.desc())
        .load::<(String, String, String)>(conn)?;
    let result = tmp
        .into_iter()
        .map(|(new_sk, new_identifier, new_human_name)| ProjectJson {
            sk: new_sk,
            identifier: new_identifier,
            human_name: new_human_name,
        })
        .collect();
    Ok(result)
}

use crate::model::EnvironmentJson;
use crate::DbConnection;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use std::collections::HashMap;

pub fn get_environment_with_test_run(
    conn: &DbConnection,
    test_run_sk: &str,
) -> Result<Vec<EnvironmentJson>, diesel::result::Error> {
    let tmp = crate::schema::test_run::dsl::test_run
        .inner_join(crate::schema::environment::dsl::environment)
        .filter(crate::schema::test_run::dsl::sk.eq(test_run_sk))
        .select((
            crate::schema::environment::dsl::sk,
            crate::schema::environment::dsl::hash_keyvalue,
        ))
        .load::<(String, String)>(conn)?;
    let result = tmp
        .into_iter()
        .map(|(new_sk, new_hash_keyvalue)| EnvironmentJson {
            sk: new_sk,
            hash_keyvalue: new_hash_keyvalue,
        })
        .collect();
    Ok(result)
}

pub fn get_environments_details(
    conn: &DbConnection,
    env_sk: &str,
) -> Result<HashMap<String, String>, diesel::result::Error> {
    let tmp = crate::schema::bind_environment_keyvalue::dsl::bind_environment_keyvalue
        .inner_join(crate::schema::environment::dsl::environment)
        .inner_join(crate::schema::keyvalue::dsl::keyvalue)
        .filter(crate::schema::environment::dsl::sk.eq(env_sk))
        .select((
            crate::schema::keyvalue::dsl::key,
            crate::schema::keyvalue::dsl::value,
        ))
        .load::<(String, String)>(conn)?;
    let result = tmp.into_iter().collect();
    Ok(result)
}

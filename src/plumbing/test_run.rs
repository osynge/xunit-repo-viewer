use crate::model::TestRunJson;
use crate::schema::project::dsl::project;
use crate::xunit_repo_db::model::run_identifier;
use crate::xunit_repo_db::model::test_run;
use crate::DbConnection;
use diesel::prelude::*;
use diesel::query_dsl::QueryDsl;
use diesel::RunQueryDsl;

pub fn get_test_run_with_run_identifier(
    conn: &DbConnection,
    run_identifier_sk: &str,
) -> Result<Vec<TestRunJson>, diesel::result::Error> {
    let tmp = crate::schema::run_identifier::dsl::run_identifier
        .inner_join(crate::schema::test_run::dsl::test_run)
        .filter(crate::schema::run_identifier::dsl::sk.eq(run_identifier_sk))
        .select((
            crate::schema::test_run::dsl::sk,
            crate::schema::test_run::dsl::created,
        ))
        .load::<(String, i64)>(conn)?;
    let result = tmp
        .into_iter()
        .map(
            |(new_sk, new_created)| TestRunJson {
                sk: new_sk,
                created: new_created,
            },
        )
        .collect();
    Ok(result)
}

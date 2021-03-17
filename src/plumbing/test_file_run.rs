use crate::model::TestFileRunJson;
use crate::schema::project::dsl::project;
use crate::xunit_repo_db::model::test_file;
use crate::xunit_repo_db::model::test_file_run;
use crate::xunit_repo_db::model::test_run;
use crate::DbConnection;
use diesel::prelude::*;
use diesel::query_dsl::QueryDsl;
use diesel::RunQueryDsl;

pub fn get_test_file_run_with_test_run(
    conn: &DbConnection,
    run_identifier_sk: &str,
) -> Result<Vec<TestFileRunJson>, diesel::result::Error> {
    let tmp = crate::schema::test_file_run::dsl::test_file_run
        .inner_join(crate::schema::test_file::dsl::test_file)
        .inner_join(crate::schema::test_run::dsl::test_run)
        .filter(crate::schema::test_run::dsl::sk.eq(run_identifier_sk))
        .select((
            crate::schema::test_file_run::dsl::sk,
            crate::schema::test_file::dsl::directory,
            crate::schema::test_file::dsl::file_name,
        ))
        .load::<(String, String, String)>(conn)?;
    let result = tmp
        .into_iter()
        .map(|(new_sk, new_directory, new_file_name)| TestFileRunJson {
            sk: new_sk,
            directory: new_directory,
            file_name: new_file_name,
        })
        .collect();
    Ok(result)
}

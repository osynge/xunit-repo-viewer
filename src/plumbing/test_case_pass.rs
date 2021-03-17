use crate::model::TestCasePassJson;
use crate::DbConnection;
use diesel::prelude::*;
use diesel::query_dsl::QueryDsl;
use diesel::RunQueryDsl;

pub fn get_test_case_pass_with_test_file_run(
    conn: &DbConnection,
    test_file_run_sk: &str,
) -> Result<Vec<TestCasePassJson>, diesel::result::Error> {
    let tmp = crate::schema::test_case_pass::dsl::test_case_pass
        .inner_join(crate::schema::test_file_run::dsl::test_file_run)
        .inner_join(crate::schema::test_case::dsl::test_case)
        .filter(crate::schema::test_file_run::dsl::sk.eq(test_file_run_sk))
        .select((
            crate::schema::test_case::dsl::sk,
            crate::schema::test_case::dsl::name,
            crate::schema::test_case_pass::dsl::time,
        ))
        .load::<(String, String, Option<f32>)>(conn)?;
    let result = tmp
        .into_iter()
        .map(|(new_sk, new_name, new_time)| TestCasePassJson {
            test_case_sk: new_sk,
            name: new_name,
            time: new_time,
        })
        .collect();
    Ok(result)
}

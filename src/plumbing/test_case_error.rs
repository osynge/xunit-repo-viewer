use crate::model::TestCaseErrorJson;
use crate::DbConnection;
use diesel::prelude::*;
use diesel::query_dsl::QueryDsl;
use diesel::RunQueryDsl;

pub fn get_test_case_error_with_test_file_run(
    conn: &DbConnection,
    test_file_run_sk: &str,
) -> Result<Vec<TestCaseErrorJson>, diesel::result::Error> {
    let tmp = crate::schema::test_case_error::dsl::test_case_error
        .inner_join(crate::schema::test_file_run::dsl::test_file_run)
        .inner_join(crate::schema::test_case::dsl::test_case)
        .filter(crate::schema::test_file_run::dsl::sk.eq(test_file_run_sk))
        .select((
            crate::schema::test_case::dsl::sk,
            crate::schema::test_case::dsl::name,
            crate::schema::test_case_error::dsl::time,
            crate::schema::test_case_error::dsl::error_message,
            crate::schema::test_case_error::dsl::error_type,
            crate::schema::test_case_error::dsl::error_description,
            crate::schema::test_case_error::dsl::system_out,
            crate::schema::test_case_error::dsl::system_err,
        ))
        .load::<(
            String,
            String,
            Option<f32>,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
        )>(conn)?;
    let result = tmp
        .into_iter()
        .map(
            |(
                test_case_sk,
                name,
                time,
                error_message,
                error_type,
                error_description,
                system_out,
                system_err,
            )| TestCaseErrorJson {
                test_case_sk,
                name,
                time,
                error_message,
                error_type,
                error_description,
                system_out,
                system_err,
            },
        )
        .collect();
    Ok(result)
}

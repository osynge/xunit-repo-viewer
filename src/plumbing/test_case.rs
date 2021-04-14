use crate::model::TestCase;
use crate::DbConnection;
use diesel::prelude::*;
use diesel::query_dsl::QueryDsl;
use diesel::RunQueryDsl;

pub fn get_test_case_class_and_suite(
    conn: &DbConnection,
    test_case_sk: &str,
) -> Result<Option<TestCase>, diesel::result::Error> {
    let tmp = crate::schema::test_case::dsl::test_case
        .inner_join(crate::schema::test_case_class::dsl::test_case_class)
        .inner_join(crate::schema::test_suite::dsl::test_suite)
        .filter(crate::schema::test_case::dsl::sk.eq(test_case_sk))
        .select((
            crate::schema::test_case_class::dsl::name,
            crate::schema::test_suite::dsl::name,
        ))
        .first(conn)
        .optional()?;
    match tmp {
        Some((new_class, new_suite)) => Ok(Some(TestCase {
            class: new_class,
            suite: new_suite,
        })),
        None => Ok(None),
    }
}

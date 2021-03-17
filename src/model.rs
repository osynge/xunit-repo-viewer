use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct RunIdentifierJson {
    pub sk: String,
    pub client_identifier: String,
    pub created: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestRunJson {
    pub sk: String,
    pub created: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentJson {
    pub sk: String,
    pub hash_keyvalue: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestFileRunJson {
    pub sk: String,
    pub directory: String,
    pub file_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestCasePassJson {
    pub test_case_sk: String,
    pub name: String,
    pub time: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestCaseFailureJson {
    pub test_case_sk: String,
    pub name: String,
    pub time: Option<f32>,
    pub failure_message: Option<String>,
    pub failure_type: Option<String>,
    pub failure_description: Option<String>,
    pub system_out: Option<String>,
    pub system_err: Option<String>,
}

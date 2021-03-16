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

use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct RunIdentifierJson {
    pub sk: String,
    pub client_identifier: String,
    pub created: i64,
}

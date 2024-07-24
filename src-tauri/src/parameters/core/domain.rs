use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::common::secure_string::SecureString;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ParameterValue {
    String(String),
    StringList(Vec<String>),
    SecureString(SecureString),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub value: ParameterValue,
    pub version: i64,
    // #[serde(with = "ts_milliseconds_option")]
    pub last_modified_date: Option<DateTime<Utc>>,
    pub identifier: Option<String>,
}

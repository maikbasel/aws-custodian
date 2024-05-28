use aws_sdk_sts::primitives::DateTime;
use secstr::SecStr;

#[derive(Debug, Clone, PartialEq)]
pub enum ParameterValue {
    String(String),
    StringList(Vec<String>),
    SecureString(SecStr)
}

pub struct Parameter {
    name: String,
    value: ParameterValue,
    version: i64,
    last_modified_date: Option<DateTime>,
    identifier: Option<String>,
}
use aws_sdk_sts::primitives::DateTime;
use secstr::SecStr;

#[derive(Debug, Clone, PartialEq)]
pub enum ParameterValue {
    String(String),
    StringList(Vec<String>),
    SecureString(SecStr),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub value: Option<ParameterValue>,
    pub version: i64,
    pub last_modified_date: Option<DateTime>,
    pub identifier: Option<String>,
}

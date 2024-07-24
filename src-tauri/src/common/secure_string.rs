use std::fmt::Formatter;

use secstr::SecStr;
use serde::de::{Error, Visitor};
use serde::{Deserializer, Serializer};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct SecureString(SecStr);

impl SecureString {
    pub fn new(bytes: Vec<u8>) -> SecureString {
        SecureString(SecStr::from(bytes))
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(self.0.unsecure()).expect("should be serializable to be UTF-8 string")
    }
}

impl From<&str> for SecureString {
    fn from(s: &str) -> Self {
        SecureString(SecStr::from(s))
    }
}

impl From<String> for SecureString {
    fn from(value: String) -> Self {
        SecureString(SecStr::from(value))
    }
}

impl From<Vec<u8>> for SecureString {
    fn from(bytes: Vec<u8>) -> Self {
        SecureString(SecStr::new(bytes))
    }
}

impl serde::Serialize for SecureString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let secure_str = &self.0.unsecure();
        let utf8_str = std::str::from_utf8(secure_str).expect("should serialize to utf8 string");

        serializer.serialize_str(utf8_str)
    }
}

struct SecureStringVisitor;

impl<'de> Visitor<'de> for SecureStringVisitor {
    type Value = SecureString;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("struct SecureString")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(SecureString(SecStr::from(v)))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(SecureString(SecStr::from(v)))
    }
}

impl<'de> serde::Deserialize<'de> for SecureString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(SecureStringVisitor)
    }
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;

    use super::*;

    #[test]
    fn should_serialize_secure_string_to_string() {
        let secure_str = SecureString(SecStr::from("super_secret_password"));

        let serialized = serde_json::to_string(&secure_str).unwrap();

        assert_that!(serialized).is_equal_to("\"super_secret_password\"".to_string())
    }

    #[test]
    fn should_serialize_empty_secure_string_to_string() {
        let secure_str = SecureString(SecStr::from(""));

        let serialized = serde_json::to_string(&secure_str).unwrap();

        assert_that!(serialized).is_equal_to("\"\"".to_string());
    }

    #[test]
    fn should_serialize_secure_string_with_special_chars_to_string() {
        let secure_str = SecureString(SecStr::from("s3cr3t_$tr!ng"));

        let serialized = serde_json::to_string(&secure_str).unwrap();

        assert_eq!(serialized, "\"s3cr3t_$tr!ng\"");
    }

    #[test]
    fn should_deserialize_str_to_secure_string() {
        let serialized = "\"super_secret_password\"";

        let deserialized: SecureString = serde_json::from_str(serialized).unwrap();

        assert_eq!(
            deserialized.0.unsecure(),
            "super_secret_password".as_bytes()
        );
    }

    #[test]
    fn should_deserialize_empty_str_to_secure_string() {
        let serialized = "\"\"";

        let deserialized: SecureString = serde_json::from_str(serialized).unwrap();

        assert_eq!(deserialized.0.unsecure(), "".as_bytes());
    }

    #[test]
    fn should_deserialize_str_with_special_chars_to_secure_string() {
        let serialized = "\"s3cr3t_$tr!ng\"";

        let deserialized: SecureString = serde_json::from_str(serialized).unwrap();

        assert_eq!(deserialized.0.unsecure(), "s3cr3t_$tr!ng".as_bytes());
    }
}

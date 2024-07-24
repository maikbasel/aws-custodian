use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::common::secure_string::SecureString;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
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

impl Parameter {
    pub fn new(
        name: String,
        value: ParameterValue,
        version: i64,
        last_modified_date: Option<DateTime<Utc>>,
        identifier: Option<String>,
    ) -> Self {
        Self {
            name,
            value,
            version,
            last_modified_date,
            identifier,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Parameters {
    #[serde(rename = "parameters")]
    values: Vec<Parameter>,
}

impl Parameters {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }

    pub fn add_parameter(&mut self, parameter: Parameter) {
        self.values.push(parameter);
    }

    pub fn add_all_parameters(&mut self, parameters: Vec<Parameter>) {
        self.values.extend(parameters)
    }

    pub fn values(&self) -> &Vec<Parameter> {
        &self.values
    }

    pub fn sort_parameters_asc(&mut self) {
        self.values.sort_by(|a, b| a.name.cmp(&b.name));
    }
}

#[cfg(test)]
mod tests {
    use fake::faker::lorem::en::Word;
    use fake::Fake;
    use spectral::prelude::*;

    use crate::parameters::core::domain::{Parameter, ParameterValue, Parameters};

    #[test]
    fn should_create_empty_parameters() {
        let expected = Parameters::new();

        let actual = Parameters::default();

        assert_that(&actual).is_equal_to(expected)
    }

    #[test]
    fn should_add_parameter() {
        let mut cut: Parameters = Parameters::new();
        let input_parameter_name: String = Word().fake();
        let input_parameter: Parameter = Parameter::new(
            input_parameter_name.clone(),
            ParameterValue::String("param".to_string()),
            1,
            None,
            None,
        );

        cut.add_parameter(input_parameter.clone());
        let actual = cut
            .values
            .iter()
            .find(|parameter| parameter.name == input_parameter_name);

        assert_eq!(actual, Some(&input_parameter))
    }

    #[test]
    fn should_add_all_parameters() {
        let mut cut: Parameters = Parameters::new();
        let input_parameter_name: String = Word().fake();
        let input_parameter_1: Parameter = Parameter::new(
            input_parameter_name.clone(),
            ParameterValue::String("param1".to_string()),
            1,
            None,
            None,
        );
        let input_parameter_2: Parameter = Parameter::new(
            input_parameter_name.clone(),
            ParameterValue::String("param2".to_string()),
            1,
            None,
            None,
        );

        cut.add_all_parameters(vec![input_parameter_1.clone(), input_parameter_2.clone()]);
        let actual = cut.values;

        assert_that!(actual.contains(&input_parameter_1)).is_true();
        assert_that!(actual.contains(&input_parameter_2)).is_true();
    }

    #[test]
    fn should_return_parameters() {
        let mut cut: Parameters = Parameters::new();
        let input_parameter_name: String = Word().fake();
        let input_parameter: Parameter = Parameter::new(
            input_parameter_name.clone(),
            ParameterValue::String("param".to_string()),
            1,
            None,
            None,
        );

        cut.add_parameter(input_parameter.clone());
        let actual = cut.values();

        assert_that!(actual.len()).is_equal_to(1);
    }

    #[test]
    fn should_sort_parameters_asc() {
        let mut cut: Parameters = Parameters::new();
        let input_parameter_1: Parameter = Parameter::new(
            "c".to_string(),
            ParameterValue::String("c".to_string()),
            1,
            None,
            None,
        );
        let input_parameter_2: Parameter = Parameter::new(
            "b".to_string(),
            ParameterValue::String("b".to_string()),
            1,
            None,
            None,
        );
        let input_parameter_3: Parameter = Parameter::new(
            "a".to_string(),
            ParameterValue::String("a".to_string()),
            1,
            None,
            None,
        );
        cut.add_all_parameters(vec![
            input_parameter_1,
            input_parameter_2,
            input_parameter_3,
        ]);

        cut.sort_parameters_asc();

        let sorted_profiles = cut.values();

        assert_that!(&sorted_profiles[0].name).is_equal_to("a".to_string());
        assert_that!(&sorted_profiles[1].name).is_equal_to("b".to_string());
        assert_that!(&sorted_profiles[2].name).is_equal_to("c".to_string());
    }
}

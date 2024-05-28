use std::fmt::{Display, Formatter};
use error_stack::Context;

#[derive(Debug, PartialEq, Clone)]
pub enum ParameterDataError {
    ParameterMetaDataLoadError
}

impl Context for ParameterDataError {}

impl Display for ParameterDataError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
use crate::core::configuration::domain::{Configuration};
use crate::core::configuration::error::ConfigurationError;

pub trait ConfigurationAPI {
    fn get() -> Result<Configuration, ConfigurationError>;
}
use crate::core::configuration::domain::{Configuration};
use crate::core::configuration::error::ConfigurationError;

pub trait ConfigurationSPI {
    fn load() -> Result<Configuration, ConfigurationError>;
}
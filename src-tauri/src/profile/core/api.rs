use async_trait::async_trait;

use crate::profile::core::domain::ProfileSet;
use crate::profile::core::error::ProfileError;

#[async_trait]
pub trait ProfileAPI: Send + Sync {
    async fn get_profiles<'a>(&'a self) -> error_stack::Result<ProfileSet, ProfileError>;
}

use crate::domain::{DataPointSeries, DataTypeT, DataType, UserId};

// TODO: this should store actual usable credentials
pub struct ProviderCredentials {
    username: String,
    password: String,
}

pub enum ProviderError {
    NotImplementedError,
    TimeoutError,
    AuthenticationError,
    AuthorizationError,
    RateLimitError,
}

pub trait DataProvider {
    fn provider_id() -> &'static str;
    fn provider_name() -> &'static str;
    fn get_supported_metrics() -> Vec<DataType>;  // TODO: what's the type here?
    fn fetch_data<T: DataTypeT>(user_id: UserId, credentials: &ProviderCredentials) -> Result<DataPointSeries<T>, ProviderError>;
}


use super::provider::{DataProvider, ProviderError, ProviderCredentials};
use crate::domain::{DataType, DataTypeT, DataPointSeries, UserId};

pub enum GarminProvider {}
impl DataProvider for GarminProvider {
    fn provider_id() -> &'static str {
        "garmin_connect"
    }

    fn provider_name() -> &'static str {
        "Garmin Connect"
    }

    fn get_supported_metrics() -> Vec<DataType> {
        vec![DataType::HeartRate, DataType::Weight]
    }

    fn fetch_data<T: DataTypeT>(user_id: UserId, credentials: &ProviderCredentials) -> Result<DataPointSeries<T>, ProviderError> {
        // TODO: Implement actual data fetching from Garmin Connect API
        Err(ProviderError::NotImplementedError)
    }
}


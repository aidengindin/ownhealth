use chrono::{DateTime, Utc};
use sqlx::{postgres::PgPool, Pool, Postgres, Error as SqlxError, Row, QueryBuilder};

use crate::domain::{
    DataPoint, DataPointSeries, DataTypeT,
    HeartRateType, WeightType, HydrationType, VO2MaxType,
    SleepDurationType, SleepStageType,
    UserId,
};

#[derive(Clone)]
pub struct Database {
    pool: Pool<Postgres>,
}

impl Database {
    pub async fn new() -> Result<Self, SqlxError> {
        // TODO: replace with config-based string
        let connection_string = "postgres://user:pass@host/database";
        let pool = PgPool::connect(connection_string).await?;
        Ok(Database { pool })
    }

    async fn fetch_data_points<T: DataTypeT>(
        &self, 
        user_id: &UserId, 
        query: &str,
        start_date: Option<DateTime<Utc>>,
        end_date: Option<DateTime<Utc>>
    ) -> Result<DataPointSeries<T>, SqlxError> {
        let mut query_builder = QueryBuilder::new(query);
        
        // Add WHERE clause if it's not already in the query
        if !query.to_lowercase().contains("where") {
            query_builder.push(" WHERE ");
        } else {
            query_builder.push(" AND ");
        }
        
        query_builder.push("user_id = ");
        query_builder.push_bind(user_id.to_string());
        
        if let Some(start) = start_date {
            query_builder.push(" AND timestamp >= ");
            query_builder.push_bind(start);
        }
        
        if let Some(end) = end_date {
            query_builder.push(" AND timestamp <= ");
            query_builder.push_bind(end);
        }
        
        // Add ORDER BY if not already in the query
        if !query.to_lowercase().contains("order by") {
            query_builder.push(" ORDER BY timestamp");
        }
        
        let query = query_builder.build();
        let rows = query.fetch_all(&self.pool).await?;

        let mut points = Vec::new();

        for row in rows {
            let value = T::from_db_row(&row)?;
            let timestamp: DateTime<Utc> = row.try_get("timestamp")?;
            points.push(DataPoint::<T>::new(value, timestamp));
        }

        Ok(DataPointSeries::<T>::new(points))
    }

    pub async fn fetch_heart_rate(
        &self, 
        user_id: &UserId,
        start_date: Option<DateTime<Utc>>,
        end_date: Option<DateTime<Utc>>
    ) -> Result<DataPointSeries<HeartRateType>, SqlxError> {
        let query = "SELECT value, timestamp FROM heart_rate";
        self.fetch_data_points::<HeartRateType>(user_id, query, start_date, end_date).await
    }

    pub async fn fetch_weight(
        &self, 
        user_id: &UserId,
        start_date: Option<DateTime<Utc>>,
        end_date: Option<DateTime<Utc>>
    ) -> Result<DataPointSeries<WeightType>, SqlxError> {
        let query = "SELECT value, timestamp FROM weight";
        self.fetch_data_points::<WeightType>(user_id, query, start_date, end_date).await
    }

    pub async fn fetch_hydration(
        &self, 
        user_id: &UserId,
        start_date: Option<DateTime<Utc>>,
        end_date: Option<DateTime<Utc>>
    ) -> Result<DataPointSeries<HydrationType>, SqlxError> {
        let query = "SELECT value, timestamp FROM hydration";
        self.fetch_data_points::<HydrationType>(user_id, query, start_date, end_date).await
    }

    pub async fn fetch_vo2_max(
        &self, 
        user_id: &UserId,
        start_date: Option<DateTime<Utc>>,
        end_date: Option<DateTime<Utc>>
    ) -> Result<DataPointSeries<VO2MaxType>, SqlxError> {
        let query = "SELECT value, timestamp FROM vo2_max";
        self.fetch_data_points::<VO2MaxType>(user_id, query, start_date, end_date).await
    }

    pub async fn fetch_sleep_duration(
        &self, 
        user_id: &UserId,
        start_date: Option<DateTime<Utc>>,
        end_date: Option<DateTime<Utc>>
    ) -> Result<DataPointSeries<SleepDurationType>, SqlxError> {
        let query = "SELECT value, timestamp FROM sleep_duration";
        self.fetch_data_points::<SleepDurationType>(user_id, query, start_date, end_date).await
    }

    pub async fn fetch_sleep_stages(
        &self, 
        user_id: &UserId,
        start_date: Option<DateTime<Utc>>,
        end_date: Option<DateTime<Utc>>
    ) -> Result<DataPointSeries<SleepStageType>, SqlxError> {
        let query = "SELECT value, timestamp FROM sleep_stages";
        self.fetch_data_points::<SleepStageType>(user_id, query, start_date, end_date).await
    }
}


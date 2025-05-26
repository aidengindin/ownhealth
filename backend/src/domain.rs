use chrono::{DateTime, Utc};
use serde::{Serialize, Serializer};
use std::fmt::{Debug, Display};
use uuid::Uuid;
use sqlx::{postgres::PgRow, Error as SqlxError, Result, Row};

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct UserId (Uuid);
impl UserId {
    pub fn new() -> Self {
        UserId(Uuid::new_v4())
    }
    pub fn from_existing(id: &str) -> Result<Self, uuid::Error> {
        Ok(UserId(Uuid::parse_str(id)?))
    }
}
impl Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub enum Unit {
    Unitless,
    Bpm,
    Kg,
    Ml,
    #[serde(rename = "ml_kg_min")]
    VO2Max,
    Min,
    #[serde(rename = "score_100")]
    Score100,
}

pub enum DataType {
    HeartRate,
    Weight,
    Hydration,
    VO2Max,
    SleepDuration,
    SleepStage,
}

pub trait DataTypeT {
    type ValueType: Serialize + Clone + Debug;

    fn unit() -> Unit;
    fn name() -> &'static str;
    fn from_db_row(row: &PgRow) -> Result<Self::ValueType, SqlxError>;
}

#[derive(Serialize, Debug)]
pub enum HeartRateType {}
impl DataTypeT for HeartRateType {
    type ValueType = u16;

    fn unit() -> Unit { Unit::Bpm }
    fn name() -> &'static str { "Heart rate" }
    fn from_db_row(row: &PgRow) -> Result<Self::ValueType, SqlxError> {
        let value: i32 = row.try_get("value")?;
        u16::try_from(value).map_err(|_| SqlxError::ColumnNotFound("Value out of range for u16".to_string()))
    }
}

#[derive(Serialize, Debug)]
pub enum WeightType {}
impl DataTypeT for WeightType {
    type ValueType = f64;

    fn unit() -> Unit { Unit::Kg }
    fn name() -> &'static str { "Weight" }
    fn from_db_row(row: &PgRow) -> Result<Self::ValueType, SqlxError> {
        let value: f64 = row.try_get("value")?;
        Ok(value)
    }
}

#[derive(Serialize, Debug)]
pub enum HydrationType {}
impl DataTypeT for HydrationType {
    type ValueType = f64;

    fn unit() -> Unit { Unit::Ml }
    fn name() -> &'static str { "Hydration" }
    fn from_db_row(row: &PgRow) -> Result<Self::ValueType, SqlxError> {
        let value: f64 = row.try_get("value")?;
        Ok(value)
    }
}

#[derive(Serialize, Debug)]
pub enum VO2MaxType {}
impl DataTypeT for VO2MaxType {
    type ValueType = f64;

    fn unit() -> Unit { Unit::VO2Max }
    fn name() -> &'static str { "VO2Max" }
    fn from_db_row(row: &PgRow) -> Result<Self::ValueType, SqlxError> {
        let value: f64 = row.try_get("value")?;
        Ok(value)
    }
}

#[derive(Serialize, Debug)]
pub enum SleepDurationType {}
impl DataTypeT for SleepDurationType {
    type ValueType = i32;

    fn unit() -> Unit { Unit::Min }
    fn name() -> &'static str { "Sleep duration" }
    fn from_db_row(row: &PgRow) -> Result<Self::ValueType, SqlxError> {
        Ok(row.try_get("value")?)
    }
}

#[derive(Clone, Serialize, Debug, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "sleep_stage", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum SleepStage {
    Awake,
    Light,
    Deep,
    REM,
}

#[derive(Serialize, Debug)]
pub enum SleepStageType {}
impl DataTypeT for SleepStageType {
    type ValueType = SleepStage;

    fn unit() -> Unit { Unit::Unitless }
    fn name() -> &'static str { "Sleep stage" }
    fn from_db_row(row: &PgRow) -> Result<Self::ValueType, SqlxError> {
        Ok(row.try_get("value")?)
    }
}

impl Unit {
    pub fn name(&self) -> &'static str {
        match self {
            Unit::Unitless => "",
            Unit::Bpm => "bpm",
            Unit::Kg => "kg",
            Unit::Ml => "mL",
            Unit::VO2Max => "mL/kg/min",
            Unit::Min => "min",
            Unit::Score100 => "/100",
        }
    }
}

fn serialize_unit_as_name<S>(unit: &Unit, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(unit.name())
}

#[derive(Serialize, Debug)]
pub struct DataPoint<T: DataTypeT> {
    value: T::ValueType,
    #[serde(with = "chrono::serde::ts_seconds")]
    timestamp: DateTime<Utc>,
}

impl<T: DataTypeT> DataPoint<T> {
    pub fn new(value: T::ValueType, timestamp: DateTime<Utc>) -> Self {
        DataPoint {
            value,
            timestamp,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct DataPointSeries<T: DataTypeT> {
    points: Vec<DataPoint<T>>,
    #[serde(serialize_with = "serialize_unit_as_name")]
    unit: Unit,
}

impl<T: DataTypeT> DataPointSeries<T> {
    pub fn new(points: Vec<DataPoint<T>>) -> Self {
        DataPointSeries {
            points,
            unit: T::unit(),
        }
    }
}


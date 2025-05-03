use chrono::{DateTime, Utc};
use serde::{Serialize, Serializer};
use std::fmt::Debug;

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

pub trait DataTypeT {
    type ValueType: Serialize + Clone + Debug;

    fn unit() -> Unit;
    fn name() -> &'static str;
}

#[derive(Serialize, Debug)]
pub enum HeartRateType {}
impl DataTypeT for HeartRateType {
    type ValueType = u16;

    fn unit() -> Unit { Unit::Bpm }
    fn name() -> &'static str { "Heart rate" }
}

#[derive(Serialize, Debug)]
pub enum WeightType {}
impl DataTypeT for WeightType {
    type ValueType = f64;

    fn unit() -> Unit { Unit::Kg }
    fn name() -> &'static str { "Weight" }
}

#[derive(Serialize, Debug)]
pub enum HydrationType {}
impl DataTypeT for HydrationType {
    type ValueType = f64;

    fn unit() -> Unit { Unit::Ml }
    fn name() -> &'static str { "Hydration" }
}

#[derive(Serialize, Debug)]
pub enum VO2MaxType {}
impl DataTypeT for VO2MaxType {
    type ValueType = f64;

    fn unit() -> Unit { Unit::VO2Max }
    fn name() -> &'static str { "VO2Max" }
}

#[derive(Serialize, Debug)]
pub enum SleepDurationType {}
impl DataTypeT for SleepDurationType {
    type ValueType = i32;

    fn unit() -> Unit { Unit::Min }
    fn name() -> &'static str { "Sleep duration" }
}

#[derive(Clone, Serialize, Debug, Copy, PartialEq, Eq)]
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


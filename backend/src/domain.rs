use chrono::{DateTime, Utc};

pub enum Unit {
    Unitless,
    Bpm,
    Kg,
    Ml,
    VO2Max,
    Min,
    Score100,
}

pub trait DataTypeT {
    type ValueType: Clone;

    fn unit(&self) -> Unit;
    fn name(&self) -> &'static str;
}

pub enum HeartRateType {}
impl DataTypeT for HeartRateType {
    type ValueType = f64;

    fn unit(&self) -> Unit { Unit::Bpm }
    fn name(&self) -> &'static str { "Heart rate" }
}

pub enum WeightType {}
impl DataTypeT for WeightType {
    type ValueType = f64;

    fn unit(&self) -> Unit { Unit::Kg }
    fn name(&self) -> &'static str { "Weight" }
}

pub enum HydrationType {}
impl DataTypeT for HydrationType {
    type ValueType = f64;

    fn unit(&self) -> Unit { Unit::Ml }
    fn name(&self) -> &'static str { "Hydration" }
}

pub enum VO2MaxType {}
impl DataTypeT for VO2MaxType {
    type ValueType = f64;

    fn unit(&self) -> Unit { Unit::VO2Max }
    fn name(&self) -> &'static str { "VO2Max" }
}

pub enum SleepDurationType {}
impl DataTypeT for SleepDurationType {
    type ValueType = i32;

    fn unit(&self) -> Unit { Unit::Min }
    fn name(&self) -> &'static str { "Sleep duration" }
}

#[derive(Clone)]
pub enum SleepStage {
    Awake,
    Light,
    Deep,
    REM,
}
pub enum SleepStageType {}
impl DataTypeT for SleepStageType {
    type ValueType = SleepStage;

    fn unit(&self) -> Unit { Unit::Unitless }
    fn name(&self) -> &'static str { "Sleep stage" }
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

pub struct DataPoint<T: DataTypeT> {
    data_type: T,
    value: T::ValueType,
    timestamp: DateTime<Utc>,
}


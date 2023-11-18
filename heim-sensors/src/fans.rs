
use heim_common::{prelude::*, units::RationalFrequency};
//use heim_common::units::Frequency;

use crate::sys;

/// Hardware temperature sensor.
#[derive(Debug)]
pub struct FanSensor {
    pub(crate) unit: String,
    pub(crate) label: Option<String>,
    //TODO: Should this be a different type? Is using f64 as quantity ok? Will cause some small imprecision
    pub(crate) current: RationalFrequency,
}

impl FanSensor{
    /// Returns sensor unit name.
    pub fn unit(&self) -> &str {
        &self.unit
    }
    
    /// Returns sensor label.
    pub fn label(&self)->Option<&str>{
        self.label.as_ref().map(String::as_str)
    }
    /// Returns the frequency of the fan in rpm
    pub fn current(&self)->RationalFrequency{
        self.current
    }
}

/// Returns a stream over the [fan sensors] statistics.
///
/// ## Compatibility
///
/// At the moment, this function works only with Linux.
/// For other platforms it returns an empty stream.
///
/// [fan sensors]: ./struct.FanSensor.html
pub fn fans() -> impl Stream<Item = Result<FanSensor>> + Send {
    sys::fans()
}

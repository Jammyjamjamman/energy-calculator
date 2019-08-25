// Methods to convert units from imperial to metric.
use std::convert::From;

#[derive(Debug, Clone, Copy)]
pub struct Metres(f64);

#[derive(Debug, Clone, Copy)]
pub struct Kilometres(f64);

#[derive(Debug, Clone, Copy)]
pub struct Miles(f64);

#[derive(Debug, Clone, Copy)]
pub struct Seconds(pub f64);

#[derive(Debug, Clone, Copy)]
pub struct Hours(pub f64);

#[derive(Debug, Clone, Copy)]
pub struct MetresPerSecond(pub f64);

#[derive(Debug, Clone, Copy)]
pub struct MilesPerHour(pub f64);

#[derive(Debug, Clone, Copy)]
pub struct KilometresPerHour(pub f64);

// Function to increase result by order of magnitude e.g. normal to kilo, kilo to Mega etc.
fn increase_magnitude(val: f64) -> f64 {
    val*1000f64
}

// Functions to convert from S.I. units to other units.
// TODO: add more conversions.
fn metres_to_miles(length: f64) -> f64 {
    length/1609.344f64
}

fn seconds_to_hours(time: f64) -> f64 {
    time/3600f64
}

impl From<Metres> for Miles {
    fn from(measure: Metres) -> Self {
        Miles(metres_to_miles(measure.0))
    }
}

impl From<Seconds> for Hours {
    fn from(time: Seconds) -> Self {
        Hours(seconds_to_hours(time.0))
    }
}

impl From<MetresPerSecond> for MilesPerHour {
    fn from(speed: MetresPerSecond) -> Self {
        // Convert metres per second to miles per hour. The reason that
        // we divide the "seconds to hours" is because speed is distance
        // times the RECIPROCAL (1/x) of time, i.e. distance/time.
        MilesPerHour(metres_to_miles(speed.0)/seconds_to_hours(1f64))
    }
}

impl From<MetresPerSecond> for KilometresPerHour {
    fn from(speed: MetresPerSecond) -> Self {
        KilometresPerHour(speed.0/increase_magnitude(seconds_to_hours(1f64)))
    }
}
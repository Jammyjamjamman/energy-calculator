// Methods to convert units from imperial to metric.
use std::convert::From;

// Temperature
#[derive(Debug, Clone, Copy)]
pub struct Kelvin(pub f64);

#[derive(Debug, Clone, Copy)]
pub struct Celsius(pub f64);

#[derive(Debug, Clone, Copy)]
pub struct Fahrenheit(pub f64);

fn kelvin_to_fahrenheit(k: f64) -> f64 {
    k * (9.0 / 5.0) - 459.67
}

impl From<Kelvin> for Fahrenheit {
    fn from(temperature: Kelvin) -> Self {
        Fahrenheit(kelvin_to_fahrenheit(temperature.0))
    }
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * (9.0 / 5.0) + 32.0
}

impl From<Celsius> for Fahrenheit {
    fn from(temperature: Celsius) -> Self {
        Fahrenheit(celsius_to_fahrenheit(temperature.0))
    }
}

// Weight
#[derive(Debug, Clone, Copy)]
pub struct Kilograms(pub f64);

#[derive(Debug, Clone, Copy)]
pub struct Pounds(pub f64);

fn kg_to_pounds(kg: f64) -> f64 {
    kg * 2.2046226218488
}

impl From<Kilograms> for Pounds {
    fn from(weight: Kilograms) -> Self {
        Pounds(kg_to_pounds(weight.0))
    }
}

// Force
#[derive(Debug, Clone, Copy)]
pub struct Newtons(pub f64);

#[derive(Debug, Clone, Copy)]
pub struct PoundForce(pub f64);

fn newtons_to_pound_force(newtons: f64) -> f64 {
    newtons * 0.224809
}

impl From<Newtons> for PoundForce {
    fn from(force: Newtons) -> Self {
        PoundForce(newtons_to_pound_force(force.0))
    }
}

// Torque
#[derive(Debug, Clone, Copy)]
pub struct NewtonMetre(pub f64);

#[derive(Debug, Clone, Copy)]
pub struct PoundFeet(pub f64);

fn newton_metre_to_pound_feet(newton_metre: f64) -> f64 {
    newton_metre * 0.73756214927727
}

impl From<NewtonMetre> for PoundFeet {
    fn from(torque: NewtonMetre) -> Self {
        PoundFeet(newton_metre_to_pound_feet(torque.0))
    }
}


// Energy
#[derive(Debug, Clone, Copy)]
pub struct Joules(pub f64);

#[derive(Debug, Clone, Copy)]
pub struct ElectronVolts(pub f64);

fn joules_to_electron_volts(joules: f64) -> f64 {
    let m = 10_i64;
    joules * 6.241509 * (m.pow(18) as f64)
}

impl From<Joules> for ElectronVolts {
    fn from(energy: Joules) -> Self {
        ElectronVolts(joules_to_electron_volts(energy.0))
    }
}

// Time
#[derive(Debug, Clone, Copy)]
pub struct Seconds(pub f64);

#[derive(Debug, Clone, Copy)]
pub struct Hours(pub f64);

fn seconds_to_hours(time: f64) -> f64 {
    time / 3600f64
}

impl From<Seconds> for Hours {
    fn from(time: Seconds) -> Self {
        Hours(seconds_to_hours(time.0))
    }
}

// Distance/Speed
// Coding Style: Use "Metres" not "Meters"
#[derive(Debug, Clone, Copy)]
pub struct Metres(pub f64);

#[derive(Debug, Clone, Copy)]
pub struct Feet(pub f64);

#[derive(Debug, Clone, Copy)]
pub struct Kilometres(f64);

#[derive(Debug, Clone, Copy)]
pub struct Miles(f64);

#[derive(Debug, Clone, Copy)]
pub struct MetresPerSecond(pub f64);

#[derive(Debug, Clone, Copy)]
pub struct MilesPerHour(pub f64);

#[derive(Debug, Clone, Copy)]
pub struct KilometresPerHour(pub f64);

fn metres_to_feet(metres: f64) -> f64 {
    metres * 3.28084
}

impl From<Metres> for Feet {
    fn from(distance: Metres) -> Self {
        Feet(metres_to_feet(distance.0))
    }
}

fn metres_to_miles(metres: f64) -> f64 {
    metres / 1609.344f64
}

impl From<Metres> for Miles {
    fn from(distance: Metres) -> Self {
        Miles(metres_to_miles(distance.0))
    }
}

impl From<MetresPerSecond> for MilesPerHour {
    fn from(speed: MetresPerSecond) -> Self {
        // Convert metres per second to miles per hour. The reason that
        // we divide the "seconds to hours" is because speed is distance
        // times the RECIPROCAL (1/x) of time, i.e. distance/time.
        MilesPerHour(metres_to_miles(speed.0) / seconds_to_hours(1f64))
    }
}

// Function to increase result by order of magnitude e.g. normal to kilo, kilo to Mega etc.
fn increase_magnitude(val: f64) -> f64 {
    val * 1000f64
}

impl From<MetresPerSecond> for KilometresPerHour {
    fn from(speed: MetresPerSecond) -> Self {
        KilometresPerHour(speed.0 / increase_magnitude(seconds_to_hours(1f64)))
    }
}

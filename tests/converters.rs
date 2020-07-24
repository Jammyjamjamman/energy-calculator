use energy_calculator::physics::{converter, Item};

#[test]
fn test_speed() {
    let some_speed = converter::MetresPerSecond(30.0);
    assert_eq!(30.0, some_speed.0);
    assert_eq!(
        67.10808876163208,
        converter::MilesPerHour::from(some_speed).0
    );
    assert_eq!(108.0, converter::KilometresPerHour::from(some_speed).0);
}

#[test]
fn test_distance() {
    let some_distance = converter::Metres(1.0);
    assert_eq!(3.28084, converter::Feet::from(some_distance).0);
}

#[test]
fn test_energy() {
    let some_energy = converter::Joules(5.0);
    assert_eq!(
        31207545000000000000.0,
        converter::ElectronVolts::from(some_energy).0
    );
}

#[test]
fn test_weight() {
    let some_weight = converter::Kilograms(5.75);
    assert_eq!(12.6765800756306, converter::Pounds::from(some_weight).0);
}

#[test]
fn test_temperature() {
    let some_temp = converter::Kelvin(5778.53);
    assert_eq!(9941.684, converter::Fahrenheit::from(some_temp).0);

    let some_temp = converter::Celsius(0.0);
    assert_eq!(32.0, converter::Fahrenheit::from(some_temp).0);
}

#[test]
fn test_time() {
    let some_time = converter::Seconds(3600.0);
    assert_eq!(1.0, converter::Hours::from(some_time).0);
}

#[test]
fn test_force() {
    let some_force = converter::Newtons(5.0);
    assert_eq!(1.124045, converter::PoundForce::from(some_force).0);
}

#[test]
fn test_torque() {
    let some_torque = converter::NewtonMetre(10.0);
    assert_eq!(7.375621492772701, converter::PoundFeet::from(some_torque).0);
}

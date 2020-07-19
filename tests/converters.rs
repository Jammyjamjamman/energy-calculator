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
}

mod physics;
use physics::{Item, converter};

fn main() {
    // should the weight be displayed using the metric system?
    // Should the weight be using a value that can be used to auto-determine its mass?
    // We are assuming mass is given in kg. N.B. these are not realistic
    let data = [
        Item {
            name: String::from("Chocolate chip cookie"),
            weight: String::from("2 ounces"),
            mass: 0.03,
        },
        Item {
            name: String::from("Uranium"),
            weight: String::from("1 ounces"),
            mass: 0.04,
        },
        Item {
            name: String::from("Newborn baby"),
            weight: String::from("8 pounds, 6 ounces"),
            mass: 0.05,
        },
    ];

    let energy = data[0].to_energy();
    println!("mass: {}kg, energy: {}J", data[0].mass, energy.0);

    // Test unit converter.
    let some_speed = converter::MetresPerSecond(30f64);
    println!("Speed in m/s: {}m/s", some_speed.0);
    println!("Speed in mph: {:.1}mph", converter::MilesPerHour::from(some_speed).0);
    println!("Speed in kph: {}kph", converter::KilometresPerHour::from(some_speed).0);
}

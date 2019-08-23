const SPEED_OF_LIGHT : f64 = 299792458f64;

struct Item {
    name: String,
    weight: String,
    mass: f64,
}

struct Energy(f64);


// This gives an Item (which has mass) a method to convert it to energy.
impl Item {
    fn to_energy(&self) -> Energy {
        Energy(self.mass * SPEED_OF_LIGHT * SPEED_OF_LIGHT)
    }
}

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
    println!("mass: {}, energy: {}", data[0].mass, energy.0);
}

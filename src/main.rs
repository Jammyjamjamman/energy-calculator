
struct Item {
    name: String,
    weight: String,
    mass: u64,
}

fn main() {
    println!("Hello, world!");

// should the weight be displayed using the metric system?
// Should the weight be using a value that can be used to auto-determine its mass?
    let data = [
    Item {
        name: String::from("Chocolate chip cookie"),
        weight: String::from("2 ounces"),
        mass: 0,
    },
    Item {
        name: String::from("Uranium"),
        weight: String::from("1 ounces"),
        mass: 0,
    },
    Item {
        name: String::from("Newborn baby"),
        weight: String::from("8 pounds, 6 ounces"),
        mass: 0,
    },
];

}

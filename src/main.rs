mod physics;
use physics::{converter, Item};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    /// Activate debug mode
    // short and long flags (-d, --debug) will be deduced from the field's name
    #[structopt(short, long)]
    debug: bool,

    // Show version
    #[structopt(short = "V", long = "version")]
    version: bool,
}

fn display_package_info() {
    println!(
        "{} version: {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
    println!();
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

    let opt = Opt::from_args();
    if opt.version {
        display_package_info();
    }

    let energy = data[0].to_energy();
    println!("mass: {}kg, energy: {}J", data[0].mass, energy.0);
}

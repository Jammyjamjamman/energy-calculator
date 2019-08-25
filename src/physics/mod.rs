pub mod converter;

const SPEED_OF_LIGHT : f64 = 299792458f64;

pub struct Item {
    pub name: String,
    pub weight: String,
    pub mass: f64
}

pub struct Element {
    pub symbol: String,
    pub atomic_weight: f64
}

pub struct Material {
    pub element: Element,
    pub mass: f64
}

pub struct Energy(pub f64);


// This gives an Item (which has mass) a method to convert it to energy.
impl Item {
    pub fn to_energy(&self) -> Energy {
        Energy(self.mass * SPEED_OF_LIGHT * SPEED_OF_LIGHT)
    }
}
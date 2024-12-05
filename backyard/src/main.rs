use crate::garden::vegetables::Tomato;

pub mod garden;

fn main() {
    let plant = Tomato {};
    println!("I'm growing {plant:?}");
}

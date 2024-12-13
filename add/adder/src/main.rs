use add_one;
use add_two;
use add_rand;

fn main() {
    let num = 68;
    println!("{num} + 1 = {}", add_one::add_one(num));

    let num = 67;
    println!("{num} + 2 = {}", add_two::add_two(num));

    let num = 69;
    println!("{num} + ? = {}", add_rand::add_rand(num));
}

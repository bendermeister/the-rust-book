// constants are always immutable and their type must be annotated
const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;

fn main() {
    let x = 5;
    println!("The value of x is: {x}");

    // let makes variables immutable so this is not allowed
    // x = 6;
    // println!("The value of x is: {x}");

    // Shadowing
    let spaces = "   ";
    let spaces = spaces.len();
    // the first spaces with a string type is overshadowed by the second spaces
    // with an integer type
}

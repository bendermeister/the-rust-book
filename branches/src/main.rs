fn main() {
    let number = 3;

    // basic if conditions
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // if only works with bools
    // if number {
    //     println!("can conditions be something else than bool?")
    // }

    // basic if else shinanigans
    if number == 0 {
        println!("Number is 0");
    } else if number == 1 {
        println!("Number is 1");
    } else if number == 2 {
        println!("Number is 2");
    } else if number == 3 {
        println!("Number is 3");
    } else {
        println!("Number to high for this if else shit");
    }

    // ifs are expressions not statements -> they can return values
    let condition = true;
    let number = if condition { 69 } else { 420 };
    println!("Number = {number}");

    // if you use an if as an expression both arms must have the same type
    // let condition = false;
    // let number = if condition { 69 } else { "weed" };
}

fn main() {
    another_function(69);
    print_labeled_measurement(420, 'b');
    another_function(plus_one(68));
}

fn another_function(x: i32) {
    println!("The value of x: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn plus_one(x: i32) -> i32 {
    // no semicolon = expression, expresions return values
    x + 1
}

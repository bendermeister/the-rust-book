fn main() {
    println!("Hello, world!");

    // this is an infinite loop
    // loop {
    //     println!("again!");
    // }

    // loops are also expressions and may return a value using the break
    // statement
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("result: {result}");

    // loop labels
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }

    // while loop
    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }

    // looping over collections
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    println!();

    // better way
    for element in a {
        println!("the value is: {element}");
    }

    println!("Ranged based for loops!");
    for i in 0..4 {
        println!("i = {i}");
    }

    println!("Reverse range based for loops");
    for i in (0..=4).rev() {
        println!("i = {i}");
    }
}

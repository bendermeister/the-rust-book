// #[derive(Debug)]
// enum IpAddrKind {
//     V4,
//     V6,
// }

// enum IpAddr {
//     V4(String),
//     V6(String),
// }

// fn main() {
//     let four = IpAddrKind::V4;
//     let six = IpAddrKind::V6;
//     println!("four: {:?}", four);
//     println!("six: {:?}", six);

//     let _four = IpAddr::V4(String::from("127.0.0.1"));
//     let _six = IpAddr::V6(String::from("::1"));

//     // Options
//     let _x: i32 = 5;
//     let _y = Some(5);

//     // let sum = x + y; // will result in error because y is of type
//     // Option<{integer}> and could be not real
// }

#[derive(Debug)]
enum State {
    Alabama,
    Ohio,
    Alaska,
    // ...
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}

impl Coin {
    fn value_in_cents(&self) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("Qarter from: {state:?}");
                25
            }
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let c = Coin::Dime;
    println!("c: {}", value_in_cents(&c));
    println!("c: {}", value_in_cents(&c));

    println!("Top to bottom:");
    println!("c: {}", value_in_cents(&Coin::Penny));
    println!("c: {}", value_in_cents(&Coin::Nickel));
    println!("c: {}", value_in_cents(&Coin::Dime));
    println!("c: {}", value_in_cents(&Coin::Quarter(State::Ohio)));
    println!("c: {}", value_in_cents(&Coin::Quarter(State::Alaska)));
    println!("c: {}", value_in_cents(&Coin::Quarter(State::Alabama)));

    let val = Some(68);
    println!("{:?} + 1 = {:?}", val, plus_one(val));
    let val = None;
    println!("{:?} + 1 = {:?}", val, plus_one(val));

    // matching we need 3 for match of 3; 7 for match on 7; all other values
    // match to other and will be bound to other
    let dice_roll = 9;
    match dice_roll {
        3 => println!("you get fancy hat!"),
        7 => println!("you loose fancy hat!"),
        other => println!("you move {other} units"),
    }

    // _ placeholder will match all values
    let dice_roll = 9;
    match dice_roll {
        3 => println!("You win!"),
        _ => println!("You loose!"),
    }

    let config_max = Some(3);
    match config_max {
        Some(max) => println!("The max is {max}"),
        _ => (),
    }
    // this code is equivalent to the code above
    if let Some(max) = config_max {
        println!("The max is {max}");
    }

    // if let syntax is syntax sugar for a match expression which runs code on
    // one pattern and ignores all other patterns
    // if let syntax is compatible with else

    if let Some(max) = config_max {
        println!("The max is {max}");
    } else {
        println!("No max available");
    }
}

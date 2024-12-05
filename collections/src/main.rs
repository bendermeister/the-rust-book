fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let third = &v[2];
    println!("The third element is {third}");

    let third = v.get(2);
    match third {
        Some(&x) => println!("The third element is {x}"),
        None => (),
    }

    let n500 = v.get(500);
    match n500 {
        Some(&x) => println!("The 500th element is {x}"),
        None => println!("There is no 500th element"),
    }

    let borrow = &v[0];
    println!("Borrow is {borrow}");
    v.push(69);
    // this results in an error -> mutable borrow
    // println!("Borrow is {borrow}");

    for i in &v {
        print!("{i} ");
    }
    println!();

    for i in &mut v {
        *i += 1;
    }
    for i in &v {
        print!("{i} ");
    }
    println!();

    // this will always cause an error
    // for i in &mut v {
    //     v.push(*i);
    // }

    let row = vec![
        SpreadsheetCell::Int(-69),
        SpreadsheetCell::Float(420.69),
        SpreadsheetCell::Text(String::from("Hello")),
        SpreadsheetCell::Text(String::from("World")),
    ];

    for cell in &row {
        match cell {
            SpreadsheetCell::Int(x) => println!("Integer: {x}"),
            SpreadsheetCell::Float(x) => println!("Float: {x}"),
            SpreadsheetCell::Text(x) => println!("Text: '{x}'"),
        }
    }

    // all contents of a vector get dropped when the vector gets dropped
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

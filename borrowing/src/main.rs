fn main() {
    let mut s = String::from("Hello World");
    let len = calc_length(&s);
    println!("The String '{s}' is {len} characters long");

    change(&mut s);
    println!("{s}");

    {
        let r1 = &mut s;
        // not allowed because no more than one borrow shall exist
        // let r2 = &mut s;
        //println!("{}, {}", r1, r2);
        println!("{}", r1);
    }

    // this is not only the case for mutable references
    {
        let r0 = &s; // no problem
        let r1 = &s; // no problem
                     //let r2 = &mut s; // BIG PROBLEM
                     //println!("{}, {}, {}", r0, r1, r2);
        println!("{}, {}", r0, r1);
    }
}

fn calc_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(" some more words");
}

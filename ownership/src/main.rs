fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{s}");

    let s1 = String::from("hello");
    let s2 = s1;

    // this is not possible s1 has been moved into s2 therefore s1 is no longer
    // valid
    // println!("s1 = {s1}");

    println!("s2 = '{s2}'");

    // not with clone

    // now possible because we clone s1 (= deep copy) now we don't have the same
    // string 2 times but we have two strings with the same content
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {s1}");
    println!("s2 = '{s2}'");

    let s1 = String::from("Hello World");
    let i1 = 69;

    takes_ownership(s1);
    // does not work because the first function call takes ownership of s1 and
    // drops it after first takes_ownership returns to this scope s1 is no
    // longer valid
    // takes_ownership(s1);
    // takes_ownership(s1);

    // does work because i32 has Copy trait and does not move or drop the value
    // of i1
    makes_copy(i1);
    makes_copy(i1);
    makes_copy(i1);

    {
        let s = gives_ownership();
        println!("{s}");
    }
    // s will be dropped here

    {
        let s = String::from("Hello World!");
        let s = takes_and_gives_back(s);
        // ownership is here again
        println!("{s}");
    }
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}

// ownership will be transfered to caller
fn gives_ownership() -> String {
    String::from("Text")
}

fn takes_and_gives_back(a: String) -> String {
    a
}

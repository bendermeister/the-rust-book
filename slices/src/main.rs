fn main() {
    let s = String::from("Hello World");
    let l = first_word_0(&s);

    println!("{l}");

    // slices
    let hello = &s[0..5];
    let world = &s[6..11];

    println!("Hello: {hello}");
    println!("World: {world}");

    // these two are equal
    let slice0 = &s[0..2];
    let slice1 = &s[..2];
    println!("'{slice0}' == '{slice1}'");

    // these two are equal
    let slice0 = &s[3..s.len()];
    let slice1 = &s[3..];
    println!("'{slice0}' == '{slice1}'");

    // these two are equal
    let slice0 = &s[0..s.len()];
    let slice1 = &s[..];
    println!("'{slice0}' == '{slice1}'");

    // the type of a string slice is &str
    // and now for the grand finale
    let mut s = String::from("Hello World");
    let l = first_word(&s);

    println!("{l}");

    s.clear();
    //println!("{l}"); // this will cause an error because the reference is no
    // longer valid

    // the type of a string slice is &str
    // and now for the grand finale
    let s = String::from("Hello World");
    let l = first_word_better(&s);

    println!("First Word better: {l}");

    // other slices are possible too
    other_slices();
}

// Problem: return the first word in a given string
fn first_word_0(s: &String) -> usize {
    // convert string into an array of bytes
    // does this actually convert the string or just take a reference?
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn first_word_better(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    s
}

fn other_slices() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

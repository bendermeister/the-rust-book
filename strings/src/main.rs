fn main() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // format! does not take ownership of the strings
    let s = format!("{s1}-{s2}-{s3}");

    println!("{s1}");
    println!("{s2}");
    println!("{s3}");
    println!("{s}");

    let s = s1 + &s2;
    println!("{s}");
    //println!("{s1}"); // this causes error '+' takes ownership of s1
    // '+' needs one owning string
}

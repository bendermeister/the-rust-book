struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        email: String::from("arnold@test.com"),
        username: String::from("Arnold"),
        sign_in_count: 1,
    };
    println!("name: {}", user1.email);
    user1.email = String::from("another@email.com");

    let user2 = build_user("bestuser", "someother@mail.com");
    println!("name: {}", user2.username);

    // note: username gets moved here
    // this is tedious
    // let user3 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };
    // struct update syntax
    // note: user1 gets moved here -> if we would have only used active and
    // sign_in_count from user1 user1 would not have been moved as they
    // implement Copy trait
    let _user3 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    let _c = Color(23, 23, 23);

    let _subject = AlwaysEqual;
}

// Tuple Structs
struct Color(i32, i32, i32);

// Unit Like structs
struct AlwaysEqual;

fn build_user(email: &str, username: &str) -> User {
    User {
        active: true,
        username: String::from(email),
        email: String::from(username),
        sign_in_count: 1,
    }
}

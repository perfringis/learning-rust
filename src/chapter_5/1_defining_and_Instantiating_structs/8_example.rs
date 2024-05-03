// Creating Instances from Other Instances with Struct Update Syntax

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // The ".." operator reminds javascript's deconstruction
    // in this example we are updating user2 using user1 values
    // ..user1 must be last and it's checking what has to be updated
    // https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html#variables-and-data-interacting-with-move
    let user2 = User {
        username: String::from("JohnDoe"),
        ..user1
    };

    let username1 = user1.username;
    println!("username1: {username1}");

    let username = user2.username;
    println!("username2: {username}");
}
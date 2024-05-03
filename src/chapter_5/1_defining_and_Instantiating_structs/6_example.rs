// Using the Field Init Shorthand

struct User {
    // active property of struct User is called "field"
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let email = String::from("test@test.com");
    let username = String::from("John Doe");
    let user = build_user(email, username);

    let email = user.email;
    println!("Email {email}");
}

// if the function parameter has the same like field in structure
// then we can use short hand
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // shorthand
        email,    // shorthand
        sign_in_count: 1,
    }
}

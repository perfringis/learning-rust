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

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

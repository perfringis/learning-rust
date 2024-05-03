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

    // creating new struct based on values from the first object
    let user2 = User {
        active: true,
        username: user1.email,
        email: user1.email,
        sign_in_count: 1,
    };
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

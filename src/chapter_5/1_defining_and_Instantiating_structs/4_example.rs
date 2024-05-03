fn main() {
    struct User {
        // active property of struct User is called "field"
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }


    // mutable structure
    // entire struct must be mutable
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // setting new value for fields when structure is mutable
    user1.email = String::from("test@test.com");

    let email = user1.email;
    println!("email {email}");
}

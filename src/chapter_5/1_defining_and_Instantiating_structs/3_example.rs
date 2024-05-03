fn main() {
    struct User {
        // active property of struct User is called "field"
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }


    // fields don't have to be in order
    // this is initialization of struct User
    // this struct is immutable
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // accessing fields by the "." dot sign.
    let email = user1.email;
    println!("email {email}");
}

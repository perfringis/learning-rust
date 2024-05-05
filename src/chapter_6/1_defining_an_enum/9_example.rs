// The Option Enum and Its Advantages Over Null Values

// #[derive(Debug)]

fn main() {
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
}

// The type of some_number is Option<i32>. The type of some_char is Option<char>, which is a different type. 
// Rust can infer these types because we’ve specified a value inside the Some variant. 
// For absent_number, Rust requires us to annotate the overall Option type: the compiler can’t infer the type that the corresponding Some variant will hold by looking only at a None value. 
// Here, we tell Rust that we mean for absent_number to be of type Option<i32>.

// When we have a Some value, we know that a value is present and the value is held within the Some. 
// When we have a None value, in some sense it means the same thing as null: we don’t have a valid value. 
// So why is having Option<T> any better than having null?
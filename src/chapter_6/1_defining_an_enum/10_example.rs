// The Option Enum and Its Advantages Over Null Values

// #[derive(Debug)]

// In short, because Option<T> and T (where T can be any type) are different types, the compiler won’t let us use an Option<T> 
// value as if it were definitely a valid value. For example, this code won’t compile, because it’s trying to add an i8 to an Option<i8>
fn main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;
}

// Intense! In effect, this error message means that Rust doesn’t understand how to add an i8 and an Option<i8>, because they’re different types.
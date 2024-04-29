// What Is Ownership?

// Ownership Rules

// Memory and Allocation

// in that case memory will be allocated at runtime
// Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope.

fn main() {
    // in this example String is special type
    // it is used when we don't know exact size of string type
    // it is used when how String big will be
    // the value is unknown at compile time
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`
}

// this scope is now over, and s is no longer valid

// There is a natural point at which we can return the memory our String needs to the allocator: when s goes out of scope.
// When a variable goes out of scope, Rust calls a special function for us. 
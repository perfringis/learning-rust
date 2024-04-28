// What Is Ownership?

// Ownership Rules

// - Each value in Rust has an owner.
// - There can only be one owner at a time.
// - When the owner goes out of scope, the value will be dropped.

// Variable Scope

fn main() { // s is not valid here, it’s not yet declared
    let s = "hello"; // s is valid from this point forward

    // do stuff with s
} // this scope is now over, and s is no longer valid

// In other words, there are two important points in time here:

// - When s comes into scope, it is valid.
// - It remains valid until it goes out of scope.

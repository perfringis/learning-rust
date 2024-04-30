// What Is Ownership?

// Ownership Rules

// Variables and Data Interacting with Clone

fn main() {
    let s1 = String::from("hello");
    // it is deep copy
    // independent copy of s1
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}
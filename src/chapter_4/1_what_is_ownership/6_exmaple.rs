// What Is Ownership?

// Ownership Rules

// Variables and Data Interacting with Move

// On the following image:
// on the left side where we have, ptr, len and capacity
// on the left side of this diagram data are stored in stack
// on the right side of this diagram is stored in heap
// check this image: https://doc.rust-lang.org/stable/book/img/trpl04-02.svg

fn main() {
    let s1 = String::from("hello");

    // Now the s2 variable is the owner of value
    // variable s1 is not longer valid
    // it is shallow copy
    let s2 = s1;

    // this throws the error
    println!("{}, world!", s1);
}

// This is a problem: when s2 and s1 go out of scope, they will both try to free the same memory.
// This is known as a double free error and is one of the memory safety bugs we mentioned previously.
// Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities.

// If you’ve heard the terms shallow copy and deep copy while working with other languages,
// the concept of copying the pointer, length, and capacity without copying the data probably sounds like making a shallow copy.
// But because Rust also invalidates the first variable, instead of being called a shallow copy, it’s known as a move.
// In this example, we would say that s1 was moved into s2

// https://doc.rust-lang.org/stable/book/img/trpl04-04.svg

// Mutable References

// Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value.
// This code that attempts to create two mutable references to s will fail:

fn main() {
    let mut s = String::from("hello");

    // when variable is mutable and we want to create reference or borrowing mechanism.
    // we have to have only on reference!
    let r1 = &mut s;
    let r2 = &mut s; // it throws error.

    println!("{}, {}", r1, r2);
}

// This error says that this code is invalid because we cannot borrow s as mutable more than once at a time.
// The first mutable borrow is in r1 and must last until it’s used in the println!,
// but between the creation of that mutable reference and its usage, we tried to create another mutable reference in r2 that borrows the same data as r1.

// The restriction preventing multiple mutable references to the same data at the same time allows for mutation but in a very controlled fashion.
// It’s something that new Rustaceans struggle with because most languages let you mutate whenever you’d like. 
// The benefit of having this restriction is that Rust can prevent data races at compile time. 
// A data race is similar to a race condition and happens when these three behaviors occur:

// - Two or more pointers access the same data at the same time.
// - At least one of the pointers is being used to write to the data.
// - There’s no mechanism being used to synchronize access to the data.

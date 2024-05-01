// Dangling References

// In languages with pointers, it’s easy to erroneously create a dangling pointer—a pointer that references a location in memory
// that may have been given to someone else—by freeing some memory while preserving a pointer to that memory.

fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    // dangle returns a reference to a String
    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!

// Because s is created inside dangle, when the code of dangle is finished, s will be deallocated. 
// But we tried to return a reference to it. 
// That means this reference would be pointing to an invalid String. 
// That’s no good! Rust won’t let us do this.
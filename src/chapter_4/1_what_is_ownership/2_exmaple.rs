// What Is Ownership?

// Ownership Rules

// The String Type

fn main() {
    // it is not working!
    let mut s = "Hello"; // the difference is how literal string vs String type is dealing with memory

    s.push_str(", world");

    println!("{}", s);
}

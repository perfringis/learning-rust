// First, notice that all the tuple code in the variable declaration and the function return value is gone.
// Second, note that we pass &s1 into calculate_length and, in its definition, we take &String rather than String. 
// These ampersands represent references, and they allow you to refer to some value without taking ownership of it

fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

// We call the action of creating a reference borrowing. 
// As in real life, if a person owns something, you can borrow it from them. 
// When you’re done, you have to give it back. You don’t own it.
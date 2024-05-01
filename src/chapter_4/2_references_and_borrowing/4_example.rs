// Mutable References

// First we change s to be mut.
// Then we create a mutable reference with &mut s where we call the change function, 
// and update the function signature to accept a mutable reference with some_string: &mut String. 
// This makes it very clear that the change function will mutate the value it borrows.

fn main() {
  let mut s = String::from("hello");

  change(&mut s);

  println!("s {s}");
}

fn change(some_string: &mut String) {
  some_string.push_str(", world");
}
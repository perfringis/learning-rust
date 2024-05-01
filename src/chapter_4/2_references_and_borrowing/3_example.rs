// Mutable References

fn main() {
  let s = String::from("hello");

  change(&s);
}

fn change(some_string: &String) {
  // if the main variable is immutable it means reference value is immutable too
  some_string.push_str(", world"); // it throws the error, we can't modify borrowed immutable value
}
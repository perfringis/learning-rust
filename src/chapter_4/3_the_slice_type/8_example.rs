// String Literals as Slices

fn main() {
  // The type of s here is &str: it’s a slice pointing to that specific point of the binary. 
  // This is also why string literals are immutable; &str is an immutable reference.
  let s = "Hello, world!";
}
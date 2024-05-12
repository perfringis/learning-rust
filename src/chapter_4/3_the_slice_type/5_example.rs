// String Slices

fn main() {
  let s = String::from("hello");

  let len = s.len();
  
  // immutable reference to whole string
  let slice = &s[0..len];
  let slice2 = &s[..];

  println!("slice {slice}");
  println!("slice2 {slice2}");
}

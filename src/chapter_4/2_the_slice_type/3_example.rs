// String Slices

fn main() {
  let s = String::from("hello");

  // both line are the same
  // in the second line start index is not provided but works the same like first line
  let slice = &s[0..2];
  let slice2 = &s[..2];

  println!("slice {slice}");
  println!("slice2 {slice2}");
}

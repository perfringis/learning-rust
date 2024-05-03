// String Slices

fn main() {
  let s = String::from("hello");

  let len = s.len();
  
  // the first line is limiting end index of sliced part
  // second line is the same like first line
  let slice = &s[3..len];
  let slice2 = &s[3..];

  println!("slice {slice}");
  println!("slice2 {slice2}");
}

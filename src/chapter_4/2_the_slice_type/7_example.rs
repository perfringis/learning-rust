// String Slices

fn main() {
  let s = String::from("Hello World");

  let word = first_word(&s);

  // s.clear(); // error!

  println!("word {word}");

}

fn first_word(s: &String) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' {
          return &s[0..i];
      }
  }

  &s[..]
}
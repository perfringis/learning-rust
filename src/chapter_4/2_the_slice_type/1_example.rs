fn main() {
  let mut s = String::from("Hello world");

  let word = first_word(&s); // word will get the value 5

  // We now have a way to find out the index of the end of the first word in the string, but there’s a problem.
  // We’re returning a usize on its own, but it’s only a meaningful number in the context of the &String.
  // In other words, because it’s a separate value from the String, there’s no guarantee that it will still be valid in the future.
  s.clear(); // this empties the String, making it equal to ""

  // word still has the value 5 here, but there's no more string that
  // we could meaningfully use the value 5 with. word is now totally invalid!
}

// the parameter of the function is the immutable reference
fn first_word(s: &String) -> usize {
  let bytes: &[u8] = s.as_bytes(); // returns immutables to array with u8 type

  for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' {
          return i;
      }
  }

  s.len()
}

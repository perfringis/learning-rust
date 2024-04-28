fn main() {
    let mut spaces = "    ";
    println!("spaces {spaces}");

    // can't be assigned when we are using mut keyword
    spaces = spaces.len();
    println!("spaces {spaces}");
}

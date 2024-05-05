fn main() {
    let spaces = "    ";
    println!("spaces {spaces}");

    // in this case shadowing mechanism allows to change variable type
    let spaces = spaces.len();
    println!("spaces {spaces}");
}

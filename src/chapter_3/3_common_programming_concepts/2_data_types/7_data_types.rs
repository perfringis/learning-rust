fn main() {
    // the character type
    // single quoted sign is a character type
    // char type is 4 bytes size
    // char represent unicode scalar value
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("value {c}");
    println!("value {z}");
    println!("value {heart_eyed_cat}");
}

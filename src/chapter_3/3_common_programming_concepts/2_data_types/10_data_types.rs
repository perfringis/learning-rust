fn main() {
    // Accessing Array Elements

    let a = [1, 2, 3, 4, 5];

    // array indexes are starting from 0
    let first = a[0];
    let second = a[1];

    println!("first {first}");
    println!("second {second}");

    // this throws an error
    let value = a[5];
    println!("value {value}");
}

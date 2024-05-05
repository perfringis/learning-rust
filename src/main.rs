fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6; // assigning second time to the immutable variable is impossible
    println!("The value of x is: {x}");
}

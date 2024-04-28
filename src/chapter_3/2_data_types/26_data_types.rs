// Control Flow
// Using if in a let Statement


// after else in the second section(second "arm") the returned value is different that in the first arm
fn main() {
    let condition = true;

    let number = if condition { 5 } else { "six" };

    println!("The value of number is: {number}");
}

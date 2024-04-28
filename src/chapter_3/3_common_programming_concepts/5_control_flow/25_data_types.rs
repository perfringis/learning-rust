// Control Flow
// Using if in a let Statement

// if is an expression is a part of statement
// expression return a value that could be assigned to variable
// both arms it means conditions must return the same type
// like here we have 5 and 6, probably they are type i32 by default.

fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}

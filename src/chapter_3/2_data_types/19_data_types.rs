// Functions with Return Values

fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1; // it's not compiling. Function doesn't return value and x + 1; is the statement
}
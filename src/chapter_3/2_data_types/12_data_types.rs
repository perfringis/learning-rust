// Parameters

// We speak about parameters when we are defining function. In that case we are setting parameters and type for them!
// We speak about arguments when are passing concrete values to the functions!

fn main() {
    println!("Hello, World!");

    another_function(5); // passing argument with value 5
}

fn another_function(x: i32) { // has parameter x
    println!("The value of x is: {x}");
}

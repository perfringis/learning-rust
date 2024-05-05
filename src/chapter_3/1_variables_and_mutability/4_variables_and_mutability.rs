fn main() {
    let x = 5;

    // this mechanism is called shadowing
    // we are using old value from x variable and then we are crating new x variable with new value
    let x = x + 1; // = 6. <- this value only visible in scope of main function.

    {
        // this is inner scope

        // this shadowing
        // we are using old value from outer scope to create new variable x with new value
        let x = x * 2; // = 12 <- this is inner scope only here value is set to 12
        println!("The value of x in the inner scope is: {x}"); // 12
    }

    println!("The value of x is: {x}"); // 6 <- it's function main scope it means value wil be 6.
}

// assigning second time by let x is still immutable value. Shadowing always producing new immutable value.
// using the shadowing mechanism type of variable could be changed
// in case of mutable value the variable should stay with the same type
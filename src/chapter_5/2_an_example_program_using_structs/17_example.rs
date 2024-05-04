// Adding Useful Functionality with Derived Traits

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}

// We can put dbg! around the expression 30 * scale and, because dbg! 
// returns ownership of the expression’s value, the width field will get the same value as if we didn’t have the dbg! call there. 
// We don’t want dbg! to take ownership of rect1, so we use a reference to rect1 in the next call. 

// We can see the first bit of output came from src/main.rs line 10 where we’re debugging the expression 30 * scale, 
// and its resultant value is 60 (the Debug formatting implemented for integers is to print only their value). 
// The dbg! call on line 14 of src/main.rs outputs the value of &rect1, which is the Rectangle struct. 
// This output uses the pretty Debug formatting of the Rectangle type. The dbg! macro can be really helpful when you’re trying to figure out what your code is doing!
// Method Syntax
// Defining Methods

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

// In the signature for area, we use &self instead of rectangle: &Rectangle.
// The &self is actually short for self: &Self. Within an impl block, the type Self is an alias for the type that the impl block is for. 
// Methods must have a parameter named self of type Self for their first parameter, 
// so Rust lets you abbreviate this with only the name self in the first parameter spot. 
// Note that we still need to use the & in front of the self shorthand to indicate that this method borrows the Self instance, 
// just as we did in rectangle: &Rectangle. Methods can take ownership of self, borrow self immutably, as we’ve done here, or borrow self mutably, just as they can any other parameter.

// We chose &self here for the same reason we used &Rectangle in the function version: we don’t want to take ownership, 
// and we just want to read the data in the struct, not write to it. If we wanted to change the instance that we’ve called the method on as part of what the method does, 
// we’d use &mut self as the first parameter. Having a method that takes ownership of the instance by using just self as the first parameter is rare; 
// this technique is usually used when the method transforms self into something else and you want to prevent the caller from using the original instance after the transformation.
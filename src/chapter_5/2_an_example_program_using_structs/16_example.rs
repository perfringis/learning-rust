// Adding Useful Functionality with Derived Traits

#[derive(Debug)] // debug trait
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // println!("rect1 is {:?}", rect1); // output is in one line
    println!("rect1 is {:#?}", rect1); // output is properly formatted
}
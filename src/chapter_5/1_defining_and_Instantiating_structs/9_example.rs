// Using Tuple Structs Without Named Fields to Create Different Types

// tuple structs
// in fast those are different struct types
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0,0,0);
    let origin = Point(0,0,0);
}

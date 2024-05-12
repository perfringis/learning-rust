// Creating a New Vector

fn main() {
    let v: Vec<i32> = Vec::new();

    // Rust conveniently provides the vec! macro, which will create a new vector that holds the values you give it.
    let v = vec![1, 2, 3];
}
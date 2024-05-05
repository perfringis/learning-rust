// Matches Are Exhaustive

fn main() {
    let five = Some(5);

    let six = plus_one(five);

    println!("six: {:?}", six);
}

// match has to handle all cases from enum type
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
}

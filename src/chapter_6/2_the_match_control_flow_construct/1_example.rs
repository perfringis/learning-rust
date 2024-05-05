// The match Control Flow Construct

// Think of a match expression as being like a coin-sorting machine: coins slide down a track with variously sized holes along it,
// and each coin falls through the first hole it encounters that it fits into.
// In the same way, values go through each pattern in a match, and at the first pattern the value “fits,” the value falls
// into the associated code block to be used during execution.

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let value = value_in_cents(Coin::Penny);

    println!("val: {:?}", value)
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
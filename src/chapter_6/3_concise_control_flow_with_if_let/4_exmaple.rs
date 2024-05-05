// Concise Control Flow with if let

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let mut count = 0;
    // let coin = Coin::Quarter(UsState::Alaska);
    let coin2 = Coin::Penny;

    if let Coin::Quarter(state) = coin2 {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
        println!("count: {}", count);
    }
}

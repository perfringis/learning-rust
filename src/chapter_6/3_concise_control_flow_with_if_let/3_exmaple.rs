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

    match coin2 {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => {
            count += 1;

            println!("count: {}", count);
        }
    }
}

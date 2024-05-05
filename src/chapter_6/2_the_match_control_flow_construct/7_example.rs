// Catch-all Patterns and the _ Placeholder

fn main() {
    let val = dice_roll(9);

    println!("val: {:?}", val);
}

fn dice_roll(val: u8) -> u8 {
    match val {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
}

fn add_fancy_hat() -> u8 {
    3
}
fn remove_fancy_hat() -> u8 {
    7
}
fn move_player(num_spaces: u8) -> u8 {
    num_spaces
}

// Catch-all Patterns and the _ Placeholder

fn main() {
    let val = dice_roll(9);

    println!("val: {:?}", val);
}

fn dice_roll(val: u8) {
    match val {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }
}

fn add_fancy_hat() {
}

fn remove_fancy_hat() {
}

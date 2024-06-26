// Concise Control Flow with if let

fn main() {
    let config_max = Some(3u8);
    // pattern match
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    println!("max: {}", config_max);
}

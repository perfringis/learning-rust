// Control Flow
// Repetition with Loops

// Loop Labels to Disambiguate Between Multiple Loops

fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");

        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up; // breaking inner loop and outer loop with label
            }

            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

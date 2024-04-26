fn main() {
    // const value is always inmutable
    // u don't allowed to use mut keyword
    
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of x is: {THREE_HOURS_IN_SECONDS}");
    THREE_HOURS_IN_SECONDS = 6;
    println!("The value of x is: {THREE_HOURS_IN_SECONDS}");
}
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    // println!("{} and {}", &s, &s);
    println!("{} and {}", r1, r2);
    // variables r1 and r2 are drop after println! macro
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

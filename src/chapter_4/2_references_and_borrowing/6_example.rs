fn main() {
    let mut s = String::from("hello");

    {
        // we can create here second mutable reference to variable s.
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
}

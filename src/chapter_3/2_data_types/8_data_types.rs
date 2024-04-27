fn main() {
    // compound types

    // the tuple type
    // tuples has fixed size
    // each position could have different type
    // tuple is considered as single compound

    let tup: (i32, f64, u8) = (500, 6.2, 1); // creating tuple

    let (x,y,z) = tup; // deconstructing into 3 variables

    println!("x {x}");
    println!("y {y}");
    println!("z {z}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    // access element by "." sign
    // access by index
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    println!("five_hundred {five_hundred}");
    println!("six_point_four {six_point_four}");
    println!("one {one}");

    // empty tuple represents the unit type
    // represent empty value
    // let empty = ();

}

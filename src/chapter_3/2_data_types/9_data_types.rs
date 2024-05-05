fn main() {
    // array type
    // values in array must be the same type
    // array has a fixed size

    let a = [1, 2, 3, 4, 5];

    // array will be most useful when we know the exact number of elements
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // specifying type of an array and number of elements
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // will create array 5 times value 3
    let a = [3; 5];
}

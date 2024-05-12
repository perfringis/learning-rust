fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let does_not_exist = &v[100]; // throws panic error
    let does_not_exist = v.get(100); // return None
}

#[derive(Debug)]
enum Message {
    Quit,                       // no data associated
    Move { x: i32, y: i32 },    // has named fields
    Write(String),              // includes single String type
    ChangeColor(i32, i32, i32), // includes 3 i32 values
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

fn main() {
    let write_val = Message::Write(String::from("test"));
    m.call();
}

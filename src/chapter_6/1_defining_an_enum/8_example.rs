#[derive(Debug)]
enum Message {
    Quit,                       // no data associated
    Move { x: i32, y: i32 },    // has named fields
    Write(String),              // includes single String type
    ChangeColor(i32, i32, i32), // includes 3 i32 values
}

impl Message {
    fn call(&self) -> Message {
        // The example returned value
        self::Message::Quit
    }
}

fn main() {
    let m = Message::Write(String::from("test"));
    let value = m.call();

    println!("val: {:?}", value);
}

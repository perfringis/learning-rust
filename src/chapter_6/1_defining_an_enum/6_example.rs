#[derive(Debug)]
enum Message {
    Quit,                       // no data associated
    Move { x: i32, y: i32 },    // has named fields
    Write(String),              // includes single String type
    ChangeColor(i32, i32, i32), // includes 3 i32 values
}

fn main() {
    let quit_val = Message::Quit;
    let move_val = Message::Move { x: 1, y: 2 };
    let write_val = Message::Write(String::from("test"));
    let change_color_val = Message::ChangeColor(-1, -2, -3);

    println!("quit: {:#?}", quit_val);
    println!("move: {:#?}", move_val);
    println!("write: {:#?}", write_val);
    println!("change_color: {:#?}", change_color_val);
}

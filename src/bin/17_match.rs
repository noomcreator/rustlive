/* --- `match` Control Flow ---

🔑 Key Concepts: `match`
- Compares a value against a series of "patterns".
- Must be **exhaustive**: every possible case *must* be handled.
- The `_` (underscore) pattern is a "catch-all" for any other value.
- `match` is an expression that returns a value.
*/

// Enum from the previous lesson
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// A function that use `match` on our `Message` enum
fn process_message(msg: Message) {
    match msg {
        Message::Quit => {
            // do stuff...
            println!("The Quit variant has no data.");
        }
        Message::Move { x, y } => {
            // Deconstructs the struct
            println!("Move to x: {}, y: {}", x, y);
        }
        Message::Write(text) => {
            // Binds the string to the `text` variable
            println!("Text message: {}", text);
        }
        Message::ChangeColor(r, g, b) => {
            // Binds the three i32s
            println!("Change color to R:{}, G:{}, B:{}", r, g, b);
        }
    }
}

fn main() {

    // --- 1. `match` with a complex `enum` ---
    process_message(Message::Quit);
    process_message(Message::Move { x: 10 , y: 20});
    process_message(Message::Write(String::from("hello")));
    process_message(Message::ChangeColor(255, 0, 128));

    // `match` with `_` (underscore) placeholder ---
    // `_` is a "catch-all" pattern that matches anything
    // and does *not* bind the value.

    let some_value = 5;
    match some_value {
        1 => println!("one"),
        2 => println!("two"),
        other => println!("some other number: {}", other), // `other` will catch any other `i32` value and bind it
    }

    // If you don't *need* the value, just use `_`
    let some_other_value = 10;
    match some_other_value {
        1 => println!("one"),
        _ => println!("not one"), // Catch 2, 3, 4, ...
    }
}
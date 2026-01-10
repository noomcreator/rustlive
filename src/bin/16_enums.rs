/* --- Enums (Enumerations) ---

🔑 Key Concepts: Enums
- Defines a type by listing its possible *variants*.
- A value can be *one of* the variants, not all.
- Variants can store data (e.g., `Dog(String)`) or no data (`Quit`).
- Each variant can store different types and amounts of data.
- `Option<T>` is a very common enum: `Some(T)` or `None` (Rust's "null").
- `#[derive(Debug)]` lets us print the enum with `{:?}`.
*/

// A simple enum with no data
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

// An enum where variants store data (here, a String).
#[derive(Debug)]
#[allow(dead_code)]
enum Animal {
    Dog(String),
    Cat(String),
    Fish(String),
}

// A more complex enum with different data for each variant.
#[derive(Debug)]
#[allow(dead_code)]
enum Message {
    Quit,                        // No data
    Move { x: i32, y: i32 },     // An anonymous struct
    Write(String),               // A String
    ChangeColor(i32, i32, i32),  // Three i32s
}

fn main() {
    // --- Creating Instances ---
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("IP versions: {:?} and {:?}", four, six);

    // --- Creating Instances Width Data ---
    let dog = Animal::Dog(String::from("Rex"));
    let cat = Animal::Cat(String::from("Whiskers"));
    println!("Animals: {:?} and {:?}", dog, cat);

    // --- Creating Instances of the complex enum ---
    let m1 = Message::Quit;
    let m2 = Message::Move { x: 10, y: 10 };
    let m2b = Message::Move { x: 11, y: 21 };
    let m3 = Message::Write(String::from("hello rust"));
    let m4 = Message::ChangeColor(255, 0, 128);
    println!("\nMessages: {:?}, {:?}, {:?}, {:?}, {:?}", m1, m2, m2b, m3, m4);

    // `Option` Enum (A very important enum) ---
    // Rust doesn't have `null`. Instead, it has `Option<T>`.
    // enum Option<T> {
    //     Some(T), // Contains a value of type T
    //     None, // Has no value
    // }

    let some_number = Some(5);
    let no_number: Option<i32> = None;

    println!("\nAn optional value: {:?}", some_number);
    println!("A 'null' (None) value: {:?}", no_number);

    // NOTE: To *use* the data inside an enum (e.g., get the `5`
    // out of `some_number`), we need a control flow construct
    // called `match`, which we'll cover later.
}
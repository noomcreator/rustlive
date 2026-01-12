/* Collections: `String`

🔑 Key Concepts: `String`
- A growable, mutable, owned, UTF-8 encoded string.
- stored on the heap for dynamic sizing.
- Different from `&str` (string slice), which is an immutable *view* of a string.
- Create with `String::new()`, `String::from()`, or `.to_string()`.
- Append with `push_str()` (to add a `&str`) or `push()` (to add a `char`).
- Concatenate with `+` (moves ownership of the first string).
*/

fn main() {
    println!("--- 1. Creating Strings ---");

    // Create a new, empty string.
    let s = String::new();
    println!("Empty string: '{}'", s);

    // Convert a string literal (&str) to a String
    let data = "initial contents";
    let s_from_data = data.to_string();
    println!("From .to_string(): '{}'", s_from_data);

    // Create a String directly from a string literal
    let s_from_literal = String::from("also initial contents");
    println!("From String::from(): '{}'", s_from_literal);

    println!("\n--- 2. Updating a String ---");
    // We can append to a String (it must be `mut`)

    let mut s = String::from("foo");
    s.push_str("bar"); // `push_str` appends a string slice (&str)
    println!("After push_str: '{}'", s);

    s.push('!'); // `push` appends a single char
    println!("After push: '{}'", s);

    println!("\n--- 3. Concatenation with `+` ---");
    // This uses the `+` operator, which moves ownership.
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");

    // The signature is `fn add(self, s: &str) -> String`
    // `s1` is moved, `s2` is borrowed (coerced from `String` to `&str`)
    let s3 = s1 + &s2;

    println!("Concatenated string: {}", s3);

    // This line would fail! `s1` was moved and is no longer valid.
    // println!("{}", s1);
    println!("(Note: `s1` was moved and can no longer be used)")
}
/* --- Structs (Structures) ---

🔑 Key Concepts: Struct
- A custom data type that groups related values (fields).
- Define with `struct` and a name (e.g., `struct User`).
- Each field in the struct has a name and a type.
- Create an "instance" using `let name = User { ... }`.
- Access fields with dot notation (`instance.field`).
*/
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// A "Tuple Struct" with unnamed fields, accessed by index.
struct Color(i32, i32, i32); // R, G, B

fn main() {
    // Using "Field Init Shorthand" for email and username.
    let email = String::from("user@example.com");
    let username = String::from("someuser123");
    
    let mut user1 = User {
        email, // Shorthand for `email: email`
        username, // Shorthand for `username: username`
        active: true,
        sign_in_count: 1,
    };

    // We access fields using dot notation.
    println!("Username is: {}", user1.username);

    // If the instance is mutable, we can change a value.
    user1.username = String::from("anotheruser456");
    println!("New username is: {}", user1.username);
    
    println!("\n--- 3. Struct Update Syntax ---");
    // We can create a new instance using some fields from an old instance.
    let user2 = User {
        email: String::from("user2@example.com"),
        username: String::from("user2abc"),
        ..user1 // ".." means "use the remaining fields from user1"
    };
    // Note: `user1.active` and `user1.sign_in_count` (which are `Copy`)
    // were copied. `user1.email` and `user1.username` were *not* used.
    println!("User 2's username: {}", user2.username);
    println!("User 1's sign-in count is still available: {}", user1.sign_in_count);

    println!("\n--- 4. Creating an Instance (Tuple Struct) ---");
    let black = Color(10, 20, 30); //We create and access them using tuple syntax.
    
    // Access by index with dot notation.
    println!("The color's Red value is: {}", black.0);
    println!("The color's Green value is: {}", black.1);
    
}
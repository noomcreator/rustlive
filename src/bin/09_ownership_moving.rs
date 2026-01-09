/* --- Ownership & Moving ---
    
🔑 Key Concepts:
- Each value has *one* owner.
- When the owner goes out of scope, the value is dropped (memory freed).
- Simple "stack" values (like `i32`) are *copied*.
- Assigning a "heap" value (like `String`) *moves* ownership.
*/

fn main() {

    // --- Example 1: Stack Data (Copy) ---
    let x = 10;
    let y = x;
    println!("x = {}", y);
    println!("y = {}", x);

    // --- Example 2: Heap Data (Move) ---
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s2 = {}", s1);
    println!("s1 = {}", s2);

    // --- Example 3: Function Ownership Transfer ---
    // Passing a String to a function MOVES ownership into the function.
    let s3 = String::from("Rust");
    calculate_length(s3.clone()); // Ownership of s3 MOVES into the 'consume_string' function.
    println!("s3 = {}", s3);
}

#[allow(dead_code)]
fn calculate_length(s: String) -> usize {
    let len = s.len();
    println!("Consumed string length: {}", len);
    len // return len;
}
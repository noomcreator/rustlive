/* Variables 

🔑 Key Concepts:
- Declared using `let`.
- Immutable by default; they cannot be changed after being assigned.
- Use the `mut` keyword to make a variable mutable.
- Rust can usually infer the type (e.g., let x = 5; is inferred as i32).
- Variables exist only within their scope (the {...} block they are in).
*/

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);
}
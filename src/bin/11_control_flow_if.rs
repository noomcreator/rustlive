/* --- `if` Expressions ---

🔑 Key Concepts: 
- Used for conditional branching.
- Syntax: `if condition { ... } else if condition2 { ... } else { ... }`.
- Conditions must evaluate to a `bool`.
- `if` is an expression; it can return a value.
- All branches must return the same type when used as an expression.
- Can be used in `let` statements to assign values conditionally.
*/
fn main() {
    let number = 7;

    if number < 10 {
        println!("The number is small.");
    } else if number > 50 {
        println!("The number is large.");
    } else {
        println!("The number is medium-sized.");
    }

    // `if` is an expression, so you can use it in a `let` statement.
    let condition = false;

    // All branches must return the same type (here, an i32).
    let x = if condition {
        5
    } else {
        6
    };
    println!("The value of x from the `if` expression is: {}", x);
}
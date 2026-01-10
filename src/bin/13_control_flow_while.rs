/* --- `while` Loops ---

🔑 Key Concepts: `while` Loops
- Repeats code while a condition is true.
- Condition is checked before each iteration.
- Use `break` to exit the loop early.
*/

fn main() {
    let mut num = 3;

    while num != 0 {
        println!("{}!", num);
        num -= 1; // This change eventually makes the condition `false`.
    }

    println!("LIFTOFF!!!");
}
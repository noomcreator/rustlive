/* --- `for` Loops ---

🔑 Key Concepts: `for` Loops
- Used to iterate over collections or ranges.
- Syntax: `for variable in collection { ... }`.
- Automatically handles the iteration.
- Avoids common pitfalls of other loop types (like infinite loops).
*/

fn main() {
    // `for` loop with a collection (array) ---
    let a = [10, 20, 30, 40, 50];
    println!("--- 4. `for` loop (array) Example ---");

    // `.iter()` creates an "iterator" that lets the loop "borrow" each element.
    for element in a.iter() {
        println!("The value is: {}", element);
    }

    // --- `for` loop with a Range ---
    println!("\n--- `for` loop (range) Example ---");

    // `(1..4)` is a range from 1 to (not including) 4. `.rev()` reverse the range.
    for number in (1..4).rev() {
        println!("{}!", number);
    }

    println!("LIFTOFF AGAIN!!!");
}
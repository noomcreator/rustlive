/* --- `loop` (Infinite Loop) ---

🔑 Key Concepts: `loop`
- Creates an infinite loop.
- Use `break` to exit the loop.
- Can return a value when breaking.
*/

fn main() {
    let mut counter = 0;

    let result: i32 = loop {
        counter += 1;
        println!("Looping... count is {}", counter);

        if counter == 5 {
            // We can use `break` to stop the loop.
            break counter * 2; // We can return a value from the loop after `break`.
        }
    };

    println!("`loop` finished, result is: {}", result);
}
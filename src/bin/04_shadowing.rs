/* --- Shadowing ---

🔑 Key Concepts:
- Declares a new variable with the same name.
- Hides (shadows) the old variable.
- Is not mutation (it's a new let binding).
- Allows changing the type (mut can't do that).
- Follows standard scope rules.
*/

fn main() {
    let z = 5; //5
    println!("The value of z is: {}", z);

    // This new `let` binding shadows the previous `z`.
    let z = z + 1; //6
    println!("z is shadowed, the new value is: {}", z);

    {
        let z = z * 2; //12
        println!("In the inner scope, z is: {}", z);
    }

    // The inner shadow is gone; `z` refers to the previous binding.
    println!("Back in the outer scope, z is {}", z); //6

    println!("\n--- Shadowing vs. Mutability ---");

    // Shadowing can chage the type of the binding.
    let spaces: &str = "   "; // `spaces` is type: &str
    println!("'{}' is a string", spaces);
    let spaces: usize = spaces.len(); // `spaces` is now type: usize
    println!("...and now {} is a number (its length).", spaces);
    
}
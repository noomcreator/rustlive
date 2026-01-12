/* Collections: `HashMap<K, V>` (Hash Map)

🔑 Key Concepts: `HashMap<K, V>`
- Stores a mapping of keys (K) to values (V) (like a dictionary).
- stored on the heap for dynamic sizing.
- Keys and values can be of different types, but all keys must be the same type,
- Create with `HashMap::new()`.
- Add key-value pairs with `.insert()`.
- Access values with `.get()`, which returns an `Option<&V>`.
- Iterating gives you `(key, value)` pairs.
- Will "overwrite" a value if you insert a new value with the same key.
*/

// We need to import HashMap to use it, as it's not in the prelude.
use std::collections::HashMap;

fn main() {
    // Create a new, empty HashMap.
    let mut scores = HashMap::new();

    println!("--- 1. Inserting the keys and values ---");
    // Insert key-value pairs
    // We use String keys and i32 values: HashMap<String, i32>
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("The scores map: {:?}", scores); // `{:?}` debug prints

    println!("\n--- 2. Accessing a value ---");
    let team_name = String::from("Blue");

    // `.get()` takes a reference to the key (`&team_name`)
    // It returns an `Option<&i32>`
    let score = scores.get(&team_name);

    // `match` to handle the `Option`
    match score {
        Some(s) => println!("The score for Blue is: {}", s),
        None => println!("Team not found."),
    }

    // .get() is safer for keys that don't exist
    match scores.get("Red") {
        Some(s) => println!("The score for Red is: {}", s),
        None => println!("Team Red not found."),
    }

    println!("\n--- 3. Iterating over a Hash Map ---");
    // Iterating over a hash map gives (key, value) pairs.
    // The order is not guaranteed!
    println!("Iterating over scores:");
    for (key, value) in &scores { // `&scores` borrows the map
        println!("{}: {}", key, value);
    }

    println!("\n--- 4. Overwriting a value ---");
    // If we insert a key that already exists, it overwrites the value.
    scores.insert(String::from("Blue"), 25);
    println!("After update, scores map: {:?}", scores);

}
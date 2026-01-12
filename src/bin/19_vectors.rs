/* Collections: `Vec<T>` (Vector)

🔑 Key Concepts: 
- A Vector is a growable, resizable array.
- Vectors are stored on the Heap for dynamic sizing.
- Vectors can only store values of the same type (e.g., `Vec<i32>`).
- Created with `Vec::new()` (empty) or `vec![]`.
- Add elements with `.push()`.
- Access with `[]` (panics on bad index) or `.get()` (returns `Option`).
- Iterate immutably with `for i in &v`.
- Iterate mutably with `for i in &mut v` (use `*i` to change value).
*/
fn main() {
    println!("--- 1. Creating Vectors ---");
    // Create a new, empty vector. We must tell Rust the type, since it's empty.
    let mut v: Vec<i32> = Vec::new();

    // Add elements to the vector using `push`
    v.push(10);
    v.push(20);
    v.push(30);

    println!("The vector `v` is: {:?}", v); // `{:?}` is a debug print format

    // We can also create a vector with initial values using the `vec!` marcro
    // Rust infers the type (`Vec<i32>`) from the values.
    let v2 = vec![1, 2, 3];
    println!("Vector `v2` is: {:?}", v2);

    println!("\n--- 2. Accessing elements ---");
    // You can use `[]` or `.get()`
    // Using `[]` (panics if index out of bounds)
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    // Using `.get()` returns a safer `Option`
    match v.get(2) {
        Some(third_val) => println!("Got the third element: {}", third_val),
        None => println!("There is no third element."),
    }

    // .get() is safer for out-of-bounds
    if v.get(100) == None {
        println!("Index 100 is safely handled as None.");
    }
}
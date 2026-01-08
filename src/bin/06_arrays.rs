/* --- Primitive Types: Arrays ---

🔑 Key Concepts: 
- Groups multiple values of the same type.
- Have fixed length (size cannot change).
- Type is annotated as `[type; length]`, e.g., `[i32; 5]`.
- Access elements by index with `[]` (e.g., `arr[0]`).
- Data is allocated on the stack.
*/
fn main() {
    
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // Initialize an array to contian the same value `[value: length]`.
    let b = [3, 5]; // Same as `let b = [3, 3, 3, 3, 3];`

    // We access array elements by index (starting from 0).
    let first = a[0];
    let second = a[1];
    println!("Array 'a' elements: first = {}, second = {}", first, second);
    println!("Array 'b' (all 3s): first element is {}", b[0]);

    // NOTE: Accessing an out-of-bounds index (e.g., `a[10]`)
    // will cause your program to *panic* (a runtime error).
    let panic = a[10];
}
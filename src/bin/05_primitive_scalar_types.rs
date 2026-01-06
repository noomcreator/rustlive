/* --- Primitive Types: Scalar ---

🔑 Key Concepts: 
- They Represent a single value.
- 4 main types: Integers, Floats, Booleans, Characters.
- Statically typed: types must be known at compile time.
- Types can be annotated (e.g. `: i32`) or *inferred*.
*/

fn main() {
    println!("--- 1. Integers ---");
    // Signed (i) Unsigned (u), in 8, 16, 32, 128 bit sizes.
    // `isize` and `usize` depend on the computer's architecture.
    let a: i32 = -10;       // `i32` is the default integer type.
    let b: u64 = 100_000;   // Can use `_` as a visual separator.
    println!("Integers: a = {}, b = {}", a, b);

    println!("\n--- 2. Floating-Point Numbers ---");
    // f32 (single precision) and f64 (double precision)
    let x = 2.0; // `f64` is the default float type.
    let y: f32 = 3.14;
    println!("Floats: x = {}, y = {}", x, y);

    println!("\n--- 3. Booleans ---");
    // `bool` type, can be `true` or `false`
    let is_rust_fun = true;
    let is_learning_done = false;
    println!("Is_rust_fun = {}, is_learning_done = {}", is_rust_fun, is_learning_done);

    println!("\n--- 4. Characters ---");
    // `char` type, for a single Unicode scalar value (single quotes).
    let c = 'z';
    let z = 'ℤ'; // Supports Unicode
    let heart_eyed_cat = '😻'; // Supports Emojis
    println!("Chars: c = {}, z = {}, emoji = {}", c, z, heart_eyed_cat);
}
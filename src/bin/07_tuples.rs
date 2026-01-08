/* --- Primitive Types: Tuples ---

🔑 Key Concepts: 
- Groups multiple values of different types.
- Have a fixed length (size cannot change).
- Access elements by index with a `.` (e.g., `tup.0`, `tup.1`).
- Can be "destructured" to get all values at once.
*/
fn main() {
    let tup: (&str, f64, bool) = ("Hello, tuples!", 6.4, true);

    // We can "destructure" a tuple to get its individual values
    let (x, y, z) = tup;
    println!("Destructured: x = {}, y = {}, z = {}", x, y, z);

    // We can access tuple elements by index (starting from 0).
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("Accessed by index: {}, {}, {}", five_hundred, six_point_four, one);
}
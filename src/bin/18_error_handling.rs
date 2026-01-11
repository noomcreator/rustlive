/* Error Handling (Option & Result, unwrap & expect)

🔑 Key Concepts: 
- Rust has no "NULL" or "exceptions".
- It uses two enums: `Option<T>` and `Result<T, E>`.
- `Option<T>`: For values that might be absent (`null`): 
    - Some(T): A value is present, 
    - None: A value is absent.
- `Result<T, E>`: For operations that can fail: 
    - Ok(T):The operation succeeded, with value `T`
    - Err(E): The operation failed, with error `E`.
- `match` is the standard way to handle these enums.
- `.unwrap()` is a shortcut that gets the value or **panics** (crashes).
- `.expect("message")` is like unwrap, but with a custom panic message.
- The `?` operator can propagate errors easily in functions that return `Result`.
*/

// This function "might" fail, so it returns a `Result`.
fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {  
        Err(String::from("Cannot divide by zero!")) // Failure: return an Err variant
    } else {
        Ok(numerator / denominator) // Success: return an Ok variant
    }
}

fn main() {
    println!("--- 1. Handling `Result` with `match` ---");
    
    let good_result = divide(10.0, 2.0);
    let bad_result = divide(10.0, 0.0);

    // We use `match` to handle both `Ok` and `Err` cases.
    match good_result {
        Ok(value) => println!("Good result: {}", value),
        Err(error) => println!("Error: {}", error),
    }

    match bad_result {
        Ok(value) => println!("Good result: {}", value),
        Err(error) => println!("Error: {}", error),
    }
    
    // Use of `match` on `Option`
    println!("\n--- 2. Handling `Option` with `match` ---");
    let some_text = Some(String::from("This is some text"));
    let no_text: Option<String> = None;

    match &some_text { 
        Some(text) => println!("We got some text: {}", text),
        None => println!("We got nothing."),
    }
    
    match &no_text {
        Some(text) => println!("We got some text: {}", text),
        None => println!("We got nothing."),
    }
    
    println!("\n--- 3. The `.unwrap()` shortcut (Use with caution!) ---");
    // `.unwrap()`:
    // - If `Ok` or `Some`, it gives you the inner value.
    // - If `Err` or `None`, your program will **panic** (crash).

    // This is OK, because we know it's `Ok(5.0)`
    let good_value = divide(10.0, 2.0).unwrap();
    println!("Unwrapped good value: {}", good_value);
    
    //expect
    let some_option = Some("Hello, world!");
    let unwrapped_value = some_option.expect(">>>Expected a value, found None!");
    println!("Unwrapped value using expect: {}", unwrapped_value);
    let none_option: Option<&str> = None;
    let _unwrapped_none = none_option.expect(">>>Expected a value, found None!");

    // the ? operator (brief mention)
    fn try_divide(numerator: f64, denominator: f64) -> Result<f64, String> {
        let result = divide(numerator, denominator)?; // If Err, returns early
        Ok(result)
    }
    
}
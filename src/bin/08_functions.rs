/*  --- Functions ---

🔑 Key Concepts
- Defined using the `fn` keyword.:
- Have a name and can take parameters.
- Parameters must have their types declared.
- Can return a value; return type is declared after `->`.
- Use `return` for early returns; otherwise, the last expression is returned.
- Distinguish between statements (do not return values) and expressions (return values).
- Functions must be defined before they are called.
*/

// function with no parameters and no return value.
fn another_function() {
    println!("Hello from another_function!");
}

// function that takes one parameter.
fn print_value(x: i32) {
    println!("The value passed to print_value is: {}", x);
}

// function with multiple parameters.
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

// Function with Return Values - We declare the return type after an arrow `->`.
fn add_five(num: i32) -> i32 {
    // If there's now `;`, The last line of the function is the return value.
    // this is an expression, and its values is returned.
    num + 5
    // We could have written `return num + 5;`
    // The `return` keyword is usually only used for "early returns".
}

// The `main` Function is the entry point. It takes no parameters and returns nothing.
fn main() {
    println!("Hello from the main function!");

    // Calling Functions
    another_function();
    print_value(12);
    print_labeled_measurement(5, 'h');

    // Statements vs. Expressions
    // Statements are instructions that perform some action and do not return a value.
    // Expression evaluate to a resulting value.

    // `let x = 6;` is a STATEMENT. It doesn't return a value.
    // You can't do `let y = (let x = 6);`

    // `5 + 6` is an EXPRESSION. It evaluates to 11.
    // A function call is an expression.
    let sum = add_five(10);
    println!("10 + 5 = {}", sum);
}

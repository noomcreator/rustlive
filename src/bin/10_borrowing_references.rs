/*  --- Borrowing & References

    🔑 Key Points:
    1. "Borrowing" = creating a *reference* (`&`) to a value, not taking ownership.
    2. Immutable references (`&T`) let you read data. You can have *many*.
    3. Mutable references (`&mut T`) let you change data. You can only have *one*.
    4. You *cannot* have mutable and immutable references at the same time.
    5. The compiler guarantees references are always valid (no dangling).
*/

#[allow(dead_code)]
fn calculate_length(s: &String) -> usize {
    let len = s.len();
    println!("Consumed string length: {}", len);
    len
} 

fn main() {
    // --- Example 1. Immutable Borrows (`&`) ---
    // We "borrow" s1 immutably. We don't take ownership.
    let s1: String = String::from("hello");
    let len = calculate_length(&s1); //&s1 passes a reference to s1

    println!("The length is {}", len);
    println!("s1 is still usable: {}", s1);
    
    /*
    // --- Example 2. Mutable Borrows (`&mut`) ---
    // We "borrow" s2 mutably, to change it.
    let mut s2 = String::from("hello");
    println!("\nBefore change: {}", s2);
    change(&mut s2); // Pass a mutable reference 
    println!("After change: {}", s2);
    
    // --- 3. The Rules of Borrows (CRITICAL) ---
    // The compiler enforces these rules to prevent data races.
    let mut s3 = String::from("rule demo");

    // You can have *many* immutable references.
    let r1 = &s3;
    let r2 = &s3;
    println!("\nMultiple immutable borrows are OK: {} & {}", r1, r2);
    
    // As soon as r1 and r2 are no longer used, we can get a mutable ref.
    
    // You can have *one* mutable reference.
    let r_mut1 = &mut s3;
    println!("One mutable borrow is OK: {}", r_mut1);
    
    // But you *cannot* have two mutable references.
    let r_mut2 = &mut s3; // ERROR: cannot borrow `s3` as mutable more than once
    println!("{} and {}", r_mut1, r_mut2);
    
    // And you *cannot* mix immutable and mutable.
    let r_mut_again = &mut s3;
    let r_immut_again = &s3; // ERROR: cannot borrow `s3` as immutable
                               // because it is already borrowed as mutable
    println!("{} and {}", r_mut_again, r_immut_again);
    
    // --- 4. Dangling References (prevented by compiler) ---
    // Rust ensures all references point to valid data.
    let reference_to_nothing = dangle();
    */
}

#[allow(dead_code)]
// This function "mutably borrows" a String
fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

/*
#[allow(dead_code)]
fn dangle() -> &String {
    let s = String::from("dangle");
    &s 
}
// ERROR: `s` goes out of scope here, so the reference would be dangling.
// The compiler prevents this.
*/
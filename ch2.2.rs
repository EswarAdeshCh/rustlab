fn main() {
    // Borrowing a value using reference
    let s1 = String::from("Hello, Rust!");
    let s2 = &s1; // Borrowing a reference to s1

    println!("s1: {}", s1);
    println!("s2 (borrowed reference): {}", s2);

    // Dereferencing a reference using the * operator
    let num = 10;
    let ref_num = &num; // Borrow a reference to num

    println!("Reference to num: {}", ref_num);
    println!("Dereferenced value: {}", *ref_num); // Dereferencing

    // Mutable borrowing and changing the original value
    let mut x = 42;
    let y = &mut x; // Borrowing mutably
    *y += 10; // Dereferencing and modifying the value

    println!("Modified x: {}", x);
}

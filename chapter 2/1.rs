fn main() {
    // Implicit casting (Not allowed in Rust)
    // let a: i32 = 10;
    // let b: f64 = a; // Error! Can't implicitly cast i32 to f64

    // Explicit type casting (using `as` keyword)
    let a: i32 = 10;
    let b: f64 = a as f64;

    println!("Integer value: {}", a);
    println!("Cast to float: {}", b);

    // Casting from float to integer
    let x: f64 = 3.14;
    let y: i32 = x as i32;

    println!("Float value: {}", x);
    println!("Cast to integer: {}", y);
}

/*write a recursive function fibonacci that takes a positive integer number n as a parameter
and returns the nth Fibonacci term in that range*/
fn fibonacci(n: i32) -> i32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn main() {
    let n = 7;
    println!("Fibonacci term {} is {}", n, fibonacci(n)); // Output: 13
}

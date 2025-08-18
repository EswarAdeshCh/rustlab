//1. Write a program to Find The Factorial using functions. 
fn factorial(n: i32) -> i32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn main() {
    let number = 5; 
    let result = factorial(number);
    println!("The factorial of {} is {}", number, result);
}

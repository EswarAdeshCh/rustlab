/*Write a function test_divisibility_by_3_4 which will check whether a given integer
number is divisible by 3 or 4.
a. If the number is divisible by both return 0
b. If the number is divisible by 3 only return 1
c. If the number is divisible by 4 only return 2
d. If the number is not divisible by both, return -1
*/
fn test_divisibility_by_3_4(number: i32) -> i32 {
    if number % 3 == 0 && number % 4 == 0 {
        0
    } else if number % 3 == 0 {
        1
    } else if number % 4 == 0 {
        2
    } else {
        -1
    }
}

fn main() {
    println!("{}", test_divisibility_by_3_4(12)); // 0
    println!("{}", test_divisibility_by_3_4(9));  // 1
    println!("{}", test_divisibility_by_3_4(8));  // 2
    println!("{}", test_divisibility_by_3_4(7));  // -1
}

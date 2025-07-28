//6. Write a program to Multiplication Table using Loop Labels
fn main() {
    'outer: for i in 1..=10 {
        for j in 1..=10 {
            println!("{} * {} = {}", i, j, i * j);

            if j == 5 {
                break 'outer; // Break out of outer loop when j reaches 5
            }
        }
    }
}

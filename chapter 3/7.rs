//7. Write a program to Count Iterations of a Loop Until a Condition
fn main() {
    let mut count = 0;

    loop {
        count += 1;
        if count == 5 {
            break;
        }
        println!("Iteration: {}", count);
    }

    println!("Loop stopped after {} iterations.", count);
}

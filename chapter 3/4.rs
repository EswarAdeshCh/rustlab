//4. Write a program to Match a pattern using If Let Expression.
fn main() {
    let some_value = Some(10);

    if let Some(x) = some_value {
        println!("The value is: {}", x);
    } else {
        println!("No value found");
    }
}

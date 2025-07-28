//9. Write a program to print the values in a collection using iter() method
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    for num in numbers.iter() {
        println!("{}", num);
    }
}

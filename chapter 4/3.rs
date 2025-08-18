// Write a program to demonstrate the Pass by Value and Pass by Reference
fn pass_by_value(mut num: i32) {
    num += 10;
    println!("Inside pass_by_value: num = {}", num);
}

fn pass_by_reference(num: &mut i32) {
    *num += 10;
    println!("Inside pass_by_reference: num = {}", num);
}

fn main() {
    let x = 5;

    println!("Before pass_by_value: x = {}", x);
    pass_by_value(x);
    println!("After pass_by_value: x = {}", x); // x unchanged because it's pass by value

    let mut y = 5;

    println!("Before pass_by_reference: y = {}", y);
    pass_by_reference(&mut y);
    println!("After pass_by_reference: y = {}", y); // y changed because it's pass by reference
}
  

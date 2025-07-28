/*8. Write a program to Print the following patterns
&
&&
&&&
&&&&
&&&&&*/
fn main() {
    let mut pattern = String::new();

    for i in 1..=5 {
        pattern.push('&');
        println!("{}", pattern);
    }
}

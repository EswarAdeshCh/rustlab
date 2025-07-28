//3. Write a program to make a calculator using Match Expression
fn main() {
    let num1 = 10;
    let num2 = 5;
    let operator = "+";

    let result = match operator {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => num1 / num2,
        _ => {
            println!("Invalid operator");
            return;
        }
    };

    println!("The result is: {}", result);
}

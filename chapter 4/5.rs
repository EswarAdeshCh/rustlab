//Write a function arr_square() that returns the Array of Squares
fn arr_square(arr: &[i32; 5]) -> [i32; 5] {
    let mut squares = [0; 5];
    for i in 0..5 {
        squares[i] = arr[i] * arr[i];
    }
    squares
}

fn main() {
    let numbers = [1, 2, 3, 4, 5];
    let squares = arr_square(&numbers);
    println!("{:?}", squares); // Output: [1, 4, 9, 16, 25]
}


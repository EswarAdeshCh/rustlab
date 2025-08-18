/*Write a function calculate_area_perimeter() that takes x and y( length and width of a
rectangle) as a parameter to the function and returns a tuple (area, perimeter).*/
fn calculate_area_perimeter(x: u32, y: u32) -> (u32, u32) {
    let area = x * y;
    let perimeter = 2 * (x + y);
    (area, perimeter)
}

fn main() {
    let (area, perimeter) = calculate_area_perimeter(5, 3);
    println!("Area: {}, Perimeter: {}", area, perimeter);
}

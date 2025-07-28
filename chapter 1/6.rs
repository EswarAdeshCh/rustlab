fn main() {
    let mut arr=Vec::new();
    for i in 0..=10 {
        if i%2==0 {
            arr.push(i);
            println!("{}",i);
        }
    }
}

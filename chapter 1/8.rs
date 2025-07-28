fn main(){
    let arr=[0,1,2,3,4,5,6,7,8,9];
    let a=&arr[1..3];
    println!("{:?}",a);
    let b=&arr[..5];
    println!("{:?}",b);
    let c=&arr[5..];
    println!("{:?}",c);
    let d=&arr[..];
    println!("{:?}",d);
}

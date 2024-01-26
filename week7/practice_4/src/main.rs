use std::io;
fn add(a: i32, b: i32) {
    let sum = a + b;

    println!("Sum of A and B = {}", sum);
}

fn main(){
    let mut input1 = String::new();
    println!("Enter input foir parameter A");
    io::stdin().read_line(&mut input1).expect("incorrect input");
    let a:i32 = input1.trim().parse().expect("not valid");

    let mut input2 = String::new();
    println!("Enter input foir parameter B");
    io::stdin().read_line(&mut input2).expect("incorrect input");
    let b:i32 = input2.trim().parse().expect("not valid");

    // call a function
    add(a, b);
}
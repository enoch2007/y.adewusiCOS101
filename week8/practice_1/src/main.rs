fn main() {
    //using Vec::new()
    let v : Vec<i64> = Vec::new();

    //printing the size of vector
    println!("\nThe length of Vec::new is: {}",v.len());

    //using macro
    let v = vec!["Jhae", "Jp", "Play_taku", "basil", "kareem"];

    //printing the size of vector
    println!("\nThe length of vec macro is: {}", v.len());
}
fn main() {

    let v = vec!['C','O','M','P','T','E','R'];

    let mut input1 = String::new();

    println!("Enter an index value btw (0 - 7)");
    std::io::stdin().read_line(&mut input1).expect("failed to read input");
    //Index is the non negative value  which is smaller than the size of the vector
    let index:usize = input1.trim().parse().expect("invalid input");

    //getting value at given index value
    let ch:char = v[index];

    println!("{} is character for index [{}]\n",ch, index);
}
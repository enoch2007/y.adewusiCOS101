fn main() {
    //create an empty vector "city"
    let mut city: Vec<String> = Vec::new();
    //print City vector
    println!("The city vector has element {}",city.len());
    // Push new elements into
    let mut input1 = String::new();
    println!("How many cities ndo you want to enter??");
    std::io::stdin().read_line(&mut input1).expect("invalid");
    let number_of_cities:i32 = input1.trim().parse().expect("invalid");
    for count in 0..number_of_cities{
        let mut input2 = String::new();
        println!("Enter name of city:");
        std::io::stdin().read_line(&mut input2).expect("invalid");
        let new_city:String = input2.trim().parse().expect("invalid");
        city.push(new_city);
    }

    print!("Your preffered cities are:\n");
    let mut count=1;
    //loop to iterate elements in vector
       for i in city{
        // iterrating through i on the vector
        println!("{} {}",count,i );
        count+=1;
       }
}

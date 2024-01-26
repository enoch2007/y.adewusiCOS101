struct Employee{
    Name:String,
    company:String,
    age:u32
}

fn main() {
    let emp1 = Employee{
        company:String::from("Enrst & Young"),
        Name:String::from("Ebibiong Jessica"),
        age:25
    };
    println!("Name = {} \n",emp1.Name);
    println!("company = {} \n",emp1.company);
    println!("Age = {}",emp1.age);
}

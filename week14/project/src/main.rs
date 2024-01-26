use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();

    println!("What is your position?\nInput position:");
    io::stdin().read_line(&mut input).expect("not valid");
    let post = input.trim();

        if post == "Administrator"{
        let mut file = std::fs::File::open("globacom_database.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);
        }
        else{
        print!("Data not found");
        }
        if post == "Project Manager"{
        let mut file = std::fs::File::open("project_table.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);
        }
        else{
        print!("Data not found");
        }
        if post == "Employee"{
        let mut file = std::fs::File::open("staff_table.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        pr
        else{
        print!("Data not found");
        }int!("{}", contents);
        }
        if post == "Customer"{
        let mut file = std::fs::File::open("customer_table.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);
        }
        else{
        print!("Data not found");
        }
        if post == "Vendor"{
        let mut file = std::fs::File::open("dataplan_table.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);
        }
        else{
        print!("Data not found");
        }
}

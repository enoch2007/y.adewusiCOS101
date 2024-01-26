use std::io;
use std::io::Write;

fn main(){
    println!("PAU Student Management Information System"); 

    let mut input = String::new();
    let mut file = std::fs::File::create("PAU SMIS.txt").expect("unable to open file");
    file.write_all("Name\t Matric number\t Department\t Level\t".as_bytes()).expect("unable to do so");
    file.write_all("\n".as_bytes());

    println!("How many students are to be registered??");
    io::stdin().read_line(&mut input).expect("invalid");
    let number_of_students:usize = input.trim().parse().expect("invalid");

    for i in 0..number_of_students{
        let mut student_name = String::new();
        let mut matric_number = String::new();
        let mut departement = String::new();
        let mut level = String::new();
        let mut student_details = Vec::new();

        println!("Student name??");
        io::stdin().read_line(&mut student_name).expect("invalid");
        let student_name = student_name.trim();
        student_details.push(student_name.to_string());

        println!("Matric number??");
        io::stdin().read_line(&mut matric_number).expect("invalid");
        let matric_number = matric_number.trim();
        student_details.push(matric_number.to_string());

        println!("Department??");
        io::stdin().read_line(&mut departement).expect("invalid");
        let department = departement.trim();
        student_details.push(department.to_string());

        println!("what level??");
        io::stdin().read_line(&mut level).expect("invalid");
        let level = level.trim();
        student_details.push(level.to_string());

        for i in 0..student_details.len(){
            file.write_all(student_details[i].as_bytes()).expect("unable to do so");
            file.write_all("\t".as_bytes()).expect("unable to do so");
        }
        file.write_all("\n".as_bytes()).expect("unable to do so");

        println!("you have been registered");
    }
    
}
use std::io;
use std::io::Write;

fn main(){
    println!("PAU Student Management Information System"); 

    let mut input = String::new();
    let mut file = std::fs::File::create("File of convicted Ministers.txt").expect("unable to open file");
    file.write_all("Name of Commisioner\t Ministry\t Geopolitical Zone\t".as_bytes()).expect("unable to do so");
    file.write_all("\n".as_bytes());

    println!("How many ministers are to be registered??");
    io::stdin().read_line(&mut input).expect("invalid");
    let number_of_convicted_ministers:usize = input.trim().parse().expect("invalid");

    for i in 0..number_of_convicted_ministers{
            let mut minister_name = String::new();
            let mut ministry = String::new();
            let mut geopolitical_zone = String::new();
            let mut Convicted_Minister_details = Vec::new();
            
            println!("Minister name??");
            io::stdin().read_line(&mut minister_name).expect("invalid");
            let minister_name = minister_name.trim();
            Convicted_Minister_details.push(minister_name.to_string());

            println!("Ministry??");
            io::stdin().read_line(&mut ministry).expect("invalid");
            let ministry = ministry.trim();
            Convicted_Minister_details.push(ministry.to_string());

            println!("Geopolitical Zone??");
            io::stdin().read_line(&mut geopolitical_zone).expect("invalid");
            let geopolitical_zone = geopolitical_zone.trim();
            Convicted_Minister_details.push(geopolitical_zone.to_string());

            for i in 0..Convicted_Minister_details.len(){
                file.write_all(Convicted_Minister_details[i].as_bytes()).expect("unable to do so");
                file.write_all("\t".as_bytes()).expect("unable to do so");
            }
            file.write_all("\n".as_bytes()).expect("unable to do so");

            println!("you have been registered");
    }
    
}
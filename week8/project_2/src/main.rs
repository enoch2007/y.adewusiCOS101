use std::io;

fn details(name: &mut Vec<String>, degree: &mut Vec<String>, experience: &mut Vec<usize>){
    let mut input = String::new();
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Input name of candidate:");
    io::stdin().read_line(&mut input).expect("invalid input");
    let your_name = input.trim();
    name.push(your_name.to_string());

    println!("what did you major in??(your degree)");
    io::stdin().read_line(&mut input1).expect("invalid");
    let b_sc = input1.trim();
    degree.push(b_sc.to_string());

    println!("how long have you been a programmer??(input your experience in years)");
    io::stdin().read_line(&mut input2).expect("invalid");
    let years:usize = input2.trim().parse().expect("invalid");
    experience.push(years);
}

fn main(){
    let mut name: Vec<String> = Vec::new();
    let mut degree: Vec<String> = Vec::new();
    let mut experience: Vec<usize> = Vec::new();
    let mut input3 =String::new();
    println!("how many persons are to be interviewed??");
    io::stdin().read_line(&mut input3).expect("invalid");
    let to_be_interviewed:usize = input3.trim().parse().expect("invalid");

    for j in 0..to_be_interviewed{
        details(&mut name, &mut degree, &mut experience);
    }

    let most_experience = checker(to_be_interviewed, &experience);
    println!("\nThe person with the most experience is {}. With a degree of {} and {} years of experience",name[most_experience], degree[most_experience],
        experience[most_experience]);
}

fn checker(input4:usize, years: &Vec<usize>) -> usize {
    let mut experience = 0;
    let mut max_experience = 0;
    for j in 0..input4{
        if years[j] > experience{
            experience = years[j];
            max_experience = j;
        }
    }
    return max_experience;
}
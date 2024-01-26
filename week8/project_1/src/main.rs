use std::io;

fn main() {
    println!("Nigerian Public Service APS Level Checker");

    let mut num_of_staff = String::new();
    println!("how many staffs are in your service??");
    io::stdin().read_line(&mut num_of_staff).expect("invalid");
    let num_of_staff:usize = num_of_staff.trim().parse().expect("invalid");

    let mut name_of_staff = String::new();
    println!("what is your name??");
    io::stdin().read_line(&mut name_of_staff).expect("invalid");
    let Name = name_of_staff.trim();

    let mut Age = String::new();
    println!("input your age");
    io::stdin().read_line(&mut Age).expect("not an integer");
    let age:i32 = Age.trim().parse().expect("not an integer");

    let mut input = String::new();
    println!("What is your field of expertise??
        \nlawyer
        \nacademics
        \noffice administrator
        \nteacher");
    io::stdin().read_line(&mut input).expect("invalid");
    let field_of_expertise = input.trim();

    let mut input1 = String::new();
    println!("How many years of experience do you have??");
    io::stdin().read_line(&mut input1).expect("invalid");
    let years:i64 = input1.trim().parse().expect("invalid");

    let mut level = "";
    let year = years.to_string();
    let Age = age.to_string();

    let staff_details = vec![Name, &Age, field_of_expertise, &year];

    if field_of_expertise == "office administrator"{
        if years >=1 && years <=3{
            let  level = "intern";
            println!("Your APS level is {}", level);
        }else if years >3 && years <=6{
            let level = "administrator";
            println!("Your APS level is {}", level);
        }else if years >6 && years <=8{
            let level = "senior administrator";
            println!("Your APS level is {}", level);
        }else if years >8  && years <=11{
            let level = "office manager";
            println!("Your APS level is {}", level);
        }else if years >11 && years <=13{
            let level = "director";
            println!("Your APS level is {}", level);
        }else if years >13 {
            let level = "CEO";
            println!("Your APS level is {}", level);
        }else {
            println!("not existent");
        }
        println!("{:?}\n", staff_details);    
    }else if field_of_expertise == "academic"{
        if years >=1 && years <=3{
            let level = "rookie";
            println!("Your APS level is {}", level);
        }else if years >3 && years <=6{
            let level = "Research assistant";
            println!("Your APS level is {}", level);
        }else if years >6 && years <=8{
            let level = "PhD candidate";
            println!("Your APS level is {}", level);
        }else if years >8  && years <=11{
            let level = "post-doc researcher";
            println!("Your APS level is {}", level);
        }else if years >11 && years <=13{
            let level = "Senior lecturer";
            println!("Your APS level is {}", level);
        }else if years >13 {
            let level = "Dean";
            println!("Your APS level is {}", level);
        }else {
            println!("not existent");
        }
        println!("{:?}\n", staff_details);
    }else if field_of_expertise == "teacher" {
        if years >=1 && years <=3{
            let level = "placement";
            println!("Your APS level is {}", level);
        }else if years >3 && years <=6{
            let level = "Classroom teacher";
            println!("Your APS level is {}", level);
        }else if years >6 && years <=8{
            let level = "Senior teacher";
            println!("Your APS level is {}", level);
        }else if years >8  && years <=11{
            let level = "leading teacher";
            println!("Your APS level is {}", level);
        }else if years >11 && years <=13{
            let level = "Deputy Principal";
            println!("Your APS level is {}", level);
        }else if years >13 {
            let level = "Principal";
            println!("Your APS level is {}", level);
        }else {
            println!("not existent");
        }
        println!("{:?}\n", staff_details);
    }else if field_of_expertise == "lawyer" {
        if years >=1 && years <=3{
            let level = "Paralegal";
            println!("Your APS level is {}", level);
        }else if years >3 && years <=6{
            let level = "Junior associate";
            println!("Your APS level is {}", level);
        }else if years >6 && years <=8{
            let level = "Associate";
            println!("Your APS level is {}", level);
        }else if years >8  && years <=11{
            let level = "senior associate 1-2";
            println!("Your APS level is {}", level);
        }else if years >11 && years <=13{
            let level = "senior associate 3-4";
            println!("Your APS level is {}", level);
        }else if years >13 {
            let level = "partner";
            println!("Your APS level is {}", level);
        }else {
            println!("not existent");
        }
        println!("{:?}\n", staff_details);
    }else {
        println!("not a field of expertise here");
    }

    
}



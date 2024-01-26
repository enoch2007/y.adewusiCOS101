use std::io::Write;

fn main(){
    let lager = vec!["33export\n".to_string(), "Despersdos\n".to_string(), "Goldberg\n".to_string(), "Gulder\n".to_string(), "Heineken\n".to_string(), "Star\n".to_string()];
    let stout = vec!["Legend\n".to_string(), "Turbo King\n".to_string(), "Williams\n".to_string()];
    let non_alcholic = vec!["Maltina\n".to_string(), "Amstel Malt\n".to_string(), "Malta Gold\n".to_string(), "Fayrouz\n".to_string()];

    let mut file = std::fs::File::create("lager drinks.txt").expect("unable to open file");
    let mut files = std::fs::File::create("stout drinks.txt").expect("unable to open file");
    let mut filess = std::fs::File::create("Non alcholic drinks.txt").expect("unable to open file");
    
    for i in lager.iter(){
        file.write_all(i.as_bytes()).expect("unable to do so");
    }

    for j in stout.iter(){
        files.write_all(j.as_bytes()).expect("unable to do so");
    }

    for k in non_alcholic.iter(){
        filess.write_all(k.as_bytes()).expect("unable to do so");
    }
    println!("all data has been input into file");
}
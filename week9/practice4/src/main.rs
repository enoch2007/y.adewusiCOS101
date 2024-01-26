use std::fs::OpenOptions;
use std::io::Write;

fn main(){
    let mut file = OpenOptions::new().append(true).open("I'm Him.txt").expect("cannoyt open file");
    file.write_all("\nHello world".as_bytes()).expect("failed to write");
    file.write_all("\nthis is appendage to the document".as_bytes()).expect("write faled");
    println!("file append successful");
}
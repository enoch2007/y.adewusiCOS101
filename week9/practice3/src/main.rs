use std::fs;

fn main(){
    fs::remove_file("I'm Him.txt").expect("unable to delete");
    println!("File is removed");
}
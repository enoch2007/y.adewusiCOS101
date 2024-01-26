use std::io::Write;

fn main() {
    let announce = "I AM HIM!!!";
    let name = "Play_Taku";

    let mut file = std::fs::File::create("I'm Him.txt").expect("failed to create");
    file.write_all("welcome to my world\n"
        .as_bytes()).expect("failed to write");
    file.write_all(announce.as_bytes()).expect("failed to write");
    file.write_all(name.as_bytes()).expect("failed to write");
    println!("data written to file");
}
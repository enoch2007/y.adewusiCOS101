fn main() {
    // initialize a mutable tuple
    let mut mountain_heights = ("everest", 8848, "fishtail", 6993);

    println!("original tuple = {:?}", mountain_heights);

    //change 3rd and 4th elements
    mountain_heights.2 = "lhose";
    mountain_heights.3 = 8516;

    println!("changed tuple = {:?}", mountain_heights);
}

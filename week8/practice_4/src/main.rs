fn main() {
    //Name vector
    let name = vec!["mary","sam","sally","ade","mark","june","ife","jp","jhae"];

    //age vector
    let age = vec![16,17,17,22,20,24,21,18,23];

    print!("\nAge allocation:\n");

    //loop to iterate elements of a vector
    for i in 0..age.len()
    {
        // iterating through i on the vector
        print!("{} is {} years old\n",name[i],age[i] );
    }
}

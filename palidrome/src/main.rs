use std::io;
fn main() {
    let mut input_str: String = String::new();

    println!("Enter String below: ");
    io::stdin()
        .read_line(&mut input_str)
        .expect("Invalid string input");

    //Reverse string
    let rev_str: String = str_rev(&input_str);
    println!("Reverse string = {rev_str}");

    if input_str.trim() == rev_str.trim()
    {
        println!("Palidrome");
    }
    else
    {
        println!("Not Palidrome");
    }
}

//REVERSE STRING FUNCTION
fn str_rev(string: &str) -> String {
    //Create mutable variable string type so it can store reversed string
    let mut rev_str: String = String::new();

    //chars() convert string into character array 
    //Hello -> (H,e,l,l,o) to iterate
    for character in string.chars().rev() {
        //push() used for single character it push char into stirng
        //push_str() push &str so it push character slice
        rev_str.push(character);
    }
    return rev_str;
}

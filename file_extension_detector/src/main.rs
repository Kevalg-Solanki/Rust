use std::io::{self,Write};
fn main() {

    //Define variables
    let mut input_file_name:String = String::new();
    //GET USER INPUT
    print!("Enter File Name = ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input_file_name).expect("Please Enter Valid File Name");
    let extension = detect_file_extension(&input_file_name);
    
    println!("Extension of file = {extension}");
}

fn detect_file_extension(file_name:&str)->String
{

    let mut extension:String = String::new();
    let mut start_adding:bool = false;
    for ch in file_name.chars() 
    {
        if ch=='.'
        {
            start_adding = true;
            extension.clear();
        }
        else if start_adding
        {
            extension.push(ch);
        }
    }
    return extension;
}

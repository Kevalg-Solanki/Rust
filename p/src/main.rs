use std::io;

fn main() {
  
    //Create dynamic string variable
    let array = [1,2,4,5,6,6];
    let mut index: String = String::new();

    //Take input 
    println!("Enter string = ");
    io::stdin().read_line(&mut index).expect("No input");

    //shadow variable
    let index:usize = index.trim().parse().expect("Not a number");

    let e = array[index];
    println!("Input string = {e}");
}

use std::io;

fn main() {
    let mut input_num:String=String::new();

    println!("Enter Number = ");
    //Get input
    io::stdin().read_line(&mut input_num).expect("Please Enter valid number");

    //Convert to number
    let input_num:i32 = input_num.trim().parse().expect("Invalid Input");

    let mut result:i32;
    for i in 1..11
    {
        result = input_num*i;
        println!("{input_num} x {i} = {result}");
    }
}

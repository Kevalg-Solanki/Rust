use std::io;

fn main() {
    let mut input_num1: String = String::new();
    let mut input_num2: String = String::new();
    let mut input_symbol: String = String::new();

    println!("Num 1 :");
    io::stdin()
        .read_line(&mut input_num1)
        .expect("Enter Number");
    println!("Num 2 :");
    io::stdin()
        .read_line(&mut input_num2)
        .expect("Enter Number");

    let input_num1:f64 = input_num1.trim().parse().expect("Invalid Number 1");
    let input_num2:f64 = input_num2.trim().parse().expect("Invalid Number 2");

    println!("Enter Annotation (+,-,x,/) :");
    io::stdin().read_line(&mut input_symbol).expect("Enter Valid Symbol");

    let mut result:f64=0.00;
    let symbol = input_symbol.trim();
    if symbol=="+"
    {
        println!("Addition");
        result = input_num1+input_num2;
    }
    else if symbol =="-"
    {
        println!("Substraction");
        result = input_num1-input_num2;
    }
    else if symbol == "x"
    {
        println!("Multiplication");
        result = input_num1*input_num2;

    }
    else if symbol == "/"
    {
        println!("Division");
        result = input_num1/input_num2;
    }
    else
    {
        println!("None selected");
    }

    println!("Result = {result}");
    

}

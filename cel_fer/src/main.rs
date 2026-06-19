use std::io;

fn main() {
    //Create string instance for storing string input
    let mut input_temp_c: String = String::new();

    println!("Enter celsius = ");
    //Take user input by using io library by passing mutable referance of created string instance
    io::stdin()
        .read_line(&mut input_temp_c)
        .expect("Input is required");

    //Convert string into number
    let temp_c: f64 = input_temp_c.trim().parse().expect("Enter valid number");

    let temp_f: f64 = (temp_c * 9.0 / 5.0) + 32.00;

    println!("Fehrenheit = {temp_f}");
}

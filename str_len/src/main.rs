use std::io;

fn main() {
    let mut input_str: String = String::new();

    io::stdin().read_line(&mut input_str).expect("Valid string");

    let string_length = get_str_length(&input_str);
    println!("string length = {string_length}");
}

fn get_str_length(s: &String)->isize{
    let mut length=0;

    for _byte in s.trim().as_bytes(){
        length+=1;
        println!("length ++");
    }
    return length;
}
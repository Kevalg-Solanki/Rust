fn main() {
    let text: String = String::from("Hello");
    //Move ownership
    let full_text = add_text(text);
    println!("{full_text}");
}

fn add_text(mut text:String)->String{
    //Modify string
     text.push_str(" Rust");
    //return string by writing expression("without semicolon");
    text

}

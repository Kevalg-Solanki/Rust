use std::io;

fn main() {
    //Create mutable variable string instance type
    let mut input_str: String = String::new();

    //Get User Input
    println!("Enter String = ");
    io::stdin()
        .read_line(&mut input_str)
        .expect("Enter String Please");

    //Pass reference in the word counter
    let number_of_words: usize = word_counter(&input_str);

    println!("Number of words = {number_of_words}");
}

fn word_counter(string: &str) -> usize {
    //&str suppports String, &String, slices like $s[0..1]
    //Convert string into bytes
    let bytes = string.as_bytes(); //Converts string into bytes array
                                // Hello World = (75,40,430,405,405," ",89,345,345,345,354)

    //Variable to store counts
    let mut count = 0;

    //Creating is_in_word to check is current character is space
    let mut in_word: bool = false;

    //Loop through bytes to get space
    for &item in bytes.iter()
    //Create bytes into interation like (72),(12) type of u8 or by reference type &u8
    //enumerates() add indexes to the iterates (1,72), (2,12)
    //&item means get type &u8 from result created by iter() &72
    //(i,&item) means (1,refrenece of 72)
    {
        //If Space or New line or tab comes in
        if item == b' ' || item == b'\n' || item == b'\t' ||item ==b'\r'{
            println!("space");
            in_word = false;
        } else if !in_word
        // if character comes after the space comes than count as word
        {

            println!("Count {item}");
            count += 1; //increment in word count
            in_word = true //setting true to set this word is counted and only false when space comes
        }
    }

    return count;
}

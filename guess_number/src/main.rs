use rand::Rng;
use std::io::{self, Write};
fn main() {
    //
    let mut rng = rand::thread_rng();

    //Create guess as string instance for getting input
    let mut guess: String = String::new();

    //Generate random number 0 to 100
    let random_num: i32 = rng.gen_range(0..100);
    println!("Random Number = {random_num}");
    //CREATE INFINITE LOOP
    loop {
        //Clean string input cause read_line keep adding into exting string
        guess.clear();
        //TO TAKE INPUT IN SAME LINE AS PRINT LINE
        print!("Guess = ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut guess)
            .expect("Enter Valid Number");

        //Convert guess into number
        // HANDLE INVALID NUMBER
        let guess_number: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please Enter Valid Number");
                continue;
            }
        };

        if guess_number > random_num {
            println!("Think something Small");
        } else if guess_number < random_num {
            println!("Think Something Big");
        } else if guess_number == random_num {
            println!("You won");
            break;
        }
    }
}

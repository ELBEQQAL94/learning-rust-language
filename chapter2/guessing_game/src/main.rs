use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    
    //println!("The secret number is: {} ", secret_number);
    
    loop {
        println!("Please enter your guess.");
        // initial empty string
        let mut guess = String::new();
        
        //handle_user_input(guess);
        
        //println!("Your guessed is {}", handle_user_input(guess));
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }; 
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }


}

// fn handle_user_input(mut number: String) -> String{
//     io::stdin()
//         .read_line(&mut number)
//         // catch errors
//         .expect("Failed to read line");
//     number
// }

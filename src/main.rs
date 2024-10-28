use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;
fn main() {
    println!("Guess the Number !");

   

    loop {
        println!("Please input your number");

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guessed: {}", guess);

        let secret_number = rand::thread_rng().gen_range(1..20);
    println!("The secret number is : {}",secret_number);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("{}","Too Small!".red()),
            Ordering::Greater => println!("{}","Too Big!".red()), 
            Ordering::Equal => {
                println!("{}","You win!".green());
                break;
            },
        }
    }


   

}

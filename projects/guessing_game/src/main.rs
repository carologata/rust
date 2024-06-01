use rand::Rng;
use std::cmp::Ordering;
use std::io;


/* The :: syntax in the ::new line indicates that new is an associated function of the String type. 
An associated function is a function that’s implemented on a type, in this case String. This new function 
creates a new, empty string. You’ll find a new function on many types because it’s a common name for a 
function that makes a new value of some kind. */

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    loop
    {
        println!("Please input your guess.");

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guessed: {guess}");
    
        match guess.cmp(&secret_number)
        {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    
}
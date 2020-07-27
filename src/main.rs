use std::io::stdin;
use std::cmp::Ordering;
use rand::{thread_rng, Rng};

fn main() {
    println!("Guess the number between 0 and 10!");

    let number = thread_rng().gen_range(0, 11);

    loop 
    {
        println!("Please input your guess: ");

        let mut guess = String::new();

        match stdin().read_line(&mut guess)
        {
            Ok(_) => println!(""),
            Err(error) => println!("An error occured: {}", error)
        };

        let guess: u32 = match guess.trim().parse()
        {
            Ok(num) => num,
            Err(error) => {
                println!("{} is not a number!", error);
                continue;
            }
        };

        match guess.cmp(&number) 
        {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You guessed right {} and Win!", number);
                break;
            },
        };
        println!();
    }
}
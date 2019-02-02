use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() 
{
    println!("Guess what number I am thinking of!");
    println!("Please input your guess.");

    let mut guess = String::new();
    let secret_number = rand::thread_rng().gen_range(0,101);

    println!("The secret number is {}", secret_number);

    loop 
    {
        io::stdin().read_line(&mut guess)
            .expect("failed to read line.");

        let guess: u32 = guess.trim().parse()
            .expect("Please type a number!");

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number)
        {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater =>  println!("Too big!"),
            Ordering::Equal => 
            {
                println!("You win!");
                break;
            }
        }

    }
}

// std::io is an input/output function provided by the standard rust library.
// Rust has a lot of items defined in the standard library and all the items as a set is called prelude.
use rand::Rng;
use std::{io, cmp::Ordering};

fn main() {
    println!("Welcome to the Guess the Number Game!");

    let secret = rand::thread_rng().gen_range(1..=100);

    println!("I've selected a random number between 1 and 100. Try to guess it!");

    // putting everything into a loop will allow for the user to retry
    loop {
        println!("Enter your guess:");

        // What is a mut? By default a variable in rust is an immutable variable meaning that the value can not be changed, BUT by adding a mut infront of let will turn the
        // variable into a mutable variable which will allow it to be changed later on.
        let mut guess = String::new();

        // Now, we call the stdin function which we imported from the io function from the standard library which will allows us to handle under input
        // read_line is a method that allows us to read the user input.
        // what is the & symbol? it is basically a reference to something like for example a mut using &mut then add a variable referencing it from.
        // expext is a method that allwos us to handle potential failure with result.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // // wooo now we can print a line after the guess using brackets {} we can add which variable to output
        // println!("You guessed the number: {guess}");

        // using match we can compare the guess variable using .cmp method with another variable using &secret.
        // using the Ordering function we can see if its Less, Equal or Greater!
        match guess.cmp(&secret) {
            Ordering::Less => println!("You are {} too low!", secret - guess),
            Ordering::Equal => {
                println!("Congratulations! You guessed the number {secret}!");
                break;
            }
            Ordering::Greater => println!("You are {} too high!", guess - secret),
        }
    }
}

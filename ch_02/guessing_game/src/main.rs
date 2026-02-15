// For comparisons and matching
use std::cmp::Ordering;
// use std::io is a statement that brings the io module from the standard library
// into scope. This allows us to use the functions and types defined in the io
// module without having to prefix them with std::io every time.
use std::io;
// library for randomizations.
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        // let mut guess = String::new(); creates a mutable variable named guess and
        // initializes it with a new, empty String. The String::new() function is
        // a constructor that creates a new String instance. The mut keyword allows
        // us to modify the guess variable later in the code when we read input from
        // the user.
        // Without the mut keyword, the guess variable would be immutable.
        let mut guess = String::new();

        // io::stdin() returns a handle to the standard input stream, which allows us
        // to read input from the user. The read_line method is called on this handle,
        // and it takes a mutable reference to the guess variable as an argument. This
        // allows the read_line method to store the user's input in the guess variable.
        // The except method is called on the result of read_line to handle any errors
        // that may occur during the input process.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        // Shadowing: the process of converting a variable from one type to another
        // while reusing the variable name.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        };
    }
}

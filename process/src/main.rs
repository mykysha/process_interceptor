use std::io::{self, BufRead};
use std::process;

fn main() {
    println!("My process ID is {}", process::id());
    
    let stdin = io::stdin();
    let mut messages = Vec::new();

    for line in stdin.lock().lines() {
        match line {
            Ok(text) => {
                messages.push(text.clone()); // Save the message in the vector
                println!("you wrote -> {}", text);
            }

            Err(error) => {
                eprintln!("Error reading input: {}", error);

                break;
            }
        }
    }

    println!("Message history:");
    for message in messages {
        println!("{}", message); // Print each message in the vector
    }
}


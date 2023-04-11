use rand::seq::SliceRandom;
use std::io;

fn main() {
    // Create a vector to hold the names
    let mut names = Vec::new();

    // Ask the user for names until they enter an empty string
    loop {
        println!("Enter a name (leave blank to exit):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let trimmed_input = input.trim();
        if trimmed_input.is_empty() {
            break;
        }

        names.push(trimmed_input.to_string());
    }

    // Select a random name from the vector and print it
    if let Some(random_name) = names.choose(&mut rand::thread_rng()) {
        println!("The random name is: {}", random_name);
    } else {
        println!("No names were entered");
    }
}

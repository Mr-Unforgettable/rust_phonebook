mod options;
use options::switch;
use std::io::{self, Write};
use crate::options::contact::Phonebook;

fn main() {
    println!("++++++++++++++++++++++++++++++++");
    println!("      Welcome to Phonebook      ");
    println!("++++++++++++++++++++++++++++++++");

    let mut phonebook = Phonebook::new();  // Need some review.

    loop {
        println!("\nPlease choose an option:");
        println!("1. Add a contact");
        println!("2. View contacts");
        println!("3. Search contacts");
        println!("4. Update a contact");
        println!("5. Delete a contact");
        println!("6. Save contacts");
        println!("7. Load contacts [IN DEVELOPMENT]");
        println!("0. Exit\n");

        println!("++++++++++++++++++++++++++++++++");
        print!("Option: ");
        io::stdout().flush().unwrap();

        let mut option = String::new();
        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");

        let option: u8 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue
            },
        };

        switch(&mut phonebook, option);  // Same here
    }
}

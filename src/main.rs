mod contact;
// use contact::{Contact, Phonebook};
use std::io;

fn main() {
    println!("++++++++++++++++++++++++++++++++");
    println!("      Welcome to Phonebook      ");
    println!("++++++++++++++++++++++++++++++++");

    loop {

        println!("\nPlease choose an option:");
        println!("1. Add a contact");
        println!("2. View contacts");
        println!("3. Search contacts");
        println!("4. Update a contact");
        println!("5. Delete a contact");
        println!("6. Save contacts");
        println!("0. Exit");

        println!("++++++++++++++++++++++++++++++++");
        print!("Option: ");

        let mut option = String::new();

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to readline.");

        let option: u32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                continue
            },
        };

        match option {
            1 => {
                // Add a contact
                println!("1. Add a contact");
            },
            2 => {
                // View contacts
                println!("2. View contacts");
            },
            3 => {
                // Search contacts
                println!("3. Search contacts");
            },
            4 => {
                // Update a contact
                println!("4. Update a contact");
            },
            5 => {
                // Delete a contact
                println!("5. Delete a contact");
            },
            6 => {
                // Save contacts
                println!("6. Save contacts");
            },
            0 => {
                // Exit
                println!("Exiting program ...");
                println!("Thank you for using our phonebook!");
                break;
            },
            _ => {
                // Print some error message for now at least.
                println!("Invalid option!");
            }
        }

    }

}

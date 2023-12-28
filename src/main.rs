use std::io;

mod contact;
use contact::{Contact, Phonebook};

fn main() {
    println!("++++++++++++++++++++++++++++++++");
    println!("      Welcome to Phonebook      ");
    println!("++++++++++++++++++++++++++++++++");

    let mut phonebook = Phonebook::new();
    
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

        println!("Option: ");
        let mut option = String::new();

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to readline.");

        let option: u32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match option {
            1 => {
                // Add a contact
                println!("Enter the contact details:");

                println!("Name: ");
                let mut name = String::new();
                io::stdin()
                    .read_line(&mut name)
                    .expect("Failed to read line.");

                println!("Email: ");
                let mut email = String::new();
                io::stdin()
                    .read_line(&mut email)
                    .expect("Failed to read line.");

                println!("Phone: ");
                let mut phone = String::new();
                io::stdin()
                    .read_line(&mut phone)
                    .expect("Failed to read line.");

                let contact = Contact {
                    name: name.trim().to_string(),
                    phone_number: phone.trim().to_string(),
                    email: email.trim().to_string(),
                };

                phonebook.add_contact(contact);
                println!("Contact added successfully.");
            }
            2 => {
                // View contacts
                println!("Viewing contacts ...");
                println!("Number of contacts are {}", phonebook.contacts.len());
                phonebook.view_contacts();
            }
            3 => {
                // Search contacts
                println!("Enter your search query: ");

                let mut query = String::new();

                io::stdin()
                    .read_line(&mut query)
                    .expect("Failed to read lines.");

                let search_result = phonebook.search_contacts(&query.trim().to_lowercase());
                println!("Search results are: \n {:?}", search_result);
            }
            4 => {
                // Update a contact
                println!("4. Update a contact");
            }
            5 => {
                // Delete a contact
                println!("5. Delete a contact");
            }
            6 => {
                // Save contacts
                println!("6. Save contacts");
            }
            0 => {
                // Exit
                println!("Exiting program ...");
                println!("Thank you for using our phonebook!");
                break;
            }
            _ => {
                // Print some error message for now at least.
                println!("Invalid option!");
            }
        }
    }
}

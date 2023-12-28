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
        println!("7. Load contacts");
        println!("0. Exit\n");
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
                println!("Enter the name of the contact to be updated: ");
                let mut update_name = String::new();
                io::stdin()
                    .read_line(&mut update_name)
                    .expect("Failed to read line.");

                // Search for contact by name
                if let Some(index) = phonebook
                    .contacts
                    .iter()
                    .position(|c| c.name.trim() == update_name.trim())
                {
                    // contact found, now update the details
                    println!("Enter the updated contact details:");

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

                    // Create a update contact
                    let update_contact = Contact {
                        name: name.trim().to_string(),
                        phone_number: phone.trim().to_string(),
                        email: email.trim().to_string(),
                    };

                    // Update the contact in the phonebook
                    phonebook.update_contact(index, update_contact);
                    println!("Contact updated successfully!");
                } else {
                    println!("Contact not found.");
                }
            }
            5 => {
                // Delete a contact
                println!("Enter the name of the contact to delete: ");
                let mut delete_name = String::new();
                io::stdin()
                    .read_line(&mut delete_name)
                    .expect("Failed to read line.");

                // Search the contact.
                if let Some(index) = phonebook
                    .contacts
                    .iter()
                    .position(|c| c.name.trim() == delete_name.trim())
                {
                    // Contact found, delete it.
                    phonebook.delete_contact(index);
                    println!("Contact deleted successfully.");
                } else {
                    println!("Contact not found.");
                }
            }
            6 => {
                // Save contacts
                let mut filename = String::new();
                println!("Enter the file name:");
                io::stdin()
                    .read_line(&mut filename)
                    .expect("Failed to read line.");

                println!("Saving phonebook...");

                phonebook
                    .save_contacts(&filename)
                    .expect("Failed to save file.");

                println!("Contacts save successfully.");
            }
            // 7 => {
            //     // Load contacts
            //     let mut filename = String::new();
            //     io::stdin()
            //         .read_line(&mut filename)
            //         .expect("Failed to read line.");

            //     match env::current_dir() {
            //         Ok(current_dir) => {
            //             let absolute_path = current_dir.join(&filename);
            //             match phonebook.load_contacts(absolute_path.to_str().unwrap()) {
            //                 Ok(_) => println!("Contacts loaded successfully."),
            //                 Err(err) => eprintln!("Failed to load file: {}", err),
            //             }
            //         }
            //         Err(err) => {
            //             eprintln!("Failed to get current directory: {}", err)
            //         }
            //     }
            // }
            0 => {
                // Exit
                println!("Exiting program ...");
                println!("Bye Bye 👋");
                break;
            }
            _ => {
                // Print some error message for now at least.
                println!("Invalid option!");
            }
        }
    }
}

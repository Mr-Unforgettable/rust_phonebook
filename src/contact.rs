use std::error::Error;
use std::fmt;
use std::io::{BufWriter, BufReader};
use std::fs::File;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Contact {
   pub name: String,
   pub phone_number: String,
   pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct Phonebook {
    pub contacts: Vec<Contact>,
}

impl fmt::Display for Contact {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({}, {})", self.name, self.phone_number, self.email)
    }
}

impl Phonebook {
    pub fn new() -> Phonebook {
        Phonebook { contacts: Vec::new() }
    }

    pub fn add_contact(&mut self, contact: Contact) {
        self.contacts.push(contact);
    }

    pub fn view_contacts(&self) {
        for contact in &self.contacts {
            println!("{}", contact);
        }
    }

    pub fn search_contacts(&self, query: &str) -> Vec<&Contact> {
        self.contacts
            .iter()
            .filter(|contact| {
                contact.name.contains(query)
                || contact.phone_number.contains(query)
                || contact.email.contains(query)
            })
            .collect()
    }

    pub fn update_contact(&mut self, index: usize, contact: Contact) {
        self.contacts[index] = contact;
    }

    pub fn delete_contact(&mut self, index: usize) {
        self.contacts.remove(index);
    }

    pub fn save_contacts(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        let file = File::create(filename)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &self)?;
        Ok(())
    }

    pub fn load_contacts(filename: &str) -> Result<Phonebook, Box<dyn Error>> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let phonebook = serde_json::from_reader(reader)?;
        Ok(phonebook)
    }

}
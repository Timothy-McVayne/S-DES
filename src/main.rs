mod functions; 

use std::io; 
use std::io::Write;
use functions::{generate_keys, encrypt};
fn main() {

    /*
    Loop beginning until done
    Ask if user wants to encrypt or decrypt
    Ask user where file holding key is
    pull key from file FUNCTION
    begin encryption/decryption process FUNCTION
    message that action has been taken and where to find new file. END LOOP
     */
    let mut path = String::new();
    println!("Please enter the name of the file with the key: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut path).expect("Couldn't read line");
    encrypt("test.txt", "key.txt");
}

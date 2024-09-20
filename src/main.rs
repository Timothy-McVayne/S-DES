mod functions; 

use std::io; 
use std::io::Read;
use std::io::Write;
use std::path::Path;
use functions::encrypt;
use functions::decrypt;
fn main() {

    /*
    Loop beginning until done
    Ask if user wants to encrypt or decrypt
    Ask user where file holding key is
    pull key from file FUNCTION
    begin encryption/decryption process FUNCTION
    message that action has been taken and where to find new file. END LOOP
     */
    loop
    {
        let mut response = String::new();
        println!("Would you like to encrypt or decrypt?");
        println!("1. Encrypt");
        println!("2. Decrypt");
        println!("3. Quit");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut response).unwrap();

        //encrypt
        if response.trim() == "1"
        {
            let mut key = String::new();

            loop 
            {
                println!("What is the name of the file holding the 10 bit encryption key?");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut key).unwrap();

                let key = key.trim();

                if Path::new(&key).exists()
                {
                    break;
                }
                else
                {
                    println!("Key file not found! Try again!");
                }
            }

            let mut path = String::new();

            loop 
            {
                println!("What is the name of the file to be encrypted?");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut path).unwrap();

                let path = path.trim();

                if Path::new(&path).exists()
                {
                    break;
                }
                else 
                {
                    println!("File not found! Try again!");
                }
            }
            encrypt(&path.trim(), &key.trim());
        }

        //decrypt
        else if response.trim() == "2"
        {
            let mut key = String::new();

            loop 
            {
                println!("What is the name of the file holding the 10 bit decryption key?");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut key).unwrap();

                let key = key.trim();

                if Path::new(&key).exists()
                {
                    break;
                }
                else
                {
                    println!("Key file not found! Try again!");
                }
            }

            let mut path = String::new();

            loop 
            {
                println!("What is the name of the file to be decrypted?");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut path).unwrap();

                let path = path.trim();

                if Path::new(&path).exists()
                {
                    break;
                }
                else 
                {
                    println!("File not found! Try again!");
                }
            }
            decrypt(&path.trim(), &key.trim());
        }

        else if response.trim() == "3"
        {
            break;
        }

        else
        {
            println!("Invalid Response!");
        }
    }
}

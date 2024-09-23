mod functions; 

use std::io; 
use std::io::Write;
use std::path::Path;
use functions::encrypt;
use functions::decrypt;
fn main() {

    //Main loop that lets you do things multiple times
    loop
    {
        //get the user input depending on what they want to do, switch to different cases depending on the input
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

            //Double check the name they input actually exists, keep getting input until it finds a file
            loop 
            {
                println!("What is the name of the file holding the 10 bit encryption key?");
                io::stdout().flush().unwrap();
                key.clear();
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

            //Do the same for the name of the file to actually be encrypted
            loop 
            {
                println!("What is the name of the file to be encrypted?");
                io::stdout().flush().unwrap();
                path.clear();
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

            //Call encryption function
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
                key.clear();
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
                path.clear();
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

            //Same as encryption function, verify input then call main function
            decrypt(&path.trim(), &key.trim());
        }

        //If they want to quit, quit
        else if response.trim() == "3"
        {
            break;
        }

        //invalid input, repeat loop
        else
        {
            println!("Invalid Response!");
        }
    }
}

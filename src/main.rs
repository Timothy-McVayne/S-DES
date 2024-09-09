mod functions; 

use std::fs;
use std::fs::File;
use std::io::Read;
fn main() {

    let mut file = File::open("card.jpg").expect("Can't open file");
    let metadata = file.metadata().expect("Can't read metadata");
    let mut buffer: Vec<u8> = vec![0; metadata.len() as usize];
    file.read(&mut buffer).expect("Can't read file");
    let mut cipher_text: Vec<u8> = Vec::new();
    //let mut arr: [u8; 8] = [1, 5, 2, 0, 3, 7, 4, 6];
    for bytes in &buffer
    {
        cipher_text.push(*bytes);
        print!("{}", bytes); 
    }

    println!(""); 

    if cipher_text == buffer
    {
        println!("Equal");
        fs::write("cipher.jpg", cipher_text).unwrap();
    }

    /*println!("Plaintext is: {:08b}", value);  

    for i in 0..8 
    {
        rearranged |= ((value >> (8 - arr[i] - 1)) & 1) << ((arr.len() - 1) - i);
    };

    println!("Ciphertext after IP is: {:08b}", rearranged);*/

    //functions::print_hello(); 
    
    /*let mut i = 0; 
    while i < buffer.len()
    {
        let letter = format!("{:08b}", buffer[i]);
        println!("{}", letter);
        i += 1;
    }*/

    //io::copy(&mut file, &mut vec);

    //let size = BitVec::len(&vec); 
    //println!("{}", size);
}

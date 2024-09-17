use std::fs;
use std::fs::File;
use std::io::Read; 

const IP: [u8; 8] = [1, 5, 2, 0, 3, 7, 4, 6];
const KEYORD10: [u8; 10] = [2, 4, 1, 6, 3, 9, 0, 8, 7, 5]; 
const KEYORD8: [u8; 8] = [3, 0, 4, 1, 5, 2, 7, 6]; 

pub fn permute<T: Into<u16>>(value: T, order: &[u8], length: usize) -> u16
{
    let value: u16 = value.into();
    let mut rearranged: u16 = 0b0000000000000000; 
    for i in 0..length
    {
        rearranged |= ((value >> (length - order[i] as usize - 1)) & 1) << ((length - 1) - i);
    };

    return rearranged;
}

pub fn generate_keys(path: &str) -> (u8, u8)
{
    //Key at key.txt is 1010000010
    let path = path.trim(); 
    let key = fs::read_to_string(path).expect("Can't read file");
    let parsed_key = u16::from_str_radix(&key, 2).expect("Invalid"); 

    let permuted = permute(parsed_key, &KEYORD10, KEYORD10.len());

    let left: u8 = ((permuted >> 5) & 0b00011111) as u8;
    let right: u8 = (permuted & 0b00011111) as u8;  

    let rotleft = ((left << 1) | (left >> 4)) & 0b00011111;
    let rotright = ((right << 1) | (right >> 4)) & 0b00011111;

    let combined: u16 = ((rotleft as u16) << 5) | (rotright as u16) & 0b0000000011111111;
    let k1 = permute(combined, &KEYORD8, KEYORD8.len()) as u8;

    let rotleft = ((left << 3) | (left >> 2)) & 0b00011111;
    let rotright = ((right << 3) | (right >> 2)) & 0b00011111;

    let combined: u16 = (((rotleft as u16) << 5) | (rotright as u16)) & 0b0000000011111111;
    let k2 = permute(combined, &KEYORD8, KEYORD8.len()) as u8;

    return (k1, k2); 
    //generate keys from 10 bit input
}

pub fn encrypt(plain: &str, key: &str)
{
    let (k1, k2) = generate_keys(key); 

    let mut file = File::open(plain).expect("Can't open file");
    let metadata = file.metadata().expect("Can't read metadata");
    let mut buffer: Vec<u8> = vec![0; metadata.len() as usize];
    file.read(&mut buffer).expect("Can't read file");
    let mut cipher_text: Vec<u8> = Vec::new();
    for bytes in &buffer
    {
        cipher_text.push(*bytes);
    }

    for element in &mut cipher_text
    {
        encrypt_loop(*element, k1, k2); 
    }
    //fs::write("cipher.txt", cipher_text).unwrap();
}

pub fn decrypt()
{
    
}

fn encrypt_loop(data: u8, k1: u8, k2: u8)
{

    println!("data: {:08b}", data);
    println!("K1: {:08b}", k1); 
    println!("K2: {:08b}", k2); 

        /*
    Initial permutation FUNCTION

    Expanded permutation converts 4 bit input to 8 bit output

    S-boxes?

    another permutation on a 4 bit input

    swap the position of the groups of 4 bits

    inverse of IP

    */
}
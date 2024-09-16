use std::fs;

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

pub fn generateKeys(path: &str)
{
    //Key at key.txt is 1010000010
    let path = path.trim(); 
    let key = fs::read_to_string(path).expect("Can't read file");
    let parsed_key = u16::from_str_radix(&key, 2).expect("Invalid"); 
    let permuted = permute(parsed_key, &KEYORD10, KEYORD10.len());
    println!("{:010b}", permuted);
    let left: u8 = ((permuted >> 5) & 0b00011111) as u8;
    let right: u8 = (permuted & 0b00011111) as u8; 

    println!("{:05b}", left);
    println!("{:05b}", right); 

    let rotleft = ((left << 1) | (left >> 4)) & 0b00011111;
    let rotright = ((right << 1) | (right >> 4)) & 0b00011111;
    let combined: u16 = ((rotleft as u16) << 5) | (rotright as u16);
    println!("Combined before p8 prep: {:010b}", combined);
    let combined = (combined & 0b0000000011111111);
    println!("Combined after p8 prep: {:010b}", combined);

    let k1 = permute(combined, &KEYORD8, KEYORD8.len());
 
    let rotleft = ((left << 2) | (left >> 4)) & 0b00011111;
    let rotright = ((right << 2) | (right >> 4)) & 0b00011111;

    println!("Rotleft after a 2bit shift: {:05b}", rotleft);
    println!("Rotright after a 2bit shift: {:05b}", rotright);

    //generate keys from 10 bit input
}

pub fn encrypt()
{
    /*
    Initial permutation FUNCTION

    Expanded permutation converts 4 bit input to 8 bit output

    S-boxes?

    another permutation on a 4 bit input

    swap the position of the groups of 4 bits

    inverse of IP

    */
}

pub fn decrypt()
{
    
}
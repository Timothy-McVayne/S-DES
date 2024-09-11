use std::fs;

const IP: [u8; 8] = [1, 5, 2, 0, 3, 7, 4, 6];
const KEYORD: [u8; 10] = [2, 4, 1, 6, 3, 9, 0, 8, 7, 5]; 

pub fn permute<T: Into<u16>>(value: T, order: &[u8], length: usize)
{
    let value: u16 = value.into();
    let mut rearranged: u16 = 0b0000000000000000; 
    for i in 0..length
    {
        rearranged |= ((value >> ((order.len() - 1) - order[i] as usize - 1)) & 1) << ((order.len() - 1) - i);
    };

    println!("Rearranged is: {:b}", rearranged);
}

pub fn generateKeys(path: &str)
{
    //Key at key.txt is 1010000010
    let path = path.trim(); 
    let key = fs::read_to_string(path).expect("Can't read file");
    let parsed_key = u16::from_str_radix(&key, 2).expect("Invalid"); 
    println!("{:b}", parsed_key);
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
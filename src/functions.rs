use std::fs;
use std::fs::File;
use std::io::Read; 

const IP: [u8; 8] = [1, 5, 2, 0, 3, 7, 4, 6];
const INVIP: [u8; 8] = [3, 0, 2, 4, 6, 1, 7, 5];
const EP: [u8; 8] = [7, 4, 5, 6, 5, 6, 7, 4];
const P4: [u8; 4] = [1, 3, 2, 0];
const KEYORD10: [u8; 10] = [2, 4, 1, 6, 3, 9, 0, 8, 7, 5]; 
const KEYORD8: [u8; 8] = [3, 0, 4, 1, 5, 2, 7, 6]; 

const S0: [u8; 16] = [1, 0, 3, 2,
                      3, 2, 1, 0,
                      0, 2, 1, 3,
                      3, 1, 3, 2];

const S1: [u8; 16] = [0, 1, 2, 3,
                      2, 0, 1, 3,
                      3, 0, 1, 0,
                      2, 1, 0, 3];

fn permute<T: Into<u16>>(value: T, order: &[u8], length: usize) -> u16
{
    let value: u16 = value.into();
    let mut rearranged: u16 = 0b0000000000000000; 
    for i in 0..length
    {
        rearranged |= ((value >> (length - order[i] as usize - 1)) & 1) << ((length - 1) - i);
    };

    return rearranged;
}

fn generate_keys(path: &str) -> (u8, u8)
{
    //Key at key.txt is 1010000010
    let path = path.trim(); 
    let key = fs::read_to_string(path).expect("Key file not found!");
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

    let mut file = File::open(plain).expect("Plaintext file not found!");
    let metadata = file.metadata().expect("Can't read metadata");
    let mut buffer: Vec<u8> = vec![0; metadata.len() as usize];
    file.read(&mut buffer).expect("Can't read file");
    let mut cipher_text: Vec<u8> = Vec::new();
    for bytes in &buffer
    {
        cipher_text.push(*bytes);
    }

    for i in 0..cipher_text.len()
    {
        cipher_text[i] = encrypt_loop(cipher_text[i], k1, k2);
    }

    fs::write("cipher.txt", cipher_text).unwrap();
    fs::remove_file(plain).unwrap(); 
    println!("File has been succesfully encrypted!");
}

pub fn decrypt(cipher: &str, key: &str)
{
    let (k1, k2) = generate_keys(key); 

    let mut file = File::open(cipher).expect("Can't open file");
    let metadata = file.metadata().expect("Can't read metadata");
    let mut buffer: Vec<u8> = vec![0; metadata.len() as usize];
    file.read(&mut buffer).expect("Can't read file");
    let mut plain_text: Vec<u8> = Vec::new();
    for bytes in &buffer
    {
        plain_text.push(*bytes);
    }

    for i in 0..plain_text.len()
    {
        plain_text[i] = decrypt_loop(plain_text[i], k1, k2);
    }

    fs::write("plain.txt", plain_text).unwrap();
    fs::remove_file(cipher).unwrap(); 
    println!("File has been succesfully decrypted!");
}

fn encrypt_loop(data: u8, k1: u8, k2: u8) -> u8
{
    let permuted = permute(data, &IP, IP.len()) as u8;

    let left = (permuted >> 4) & 0b00001111; 
    let right = permuted & 0b00001111;
    let expright = permute(right, &EP, EP.len()) as u8;

    let intermediate = sbox(expright, k1, left, right);

    let left = (intermediate >> 4) & 0b00001111;
    let right = intermediate & 0b00001111;
    let expright = permute(right, &EP, EP.len()) as u8;

    let intermediate = sbox(expright, k2, left, right);

    let preinverse = (intermediate >> 4) | ((intermediate << 4) & 0b11110000);
    let ciphertext = permute(preinverse, &INVIP, INVIP.len()) as u8;
    
    return ciphertext; 
}

fn decrypt_loop(data: u8, k1: u8, k2: u8) -> u8
{
    let permuted = permute(data, &IP, IP.len()) as u8;

    let left = (permuted >> 4) & 0b00001111; 
    let right = permuted & 0b00001111;
    let expright = permute(right, &EP, EP.len()) as u8;

    let intermediate = sbox(expright, k2, left, right);

    let left = (intermediate >> 4) & 0b00001111;
    let right = intermediate & 0b00001111;
    let expright = permute(right, &EP, EP.len()) as u8;

    let intermediate = sbox(expright, k1, left, right);

    let preinverse = (intermediate >> 4) | ((intermediate << 4) & 0b11110000);
    let plaintext = permute(preinverse, &INVIP, INVIP.len()) as u8;
    
    return plaintext; 
}

fn sbox(data: u8, key: u8, left: u8, right: u8) -> u8
{
    let xorright = key ^ data; 

    let xorleft = (xorright >> 4) & 0b00001111; 
    let xorright = xorright & 0b00001111;

    let lrow = (((xorleft >> 2) & 0b00000010) | (xorleft & 0b00000001)) & 0b000000011;
    let lcol = (((xorleft >> 1) & 0b00000010) | ((xorleft >> 1) & 0b00000001)) & 0b000000011;

    let rrow = (((xorright >> 2) & 0b00000010) | (xorright & 0b00000001)) & 0b000000011;
    let rcol = (((xorright >> 1) & 0b00000010) | ((xorright >> 1) & 0b00000001)) & 0b000000011;

    let s0val = S0[(lcol + 4 * lrow) as usize]; 
    let s1val = S1[(rcol + 4 * rrow) as usize]; 
    let sval = (s0val << 2 | s1val) & 0b00001111; 

    let permsval = permute(sval, &P4, P4.len()) as u8;
    let xor2 = left ^ permsval;
    
    let fin = (right << 4) | xor2;

    return fin;
}
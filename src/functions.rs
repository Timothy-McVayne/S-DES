use std::fs;
use std::fs::File;
use std::io::Read; 
use std::path::Path;

//Prebaked constants for the different permutations that are done on the bytes
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


//generic permutation function that lets you rearrange bits in a byte based on an order you feed it. Converts to u16
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

//Function to generate the 2 subkeys from the 1 10 bit key provided by user.
fn generate_keys(path: &str) -> (u8, u8)
{
    //Extract key from file and convert to string
    let path = path.trim(); 
    let key = fs::read_to_string(path).expect("Key file not found!");
    let parsed_key = u16::from_str_radix(&key, 2).expect("Invalid"); 

    //Permute based on the first round described by algorithm
    let permuted = permute(parsed_key, &KEYORD10, KEYORD10.len());

    //split into left and right 5 bit halves
    let left: u8 = ((permuted >> 5) & 0b00011111) as u8;
    let right: u8 = (permuted & 0b00011111) as u8;  

    //Do the first bit rotation on the halves
    let rotleft = ((left << 1) | (left >> 4)) & 0b00011111;
    let rotright = ((right << 1) | (right >> 4)) & 0b00011111;

    //Combine rotated values and create first key by permuting 
    let combined: u16 = ((rotleft as u16) << 5) | (rotright as u16) & 0b0000000011111111;
    let k1 = permute(combined, &KEYORD8, KEYORD8.len()) as u8;

    //Rotate 2 more bits
    let rotleft = ((left << 3) | (left >> 2)) & 0b00011111;
    let rotright = ((right << 3) | (right >> 2)) & 0b00011111;

    //combine halves and create second key based on permutation order again. 
    let combined: u16 = (((rotleft as u16) << 5) | (rotright as u16)) & 0b0000000011111111;
    let k2 = permute(combined, &KEYORD8, KEYORD8.len()) as u8;

    return (k1, k2); 
}

pub fn encrypt(plain: &str, key: &str)
{
    let (k1, k2) = generate_keys(key); 

    //Open file and read bytes of file into buffer, each byte will need to be encrypted
    let mut file = File::open(plain).expect("Plaintext file not found!");
    let metadata = file.metadata().expect("Can't read metadata");
    let mut buffer: Vec<u8> = vec![0; metadata.len() as usize];
    file.read(&mut buffer).expect("Can't read file");

    //Create new vector that will hold the encrypted bytes as a copy of main buffer
    let mut cipher_text: Vec<u8> = Vec::new();
    for bytes in &buffer
    {
        cipher_text.push(*bytes);
    }

    //For each byte replace it with the encrypted byte
    for i in 0..cipher_text.len()
    {
        cipher_text[i] = encrypt_loop(cipher_text[i], k1, k2);
    }

    //Create the name of the encrypted file using the file type of the plaintext 
    let path = format!("cipher.{}", Path::new(plain).extension().unwrap().to_str().unwrap());

    //write to new file, delete old file, and print confirmation
    fs::write(path, cipher_text).unwrap();
    fs::remove_file(plain).unwrap(); 
    println!("File has been succesfully encrypted!");
}

pub fn decrypt(cipher: &str, key: &str)
{
    let (k1, k2) = generate_keys(key); 

    //Similar to encryption, get file information, put it into a buffer, copy to new buffer, and run decryption on each byte
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

    //Create basic filename, create file, remove old one, and print confirmation 
    let path = format!("plain.{}", Path::new(cipher).extension().unwrap().to_str().unwrap());

    fs::write(path, plain_text).unwrap();
    fs::remove_file(cipher).unwrap(); 
    println!("File has been succesfully decrypted!");
}

//Main loop
fn encrypt_loop(data: u8, k1: u8, k2: u8) -> u8
{
    //Perform the initial permutation step
    let permuted = permute(data, &IP, IP.len()) as u8;

    //Split into left and right values, and expand the right value based on the order from the algorithm
    let left = (permuted >> 4) & 0b00001111; 
    let right = permuted & 0b00001111;
    let expright = permute(right, &EP, EP.len()) as u8;

    //Perform the first round of sbox selection
    let intermediate = sbox(expright, k1, left, right);

    //Split new value from sbox selection, and expand right again based on order
    let left = (intermediate >> 4) & 0b00001111;
    let right = intermediate & 0b00001111;
    let expright = permute(right, &EP, EP.len()) as u8;

    let intermediate = sbox(expright, k2, left, right);

    //Take final sbox value and swap the left and right sides, then permute based on the inverse of IP
    let preinverse = (intermediate >> 4) | ((intermediate << 4) & 0b11110000);
    let ciphertext = permute(preinverse, &INVIP, INVIP.len()) as u8;
    
    return ciphertext; 
}

//Same as encryption loop function, but uses the keys in opposite order to reverse the operations
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

//Function to perform sbox operations
fn sbox(data: u8, key: u8, left: u8, right: u8) -> u8
{
    //XOR the data, which is from the right side of the permutation, with the desired key
    let xorright = key ^ data; 

    let xorleft = (xorright >> 4) & 0b00001111; 
    let xorright = xorright & 0b00001111;

    //use the first and fourth bit as the decimal row, and second and third bit as the decimal column of the sbox to extract the value
    let lrow = (((xorleft >> 2) & 0b00000010) | (xorleft & 0b00000001)) & 0b000000011;
    let lcol = (((xorleft >> 1) & 0b00000010) | ((xorleft >> 1) & 0b00000001)) & 0b000000011;

    //same as with the left half 
    let rrow = (((xorright >> 2) & 0b00000010) | (xorright & 0b00000001)) & 0b000000011;
    let rcol = (((xorright >> 1) & 0b00000010) | ((xorright >> 1) & 0b00000001)) & 0b000000011;

    //Extract values from the different boxes using the values from above as indices, combine into 1
    let s0val = S0[(lcol + 4 * lrow) as usize]; 
    let s1val = S1[(rcol + 4 * rrow) as usize]; 
    let sval = (s0val << 2 | s1val) & 0b00001111; 

    //Permute based on specified order, and XOR the permutation with the left half of the data we used the right half from
    let permsval = permute(sval, &P4, P4.len()) as u8;
    let xor2 = left ^ permsval;
    
    //Make the original right half the left half and the XOR'd value the right
    let fin = (right << 4) | xor2;

    return fin;
}
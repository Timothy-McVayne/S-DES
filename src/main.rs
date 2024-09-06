mod functions; 

use bitvec::vec::BitVec;
use std::fs::{self, File};
use std::io::Read;
use std::io;
use std::string::String; 
fn main() {

    let mut file = File::open("test.txt").expect("Can't open file");
    let metadata = fs::metadata("test.txt").expect("Can't read metadata"); 
    
    let mut buffer = vec![0; metadata.len() as usize];
    file.read(&mut buffer).expect("buffer overflow");

    //let mut value = buffer[0];
    let mut value: u8 = 0b10110010;
    let mut rearranged: u8 = 0; 

    println!("{:08b}", value); 

    let mut arr = [1, 5, 2, 0, 3, 7, 4, 6]; 

    for i in 0..8 
    {
        rearranged |= ((value >> (8 - arr[i] - 1)) & 1) << ((arr.len() - 1) - i);
    };

    println!("{:08b}", rearranged);

    functions::print_hello(); 
    
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

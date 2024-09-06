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

    let mut i = 0; 

    while i < buffer.len()
    {
        let letter = format!("{:08b}", buffer[i]);
        println!("{}", letter);
        i += 1;
    }

    //io::copy(&mut file, &mut vec);

    //let size = BitVec::len(&vec); 
    //println!("{}", size);
}

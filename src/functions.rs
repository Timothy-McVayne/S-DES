const IP: [u8; 8] = [1, 5, 2, 0, 3, 7, 4, 6];

pub fn print_hello()
{
    println!("Hello world!");

    for element in IP
    {
        print!("{}", element); 
    }
}
use std::io::{Read, StdinLock};
use std::time::Duration;
use brainpipe::libbrainpipe::map_brainpipe;
fn main() {
    let mut args = std::env::args();
    let path = args.nth(1).expect("a bf file path should be given as the first argument");
    // let flags = args.next();
    let file = std::fs::read_to_string(path).expect("the bf file should be valid and existent and accessible");
    let mut program_chars = file.chars().peekable();
    let stdin = std::io::stdin().lock(); 

    let mut main_iterator = map_brainpipe(&mut program_chars, Box::new(StdinByteIterator(stdin)));
    while let Some(out) = main_iterator.next() {
        if ! (out.is_ascii_graphic() || out.is_ascii_whitespace()) { 
        print!("§{};",out);
        } else {
        print!("{}",char::from_u32(out as u32).unwrap());
        }
    }  
    println!(" -Program ended.");
}
struct StdinByteIterator<'a>(StdinLock<'a>);
impl<'a> Iterator for StdinByteIterator<'a> {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        let mut byte = [0u8;1];
        let mut have_read_char = 0;
        while have_read_char == 0 {
            std::thread::sleep(Duration::new(0,1_000_000_000/60));
            have_read_char = self.0.read(&mut byte).unwrap_or(0); // when a char is read, this is 1.
        }
        Some(byte[0])
    }
}
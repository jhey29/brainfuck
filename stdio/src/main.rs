use std::io::{Read, StdinLock};
use std::time::Duration;
use libbrainpipe::{map_brainpipe, MyOptions};
fn main() {
    let full_args = std::env::args().collect::<Vec<_>>();
    let (progname, args) = full_args.split_first().expect("arguments!");
    let mut opts = getopts::Options::new();
    opts.optopt("z", "eof", "Decimal number of the byte to return from the comma command if there is no input to the relevant subunit.", "Number")
        .optflag("h", "help", "Display the help about the parameters.");
    let matches = match opts.parse(args) {
        Ok(matches) => matches,
        Err(fail) => {print!("{:?}",fail);return;}, 
    };
    if matches.opt_present("h") {
        let brief = format!("Usage: {} FILE [options]", progname);
        print!("{}", opts.usage(&brief) );
        return;
    }
    //// end getopt boilerplate 


    let path = matches.free.get(0).expect("a bf file path should be given as the first free argument");
    let file = match std::fs::read_to_string(path) {
        Ok(f) => f,
        Err(err) => {print!("the bf file should be valid and existent and accessible, Rust says: {:?} on reading path {path}",err); return}
    };
    let mut program_chars = file.chars().peekable();
    let stdin = std::io::stdin().lock(); 

    let mut main_iterator = map_brainpipe(&mut program_chars, Box::new(StdinByteIterator(stdin)), my_println as fn(String), MyOptions::from(matches) );
    while let Some(out) = main_iterator.next() {
        if ! (out.is_ascii_graphic() || out.is_ascii_whitespace()) { 
        print!("§{};",out);
        } else {
        print!("{}",char::from_u32(out as u32).unwrap());
        }
    }  
    println!(" -Program ended.");
}

fn my_println(s: String) {
    println!("{}",s);
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
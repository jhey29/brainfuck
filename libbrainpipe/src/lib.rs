use std::iter::Peekable;
use getopts;
use bf::{bf_to_ast, BrainfuckIterator};
pub mod bf;

///Options for Brainpipe execution.
#[derive(Clone, Copy)]
pub struct MyOptions {
    /// The -z flag, which specifies the Number to send if there is no input, instead of terminating the program.
    pub eof: Option<u8>, 
}
/// Cross-Platform getopt boilerplate 
pub fn parse_options(args: &[String], progname: &str, output: fn(String)) -> Result<getopts::Matches,()> {
    let mut opts = getopts::Options::new();
    opts.optopt("z", "eof", "Decimal number of the byte to return from the comma command if there is no input to the relevant subunit.", "Number")
        .optflag("h", "help", "Display the help about the parameters.");
    let matches = opts.parse(args).map_err(|fail| output(format!("{:?}",fail)))?;
    if matches.opt_present("h") {
        let brief = format!("Usage: {} FILE [options]", progname);
        return Err(output(opts.usage(&brief)));
    }
    Ok(matches)
}
impl From<getopts::Matches> for MyOptions {
    fn from(value: getopts::Matches) -> Self {
        MyOptions {
            eof: {
                if let Some(x) = value.opt_str("z") {
                    x.parse().ok()
                } else {None}
            }
        }
    }
}

pub use crate as libbrainpipe;
pub fn map_brainpipe<'a, I: Iterator<Item = char>>(program_iter: &mut Peekable<I>, input_iter: Box<dyn Iterator<Item = u8> + 'a>, 
debug: fn(String), options: MyOptions) -> Box<dyn Iterator<Item = u8> + 'a> {
    let mut which_on = input_iter;
    loop {
        let bf_ast = bf_to_ast(program_iter); 
        which_on = Box::new(BrainfuckIterator::new(bf_ast, which_on, debug, options)); 
        match program_iter.next() {
            Some('|') => continue,
            None => break,
            Some(x) => panic!("got {x}")
        }
    }
    which_on
}

#[cfg(test)]
mod tests {
    use std::iter::once;
    use super::*;
    
    #[test]
    fn bpipe_basic() {
        let resulting_iterator: Box<_> = map_brainpipe(&mut ",++.|,[->++<]>.".chars().peekable(), Box::new(once(23)), drop as fn(String), MyOptions {eof: None});
        for op in resulting_iterator {
            dbg!(op);
        }
    }
}
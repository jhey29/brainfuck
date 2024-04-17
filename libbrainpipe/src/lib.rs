use std::{iter::Peekable, str::Chars};

use bf::{bf_to_ast, BrainfuckIterator};

pub mod bf;

pub use crate as libbrainpipe;
pub fn map_brainpipe<'a>(program: &mut Peekable<Chars>, input_iterator: Box<dyn Iterator<Item = u8> + 'a>) -> Box<dyn Iterator<Item = u8> + 'a> {
    let mut which_on = input_iterator;
    loop {
        let bf_ast = bf_to_ast(program); 
        which_on = Box::new(BrainfuckIterator::new(bf_ast, which_on)); 
        match program.next() {
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
        let resulting_iterator: Box<_> = map_brainpipe(&mut ",++.|,[->++<]>.".chars().peekable(), Box::new(once(23)));
        for op in resulting_iterator {
            dbg!(op);
        }
    }
}
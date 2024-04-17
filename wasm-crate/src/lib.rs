use libbrainpipe;
use wasm_bindgen;


use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    // fn alert(s: &str);
    // async fn read_byte() -> u8; 
    //TODO: Reading the input while the user is allowed to modify it might require async
    // and might also need the Iterators next() to be async, which does not work.
    // oooor the run function has to do some magic while consuming the BrainfuckIterator.
    // fn display_char(c: char);
}

#[wasm_bindgen]
pub fn run(program: &str, input: &str) -> String {
    let input = input.to_owned();
    let iter = libbrainpipe::map_brainpipe(
        &mut program.chars().peekable(),
        Box::new(input.chars().map(|c| u32::from(c) as u8 ))
    );
    let mut string = String::new();
    string.extend(iter.map(|byte| byte as char));
    string
}

struct MyInputIterator;

impl Iterator for MyInputIterator {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
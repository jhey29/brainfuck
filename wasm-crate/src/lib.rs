
use libbrainpipe::{self, parse_options, MyOptions};
use wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/wasm_imports.js")]
extern {
    // fn alert(s: &str);
    // async fn read_byte() -> u8; 
    //TODO: Reading the input while the user is allowed to modify it might require async
    // and might also need the Iterators next() to be async, which does not work.
    // oooor the run function has to do some magic while consuming the BrainfuckIterator.
    fn output(c: String);
}


#[wasm_bindgen]
pub fn run(program: &str, input: &str, options: &str) {
    console_error_panic_hook::set_once();
    let input = input.to_owned();
    let Ok(options) = parse_options(&[options.to_owned()],"cargo run --" , output) else {return};
    let iter = libbrainpipe::map_brainpipe(
        &mut program.chars().peekable(),
        Box::new(input.chars().map(|c| u32::from(c) as u8 )),
        output as fn(String),
        MyOptions::from(options)//TODO
    );
    
    iter.for_each(|byte| output(String::from(byte as char)));
}

struct MyInputIterator;

impl Iterator for MyInputIterator {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
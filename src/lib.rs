use std::str::Chars;
use std::iter::Peekable;
use std::io::Read;
use std::time::Duration;
#[derive(Debug)]
pub enum BfAstNode {
    CodeBlob(Vec<char>),
    Conditional(Vec<BfAstNode>),
}
pub fn run_bf_ast(program: &Vec<BfAstNode>, memory:&mut Vec<u8>,mix: &mut usize,stdin:&mut impl Read ) {
    for bfn in program {
        match bfn {
            BfAstNode::CodeBlob(vec_ch) => {
                for c in vec_ch {
                    match c {
                        '+' => memory[*mix] = memory[*mix].wrapping_add(1),
                        '-' => memory[*mix] = memory[*mix].wrapping_sub(1),
                        '<' => {*mix = (*mix).checked_sub(1).expect("< should not underflow the memory index");},
                        '>' => {*mix += 1; if *mix == memory.len() {
                            memory.push(0)
                        }},
                        '.' => {if ! (memory[*mix].is_ascii_graphic() || memory[*mix].is_ascii_whitespace()) { //the only control code that does anything is beep. yay
                            print!("§{mix}:{};",memory[*mix]);
                        } else {
                            print!("{}",char::from_u32(memory[*mix] as u32).unwrap())
                        }},
                        ',' => {
                            let mut byte = [0u8;1];
                            let mut have_read_char = 0;
                            while have_read_char == 0 {
                                std::thread::sleep(Duration::new(0,1_000_000_000/60));
                                have_read_char = stdin.read(&mut byte).unwrap_or(0); // when a char is read, this is 1.
                            }
                            memory[*mix] = byte[0]
                        },
                        '#' => {
                            let plsprint = &memory.into_iter().map(|v| {let mut v: Vec<String> = (*v as i8).to_string().chars().map(|c| c.to_string()).collect::<Vec<String>>(); 
                                v.reverse(); v.push(" ".into()); v.truncate(2); v.reverse(); v.concat()}
                                ).collect::<Vec<String>>();
                            println!("{:?}", plsprint);
                            std::thread::sleep(Duration::new(0,1_000_000_000/60));
                        }
                        _ => (),
                    }
                }
            }
            BfAstNode::Conditional(vec_ast) => {
                while memory[*mix] != 0 {
                    run_bf_ast(vec_ast,memory,mix,stdin)
                }
            }
        }
    }
}
pub fn bf_to_ast(program: &mut Peekable<Chars>) -> Vec<BfAstNode> {
    let mut opvec_ast = Vec::new();
    while let Some(c) = program.peek() {
        match c {
            '[' => {program.next();  opvec_ast.push(BfAstNode::Conditional(bf_to_ast(program)))},
            ']' => {program.next();  break},
            a if "+-.,<>#".contains(*a) => opvec_ast.push(BfAstNode::CodeBlob(new_code_blob(program))),
            _ => {program.next();},
        }
    }
    opvec_ast
}
fn new_code_blob(program: &mut Peekable<Chars>) -> Vec<char> {
    let mut opvec_ch : Vec<char> = Vec::new();
    loop {
        match program.peek() {
            Some(c) => match c {
                '[' | ']' => break,
                a if "+-.,<>#".contains(*a) => opvec_ch.push(program.next().unwrap()),
                _ => {program.next();},
            }
            None => break,
        }
    }
    opvec_ch
}
#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn look_at_tree() {
        dbg!(bf_to_ast(&mut "->>>>>+[-.<+]-".chars().peekable()));
    }
}

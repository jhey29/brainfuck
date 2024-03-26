use std::str::Chars;
use std::iter::Peekable;
use std::io::Read;
use std::time::Duration;
#[derive(Debug)]
pub enum BfAstNode {
    CodeBlob(Vec<char>),
    Conditional(Vec<BfAstNode>),
}
pub struct BrainfuckGenerator {
    program: Vec<BfAstNode>,
    memory: Vec<u8>,
    mem_ix: usize,
    
}
pub struct Pipe {
    car: BrainfuckIterator,
    cdr: Box<Pipe>,
}
pub fn run_bf_ast(program: &Vec<BfAstNode>, memory:&mut Vec<u8>,mix: &mut usize,stdin:&mut impl Read ) {
    for bfn in program {
        match bfn {
            BfAstNode::CodeBlob(vec_ch) => {
                let mut chars = vec_ch.iter();
                loop {
                match chars.next() {
                    Some(c) => match c {
                        '+' => memory[*mix] = memory[*mix].wrapping_add(1),
                        '-' => memory[*mix] = memory[*mix].wrapping_sub(1),
                        '<' => *mix = match (*mix).checked_sub(1) {
                            Some(v) => v, 
                            None => {
                                println!("{:?}",&memory);
                                panic!("< should not underflow the memory index in code block {:?}",vec_ch)}
                            ,
                        },
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
                            println!("message: {}",chars.as_slice().iter().take_while(|ch| !".,+-<>".contains(**ch) ).map(|c|c.to_string()).reduce(|a,b| a+ &b ).unwrap_or("".into()));
                            println!("{:?}",&memory);
                            std::thread::sleep(Duration::new(0,1_000_000_000/60));
                        },
                        _ => (),
                    },
                    None => break,
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
                '#' => opvec_ch.append(&mut program.take_while(|ch| !ch.is_ascii_whitespace()).collect::<Vec<_>>() ),
                a if "+-.,<>".contains(*a) => opvec_ch.push(program.next().unwrap()),
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

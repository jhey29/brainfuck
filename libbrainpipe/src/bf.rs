use std::iter::Peekable;
use std::time::Duration;
#[derive(Debug)]
pub enum BfAstNode {
    CodeBlob(SelectVec<char>),
    Conditional(SelectVec<BfAstNode>),
}
pub struct BrainfuckIterator<'a> {
    program: SelectVec<BfAstNode>,
    memory: SelectVec<u8>,
    on: Box<dyn Iterator<Item = u8> + 'a>,
}
impl<'a> BrainfuckIterator<'a> {
    pub fn new(program: SelectVec<BfAstNode>,on: Box<dyn Iterator<Item = u8> + 'a>) -> Self {
        BrainfuckIterator {
            program,
            memory: SelectVec::new(vec![0,0,0,0]),
            on,
        }
    }
}
impl<'a> Iterator for BrainfuckIterator<'a> {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        match run_bf_ast(&mut self.program,&mut self.memory,&mut self.on) {
            BfResult::Output(x) => Some(x),
            BfResult::OutOfInput | BfResult::Terminated => None,
        }
    }
}


#[derive(Debug)]
pub struct SelectVec<T> {
    vec: Vec<T>,
    selected: usize,
}
impl<T> SelectVec<T> {
    fn new(vec: Vec<T>) -> Self {
        SelectVec { vec, selected: 0 }
    }
    fn select_next(&mut self) {
        self.selected += 1
    } 
    fn get_mut_selected(&mut self) -> Option<&mut T> {
        self.vec.get_mut(self.selected)
    }  
    fn reset(&mut self) {
        self.selected = 0
    }
}
impl<T: Default> SelectVec<T> {
    //Automatically grows Vec with default if index too big
    fn mut_selected(&mut self) -> &mut T {
        if self.selected >= self.vec.len() {
            self.vec.resize_with(self.selected + 1, Default::default); 
            self.vec.get_mut(self.selected).unwrap()
        } else {
            self.vec.get_mut(self.selected).unwrap()
        }
    }
}
trait IsReset {
    fn is_reset(&self) -> bool;
}
impl<T: IsReset> IsReset for SelectVec<T> {
    fn is_reset(&self) -> bool {
        self.selected == 0 && self.vec.iter().all(
           |item| item.is_reset()
        )
    }
}
impl IsReset for char {
    fn is_reset(&self) -> bool {
        true    
    }
}
impl IsReset for BfAstNode {
    fn is_reset(&self) -> bool {
        match self {
            BfAstNode::CodeBlob(select_char) => select_char.is_reset(),
            BfAstNode::Conditional(select_bfn) => select_bfn.is_reset(),
        }
    }
}
/// Clarifies why `run_bf_ast` has returned, which is mostly useful to itself, as it is recursive.
pub enum BfResult {
    Output(u8),
    Terminated,
    OutOfInput,
}
pub fn run_bf_ast(program: &mut SelectVec<BfAstNode>, memory:&mut SelectVec<u8>, input:&mut impl Iterator<Item = u8>) -> BfResult {
    loop {
        let Some(node) = program.get_mut_selected() else {return BfResult::Terminated};
        match node {
            BfAstNode::CodeBlob(ref mut chars) => {
                loop {
                    let char = chars.get_mut_selected().copied();
                    chars.select_next(); //unlike the above loop, if something quits out of this loop
                                         //it's always correct to have the next command selected
                    match char {
                        Some(c) => match c {
                            '+' => *memory.mut_selected() = memory.mut_selected().wrapping_add(1),
                            '-' => *memory.mut_selected() = memory.mut_selected().wrapping_sub(1),
                            '<' => memory.selected = match (memory.selected).checked_sub(1) {
                                Some(v) => v, 
                                None => {
                                    println!("{:?}",&memory);
                                    panic!("< should not underflow the memory index in code block {:?}",chars)
                                },
                            },
                            '>' => memory.selected += 1,
                            '.' => return BfResult::Output(*memory.mut_selected()),
                            ',' => match input.next() { 
                                Some(byte) => *memory.mut_selected() = byte,
                                None => return BfResult::OutOfInput, // if you have returned None, no one will ever ask you again because everyone is like you
                            },
                            '#' => {
                                println!("message: {}",chars.vec[chars.selected..].iter().take_while(|ch| !".,+-<>".contains(**ch) ).map(|c|c.to_string()).reduce(|a,b| a+ &b ).unwrap_or("".into()));
                                println!("{:?}",&memory);
                                std::thread::sleep(Duration::new(0,1_000_000_000/60));
                            },
                            _ => (),
                        },
                        None => {chars.reset(); break},
                    }
                }
            },
            BfAstNode::Conditional(ref mut ast) => {
                loop { 
                    if *memory.mut_selected() == 0 && ast.is_reset() { break; } //is_reset => Don't do this check when restarting the simulation, only when looping.
                    match run_bf_ast(ast,memory,input) {
                        BfResult::Output(output) => return BfResult::Output(output), //if the contents of a loop have generated output, we need to remember our place.
                        BfResult::OutOfInput => return BfResult::OutOfInput,
                        BfResult::Terminated => ast.reset(), // if the contents of a loop have terminated, they can run again
                    }
                }
            }
        }
        program.select_next()
    }
}
pub fn bf_to_ast<I>(program: &mut Peekable<I>) -> SelectVec<BfAstNode>
where I: Iterator<Item = char> {
    let mut opvec_ast = Vec::new();
    while let Some(c) = program.peek() {
        match c {
            '[' => {program.next();  opvec_ast.push(BfAstNode::Conditional(bf_to_ast(program)))},
            ']' => {program.next();  break},
            a if "+-.,<>#".contains(*a) => opvec_ast.push(BfAstNode::CodeBlob(new_code_blob(program))),
            bpipe_char if "|(){}".contains(*bpipe_char) => break, 
            _ => {program.next();},
        }
    }
    SelectVec::new(opvec_ast)
}
fn new_code_blob<I>(program: &mut Peekable<I>) -> SelectVec<char>
where I: Iterator<Item = char> {
    let mut opvec_ch : Vec<char> = Vec::new();
    loop {
        match program.peek() {
            Some(c) => match c {
                '[' | ']' => break,
                bpipe_char if "|(){}".contains(*bpipe_char) => break,
                '#' => opvec_ch.append(&mut program.take_while(|ch| !ch.is_ascii_whitespace()).collect::<Vec<_>>() ),
                a if "+-.,<>".contains(*a) => opvec_ch.push(program.next().unwrap()),
                _ => {program.next();},
            }
            None => break,
        }
    }
    SelectVec::new(opvec_ch)
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn look_at_tree() {
        dbg!(bf_to_ast(&mut "->>>>>+[-.<+]-".chars().peekable()));
    }
}

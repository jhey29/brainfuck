use brainfuck::*;
fn main() {
    let mut args = std::env::args();
    let path = args.nth(1).expect("a bf file path should be given as the first argument");
    // let flags = args.next();
    let borrow_checker_dopamine = std::fs::read_to_string(path).expect("the bf file should be valid and existent and accessible");
    let mut program_chars = borrow_checker_dopamine.chars().peekable();

    let mut memory: Vec<u8> = vec![0,0,0,0];
    let mut mix: usize = 0;
    let mut stdin = std::io::stdin().lock(); 
    run_bf_ast(&bf_to_ast(&mut program_chars),&mut memory,&mut mix, &mut stdin);
    
    println!(" -Program ended.");
}

use luar::{lexer::Lexer, parser, vm::ExeState};
use std::{env, fs::File, io::Read};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} script", args[0]);
        return;
    }

    let mut file = File::open(&args[1]).unwrap();
    let mut src = String::new();
    file.read_to_string(&mut src).unwrap();
    let lex = Lexer::new(src);
    let proto = parser::load(lex);
    ExeState::new().execute(&proto);
}

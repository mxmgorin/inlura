// The syntax analysis module

use crate::{
    bytecode::Bytecode,
    lexer::{Lexer, Token},
    lvalue::LValue,
};

pub fn load(mut lex: Lexer) -> ParseProto {
    let mut constants = Vec::new();
    let mut byte_codes = Vec::new();

    loop {
        match lex.next() {
            Token::Name(name) => {
                // `Name LiteralString` as function call
                constants.push(LValue::String(name));
                byte_codes.push(Bytecode::GetGlobal(0, (constants.len() - 1) as u8));

                if let Token::String(s) = lex.next() {
                    constants.push(LValue::String(s));
                    byte_codes.push(Bytecode::LoadConst(1, (constants.len() - 1) as u8));
                    byte_codes.push(Bytecode::Call(0, 1));
                } else {
                    panic!("expected string");
                }
            }
            Token::Eos => break,
            t => panic!("unexpected token: {t:?}"),
        }
    }

    dbg!(&constants);
    dbg!(&byte_codes);

    ParseProto {
        constants,
        byte_codes,
    }
}

#[derive(Debug)]
pub struct ParseProto {
    pub constants: Vec<LValue>,
    pub byte_codes: Vec<Bytecode>,
}

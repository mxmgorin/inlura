// The syntax analysis module

use crate::{
    bytecode::Bytecode, lexer::{Lexer, Token}, value::Value
};

pub fn load(mut lex: Lexer) -> ParseProto {
    let mut constants = Vec::new();
    let mut byte_codes = Vec::new();

    loop {
        match lex.next() {
            Token::Name(name) => {
                // function, global variable only
                let ic = add_const(&mut constants, Value::String(name));
                byte_codes.push(Bytecode::GetGlobal(0, ic as u8));

                // argument, (var) or "string"
                match lex.next() {
                    Token::ParLeft => {
                        // '('
                        let code = match lex.next() {
                            Token::Nil => Bytecode::LoadNil(1),
                            Token::True => Bytecode::LoadBool(1, true),
                            Token::False => Bytecode::LoadBool(1, false),
                            Token::ConstInteger(i) => {
                                if let Ok(ii) = i16::try_from(i) {
                                    Bytecode::LoadInt(1, ii)
                                } else {
                                    load_const(&mut constants, 1, Value::Integer(i))
                                }
                            }
                            Token::ConstFloat(f) => load_const(&mut constants, 1, Value::Float(f)),
                            Token::ConstString(s) => {
                                load_const(&mut constants, 1, Value::String(s))
                            }
                            _ => panic!("invalid argument"),
                        };

                        byte_codes.push(code);

                        if lex.next() != Token::ParRight {
                            // ')'
                            panic!("expected `)`");
                        }
                    }
                    Token::ConstString(s) => {
                        let code = load_const(&mut constants, 1, Value::String(s));
                        byte_codes.push(code);
                    }
                    _ => panic!("expected string"),
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

fn load_const(constants: &mut Vec<Value>, dst: usize, c: Value) -> Bytecode {
    Bytecode::LoadConst(dst as u8, add_const(constants, c) as u8)
}

fn add_const(constants: &mut Vec<Value>, c: Value) -> usize {
    constants.iter().position(|v| v == &c).unwrap_or_else(|| {
        constants.push(c);
        constants.len() - 1
    })
}
#[derive(Debug)]
pub struct ParseProto {
    pub constants: Vec<Value>,
    pub byte_codes: Vec<Bytecode>,
}

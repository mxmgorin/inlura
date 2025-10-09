use crate::{bytecode::Bytecode, value::Value, parser::ParseProto};
use std::collections::HashMap;

pub struct ExeState {
    globals: HashMap<String, Value>,
    stack: Vec<Value>,
}

impl ExeState {
    pub fn new() -> Self {
        let mut globals = HashMap::new();
        globals.insert(String::from("print"), Value::Function(lib_print));

        Self {
            globals,
            stack: Vec::new(),
        }
    }

    pub fn execute(&mut self, proto: &ParseProto) {
        for code in proto.byte_codes.iter() {
            match *code {
                Bytecode::GetGlobal(dst, name) => {
                                let name = &proto.constants[name as usize];
                                if let Value::String(key) = name {
                                    let v = self.globals.get(key).unwrap_or(&Value::Nil).clone();
                                    self.set_stack(dst, v);
                                } else {
                                    panic!("invalid global key: {name:?}");
                                }
                            }
                Bytecode::LoadConst(dst, c) => {
                                let v = proto.constants[c as usize].clone();
                                self.set_stack(dst, v);
                            }
                Bytecode::Call(func, _) => {
                                let func = &self.stack[func as usize];
                                if let Value::Function(f) = func {
                                    f(self);
                                } else {
                                    panic!("invalid function: {func:?}");
                                }
                            }
                Bytecode::LoadNil(_) => todo!(),
                Bytecode::LoadBool(_, _) => todo!(),
                Bytecode::LoadInt(_, _) => todo!(),
            }
        }
    }

    pub fn set_stack(&mut self, dst: u8, v: Value) {
        let index = dst as usize;

        if index >= self.stack.len() {
            self.stack.resize(index + 1, Value::Nil);
        }

        self.stack[index] = v;
    }
}

// "print" function in Lua's std-lib.
// It supports only 1 argument and assumes the argument is at index:1 on stack.
pub fn lib_print(state: &mut ExeState) -> i32 {
    println!("{:?}", state.stack[1]);
    0
}

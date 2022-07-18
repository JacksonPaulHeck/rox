use crate::*;

pub enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRuntimeError,
}

pub struct VM {
    chunk: chunk::Chunk,
    ip: Box<u8>,
    stack: Vec<value::Value>,
    stack_top: usize,
    globals: table::Table,
    strings: table::Table,
    objects: Box<object::Obj>,
}

impl VM {
    pub fn new() -> VM {
        return VM {
            chunk: chunk::Chunk::new(),
            ip: Box::new(0 as u8),
            stack: Vec::new(),
            stack_top: 0,
            globals: table::Table::new(),
            strings: table::Table::new(),
            objects: Box::new(object::Obj::new()),
        };
    }

    fn byte_instruction(&self, name: &str, offset: i64) -> i64 {
        let slot = self.chunk.get_code()[offset as usize + 1];
        println!("{} {}", name, slot);
        return offset + 2;
    }

    fn simple_instruction(&self, name: &str, offset: i64) -> i64 {
        println!("{}", name);
        return offset + 1;
    }

    fn constant_instruction(&self, name: &str, offset: i64) -> i64 {
        let constant: u8 = self.chunk.get_code()[offset as usize + 1];
        println!(
            "{} {} {:#?}",
            name,
            constant,
            self.chunk.get_constants().get_values()[constant as usize].print()
        );
        return offset + 2;
    }

    fn jump_instruction(&self, name: &str, sign: i64, offset: i64) -> i64 {
        let mut jump: u16 = (self.chunk.get_code()[offset as usize + 1] as u16) << 8;
        jump |= (self.chunk.get_code()[offset as usize + 2]) as u16;
        println!("{} {} -> {}", name, offset, offset + 3 + sign * jump as i64);
        return offset + 3;
    }

    fn disassemble_instruction(&self, mut offset: i64, set_offset: bool) -> i64 {
        if !set_offset {
            offset = (*self.ip - self.chunk.get_code()[0]) as i64;
        }
        print!("{}\t", offset);

        if offset > 0
            && self.chunk.get_lines()[offset as usize]
                == self.chunk.get_lines()[offset as usize - 1]
        {
            print!("| ");
        } else {
            print!("{} ", self.chunk.get_lines()[offset as usize]);
        }

        let instruction: chunk::OpCode = self.chunk.get_code()[offset as usize].into();
        match instruction {
            chunk::OpCode::OpConstant => return self.constant_instruction("OpConstant", offset),
            chunk::OpCode::OpNil => return self.simple_instruction("OpNil", offset),
            chunk::OpCode::OpTrue => return self.simple_instruction("OpTrue", offset),
            chunk::OpCode::OpFalse => return self.simple_instruction("OpFalse", offset),
            chunk::OpCode::OpPop => return self.simple_instruction("OpPop", offset),
            chunk::OpCode::OpGetLocal => return self.byte_instruction("OpGetLocal", offset),
            chunk::OpCode::OpSetLocal => return self.byte_instruction("OpSetLocal", offset),
            chunk::OpCode::OpGetGlobal => return self.constant_instruction("OpGetGlobal", offset),
            chunk::OpCode::OpDefineGlobal => {
                return self.constant_instruction("OpDefineGlobal", offset)
            }
            chunk::OpCode::OpSetGlobal => return self.constant_instruction("OpSetGlobal", offset),
            chunk::OpCode::OpEqual => return self.simple_instruction("OpEqual", offset),
            chunk::OpCode::OpGreater => return self.simple_instruction("OpGreater", offset),
            chunk::OpCode::OpLess => return self.simple_instruction("OpLess", offset),
            chunk::OpCode::OpAdd => return self.simple_instruction("OpAdd", offset),
            chunk::OpCode::OpSubtract => return self.simple_instruction("OpSubtract", offset),
            chunk::OpCode::OpMultiply => return self.simple_instruction("OpMultiply", offset),
            chunk::OpCode::OpDivide => return self.simple_instruction("OpDivide", offset),
            chunk::OpCode::OpNot => return self.simple_instruction("OpNot", offset),
            chunk::OpCode::OpNegate => return self.simple_instruction("OpNegate", offset),
            chunk::OpCode::OpPrint => return self.simple_instruction("OpPrint", offset),
            chunk::OpCode::OpJump => return self.jump_instruction("OpJump", 1, offset),
            chunk::OpCode::OpJumpIfFalse => {
                return self.jump_instruction("OpJumpIfFalse", 1, offset)
            }
            chunk::OpCode::OpLoop => return self.jump_instruction("OpLoop", -1, offset),
            chunk::OpCode::OpReturn => return self.simple_instruction("OpReturn", offset),
        }
    }

    fn run(&mut self) -> InterpretResult {
        loop {
            println!("          ");
            let mut slot = 0;
            while slot < self.stack_top {
                println!("[ {} ]", self.stack[slot].print());
                slot += 1;
            }

            self.disassemble_instruction(0, false);

            let instruction: chunk::OpCode = (*self.ip).into();
            *self.ip += 1;

            match instruction {
                chunk::OpCode::OpConstant => {
                    self.stack[self.stack_top] =
                        self.chunk.get_constants().get_value(*self.ip as usize);
                    self.stack_top += 1;
                    *self.ip += 1;
                    break;
                }
                chunk::OpCode::OpNil => {
                    self.stack[self.stack_top] =
                        value::Value::create(value::ValueType::ValNil, value::Union::create_num(0));
                    *self.ip += 1;
                    break;
                }
                chunk::OpCode::OpTrue => {
                    self.stack[self.stack_top] = value::Value::create(
                        value::ValueType::ValBool,
                        value::Union::create_bool(true),
                    );
                    *self.ip += 1;
                    break;
                }
                chunk::OpCode::OpFalse => {
                    self.stack[self.stack_top] = value::Value::create(
                        value::ValueType::ValBool,
                        value::Union::create_bool(false),
                    );
                    *self.ip += 1;
                    break;
                }
                chunk::OpCode::OpPop => {
                    *self.ip -= 1;
                    break;
                }
                chunk::OpCode::OpGetLocal => {
                    let slot: u8 = *self.ip;
                    self.stack[self.stack_top] = self.stack[slot as usize];
                    *self.ip += 1;
                    break;
                }
                chunk::OpCode::OpSetLocal => {
                    let slot: u8 = *self.ip;
                    self.stack[self.stack_top] = self.stack[0];
                    *self.ip += 1;
                    break;
                }
                chunk::OpCode::OpGetGlobal => {
                    let name: object::ObjString = object::ObjString::copy(self.chunk.get_constants().get_value(*self.ip as usize).get_value().get_obj());
                    self.stack_top += 1;

                    let value: value::Value = value::Value::new();
                    if !self.globals.table_get(name, value) {
                        eprintln!("RUNTIME ERROR");
                        self.stack_top = 0; 
                        return InterpretResult::InterpretRuntimeError;
                    }
                    self.stack[self.stack_top] = value;
                    break;
                },
                chunk::OpCode::OpDefineGlobal => {
                    let name: object::ObjString = object::ObjString::copy(self.chunk.get_constants().get_value(*self.ip as usize).get_value().get_obj());

                    self.chunk.print();

                    self.stack_top += 1;
                    
                    self.globals.table_set(name, self.stack[0]);

                    self.stack_top -= 1;
                },
                chunk::OpCode::OpSetGlobal => {
                    let name: object::ObjString = object::ObjString::copy(self.chunk.get_constants().get_value(*self.ip as usize).get_value().get_obj());
                    if self.globals.table_set(name, self.stack[0]) {
                        self.globals.table_delete(name);
                        eprintln!("RUNTIME ERROR");
                        return InterpretResult::InterpretRuntimeError;
                    }
                    break;
                },
                chunk::OpCode::OpEqual => {
                    let a: value::Value = self.stack[self.stack_top];
                    self.stack_top -= 1;
                    let b: value::Value = self.stack[self.stack_top];
                    self.stack_top -= 1;
                    self.stack[self.stack_top] = value::Value::create(
                        value::ValueType::ValBool,
                        value::Union::create_bool(a.equal(b)),
                    );

                    self.stack_top += 1;
                    break;
                }
                chunk::OpCode::OpGreater => {
                    if (self.stack[0].get_value_type() != value::ValueType::ValNumber)
                        || (self.stack[1].get_value_type() != value::ValueType::ValNumber)
                    {
                        return InterpretResult::InterpretRuntimeError;
                    }

                    let b = self.stack[self.stack_top];
                    self.stack_top -= 1;

                    let a = self.stack[self.stack_top];
                    self.stack_top -= 1;
                    self.stack[self.stack_top] = value::Value::create(
                        value::ValueType::ValBool,
                        value::Union::create_bool(
                            a.get_value().get_number() > b.get_value().get_number(),
                        ),
                    );
                    self.stack_top += 1;
                    break;
                }
                chunk::OpCode::OpLess => {
                    if (self.stack[0].get_value_type() != value::ValueType::ValNumber)
                        || (self.stack[1].get_value_type() != value::ValueType::ValNumber)
                    {
                        return InterpretResult::InterpretRuntimeError;
                    }

                    let b = self.stack[self.stack_top];
                    self.stack_top -= 1;

                    let a = self.stack[self.stack_top];
                    self.stack_top -= 1;
                    self.stack[self.stack_top] = value::Value::create(
                        value::ValueType::ValBool,
                        value::Union::create_bool(
                            a.get_value().get_number() < b.get_value().get_number(),
                        ),
                    );
                    self.stack_top += 1;
                    break;
                }
                chunk::OpCode::OpAdd => {
                    if (self.stack[0].get_value_type() == value::ValueType::ValNumber)
                        || (self.stack[1].get_value_type() == value::ValueType::ValNumber)
                    {
                        let b = self.stack[self.stack_top];
                        self.stack_top -= 1;

                        let a = self.stack[self.stack_top];
                        self.stack_top -= 1;
                        self.stack[self.stack_top] = value::Value::create(
                            value::ValueType::ValNumber,
                            value::Union::create_num(
                                a.get_value().get_number() + b.get_value().get_number(),
                            ),
                        );
                        self.stack_top += 1;
                    } else if ((self.stack[0].get_value_type() == value::ValueType::ValObj)
                        || (self.stack[1].get_value_type() == value::ValueType::ValObj))
                        && (self.stack[0].get_value().get_obj().get_type()
                            == object::ObjType::ObjectString)
                    {
                        todo!();
                    } else {
                        return InterpretResult::InterpretRuntimeError;
                    }
                    break;
                }
                chunk::OpCode::OpSubtract => {
                    if (self.stack[0].get_value_type() != value::ValueType::ValNumber)
                        || (self.stack[1].get_value_type() != value::ValueType::ValNumber)
                    {
                        return InterpretResult::InterpretRuntimeError;
                    }

                    let b = self.stack[self.stack_top];
                    self.stack_top -= 1;

                    let a = self.stack[self.stack_top];
                    self.stack_top -= 1;
                    self.stack[self.stack_top] = value::Value::create(
                        value::ValueType::ValNumber,
                        value::Union::create_num(
                            a.get_value().get_number() - b.get_value().get_number(),
                        ),
                    );
                    self.stack_top += 1;
                    break;
                }
                chunk::OpCode::OpMultiply => {
                    if (self.stack[0].get_value_type() != value::ValueType::ValNumber)
                        || (self.stack[1].get_value_type() != value::ValueType::ValNumber)
                    {
                        return InterpretResult::InterpretRuntimeError;
                    }

                    let b = self.stack[self.stack_top];
                    self.stack_top -= 1;

                    let a = self.stack[self.stack_top];
                    self.stack_top -= 1;
                    self.stack[self.stack_top] = value::Value::create(
                        value::ValueType::ValNumber,
                        value::Union::create_num(
                            a.get_value().get_number() * b.get_value().get_number(),
                        ),
                    );
                    self.stack_top += 1;
                    break;
                }
                chunk::OpCode::OpDivide => {
                    if (self.stack[0].get_value_type() != value::ValueType::ValNumber)
                        || (self.stack[1].get_value_type() != value::ValueType::ValNumber)
                    {
                        return InterpretResult::InterpretRuntimeError;
                    }

                    let b = self.stack[self.stack_top];
                    self.stack_top -= 1;

                    let a = self.stack[self.stack_top];
                    self.stack_top -= 1;
                    self.stack[self.stack_top] = value::Value::create(
                        value::ValueType::ValNumber,
                        value::Union::create_num(
                            a.get_value().get_number() / b.get_value().get_number(),
                        ),
                    );
                    self.stack_top += 1;
                    break;
                }

                chunk::OpCode::OpNot => todo!(),
                chunk::OpCode::OpNegate => todo!(),
                chunk::OpCode::OpPrint => {
                    self.stack[self.stack_top].print();
                    break;
                }
                chunk::OpCode::OpJump => todo!(),
                chunk::OpCode::OpJumpIfFalse => todo!(),
                chunk::OpCode::OpLoop => todo!(),
                chunk::OpCode::OpReturn => return InterpretResult::InterpretOk,
            }
        }
        return InterpretResult::InterpretOk;
    }

    pub fn interpret(
        &mut self,
        source: String,
        rules: &std::collections::HashMap<scanner::TokenType, compiler::ParseRule>,
    ) -> InterpretResult {
        let chunk: chunk::Chunk = chunk::Chunk::new();
        if !compiler::compile(source, self, &chunk, rules) {
            return InterpretResult::InterpretCompileError;
        }
        self.chunk = chunk;
        self.ip = Box::new(self.chunk.get_code()[0]);
        let result: InterpretResult = self.run();
        return result;
    }

    pub fn disassemble_chunk(&mut self, chunk: chunk::Chunk, code: &str) {
        println!("== {} ==", code);
        self.chunk = chunk; 

        let mut offset = 0;
        for mut offset in 0..chunk.get_count() {
           offset = self.disassemble_instruction(offset, true); 
        }
    }
}

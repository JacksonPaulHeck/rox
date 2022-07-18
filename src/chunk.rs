use crate::*;

#[derive(Debug)]
#[repr(u8)]
pub enum OpCode {
    OpConstant = 1,
    OpNil = 2,
    OpTrue = 3,
    OpFalse = 4,
    OpPop = 5,
    OpGetLocal = 6,
    OpSetLocal = 7,
    OpGetGlobal = 8,
    OpDefineGlobal = 9,
    OpSetGlobal = 10,
    OpEqual = 11,
    OpGreater = 12,
    OpLess = 13,
    OpAdd = 14,
    OpSubtract = 15,
    OpMultiply = 16,
    OpDivide = 17,
    OpNot = 18,
    OpNegate = 19,
    OpPrint = 20,
    OpJump = 21,
    OpJumpIfFalse = 22,
    OpLoop = 23,
    OpReturn = 24,
}

impl std::convert::From<u8> for OpCode {
    fn from(num: u8) -> Self{
        match num {
            1 => OpCode::OpConstant, 
            2 => OpCode::OpNil, 
            3 => OpCode::OpTrue, 
            4 => OpCode::OpFalse, 
            5 => OpCode::OpPop, 
            6 => OpCode::OpGetLocal, 
            7 => OpCode::OpSetLocal, 
            8 => OpCode::OpGetGlobal, 
            9 => OpCode::OpDefineGlobal, 
            10 => OpCode::OpSetGlobal, 
            11 => OpCode::OpEqual, 
            12 => OpCode::OpGreater, 
            13 => OpCode::OpLess, 
            14 => OpCode::OpAdd, 
            15 => OpCode::OpSubtract, 
            16 => OpCode::OpMultiply, 
            17 => OpCode::OpDivide, 
            18 => OpCode::OpNot, 
            19 => OpCode::OpNegate, 
            20 => OpCode::OpPrint, 
            21 => OpCode::OpJump, 
            22 => OpCode::OpJumpIfFalse, 
            23 => OpCode::OpLoop, 
            _ => OpCode::OpReturn,
        }
    } 
}

#[derive(Copy, Clone)]
pub struct Chunk {
    count: i64,
    capacity: i64,
    code: [u8; 256],
    lines: [i64; 256],
    constants: value::ValueArray,
}

impl Chunk {
    pub fn new() -> Chunk {
        return Chunk {
            count: 0,
            capacity: 0,
            code: [0; 256],
            lines: [0; 256],
            constants: value::ValueArray::new(),
        };
    }

    pub fn get_code(&self) -> [u8; 256] {
        return self.code;
    }

    pub fn get_constants(&self) -> value::ValueArray {
        return self.constants;
    }

    pub fn get_lines(&self) -> [i64; 256] {
        return self.lines;
    }

    pub fn get_count(&self) -> i64 {
        return self.count;
    }

    pub fn write_chunk(&mut self, byte: u8, line: i64) {
        self.code[self.count as usize] = byte;
        self.lines[self.count as usize] = line;
        self.count += 1;
    }

    pub fn add_constant(&mut self, value: value::Value) -> i64 {
        value.write_value_array(self.constants);
        self.count += 1; 

        return self.constants.get_count() as i64;
    }

    pub fn print(&self) {
        println!("CHUNK: \n\tCOUNT: {}, \n\tCAP: {}, \n\tCODE: {:?}, \n\tLINES: {:?}", self.count, self.capacity, self.code, self.lines);
    }
}

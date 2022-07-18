use crate::*;

#[derive(Copy, Clone)]
pub struct ParseRule {
    pub prefix: fn(
        &mut Parser,
        &mut vm::VM,
        &mut Compiler,
        &mut table::Table,
        &mut scanner::Scanner,
        bool,
        &std::collections::HashMap<scanner::TokenType, ParseRule>,
    ) -> (),

    pub infix: fn(
        &mut Parser,
        &mut vm::VM,
        &mut Compiler,
        &mut table::Table,
        &mut scanner::Scanner,
        bool,
        &std::collections::HashMap<scanner::TokenType, ParseRule>,
    ) -> (),

    pub precedence: Precedence,
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum Precedence {
    PrecNone,
    PrecAssignment,
    PrecOr,
    PrecAnd,
    PrecEquality,
    PrecComparison,
    PrecTerm,
    PrecFactor,
    PrecUnary,
    PrecCall,
    PrecPrimary,
}

#[derive(Copy, Clone, Debug)]
pub struct Parser {
    current: scanner::Token,
    previous: scanner::Token,
    had_error: bool,
    panic_mode: bool,
}

impl Parser {
    pub fn new() -> Parser {
        return Parser {
            current: scanner::Token::new(),
            previous: scanner::Token::new(),
            had_error: false,
            panic_mode: false,
        };
    }

    pub fn none(
        &mut self,
        vm: &mut vm::VM,
        compiler: &mut Compiler,
        table: &mut table::Table,
        scanner: &mut scanner::Scanner,
        can_assign: bool,
        rules: &std::collections::HashMap<scanner::TokenType, ParseRule>,
    ) {
        println!("NULL");
    }
    pub fn unary(
        &mut self,
        vm: &mut vm::VM,
        compiler: &mut Compiler,
        table: &mut table::Table,
        scanner: &mut scanner::Scanner,
        can_assign: bool,
        rules: &std::collections::HashMap<scanner::TokenType, ParseRule>,
    ) {
        todo!();
    }
    pub fn binary(
        &mut self,
        vm: &mut vm::VM,
        compiler: &mut Compiler,
        table: &mut table::Table,
        scanner: &mut scanner::Scanner,
        can_assign: bool,
        rules: &std::collections::HashMap<scanner::TokenType, ParseRule>,
    ) {
        let operator_type = self.previous.get_type();
        match rules.get(&self.previous.get_type()) {
            Some(rule) => {
                self.parse_precedence(vm, rule.precedence, compiler, table, scanner, rules);
                match operator_type {
                    scanner::TokenType::TokenBangEqual => {
                        compiler.emit_bytes(
                            chunk::OpCode::OpEqual as u8,
                            chunk::OpCode::OpNot as u8,
                            self,
                        );
                    }
                    scanner::TokenType::TokenEqualEqual => {
                        compiler.emit_byte(chunk::OpCode::OpEqual as u8, self);
                    }
                    scanner::TokenType::TokenGreater => {
                        compiler.emit_byte(chunk::OpCode::OpGreater as u8, self);
                    }
                    scanner::TokenType::TokenGreaterEqual => {
                        compiler.emit_bytes(
                            chunk::OpCode::OpLess as u8,
                            chunk::OpCode::OpNot as u8,
                            self,
                        );
                    }
                    scanner::TokenType::TokenLess => {
                        compiler.emit_byte(chunk::OpCode::OpLess as u8, self);
                    }
                    scanner::TokenType::TokenLessEqual => {
                        compiler.emit_bytes(
                            chunk::OpCode::OpGreater as u8,
                            chunk::OpCode::OpNot as u8,
                            self,
                        );
                    }
                    scanner::TokenType::TokenPlus => {
                        compiler.emit_byte(chunk::OpCode::OpAdd as u8, self);
                    }
                    scanner::TokenType::TokenMinus => {
                        compiler.emit_byte(chunk::OpCode::OpSubtract as u8, self);
                    }
                    scanner::TokenType::TokenStar => {
                        compiler.emit_byte(chunk::OpCode::OpMultiply as u8, self);
                    }
                    scanner::TokenType::TokenSlash => {
                        compiler.emit_byte(chunk::OpCode::OpDivide as u8, self);
                    }
                    _ => todo!(),
                }
            }
            None => todo!(),
        }
    }
    pub fn string(
        &mut self,
        vm: &mut vm::VM,
        compiler: &mut Compiler,
        table: &mut table::Table,
        scanner: &mut scanner::Scanner,
        can_assign: bool,
        rules: &std::collections::HashMap<scanner::TokenType, ParseRule>,
    ) {
        todo!();
    }
    pub fn literal(
        &mut self,
        vm: &mut vm::VM,
        compiler: &mut Compiler,
        table: &mut table::Table,
        scanner: &mut scanner::Scanner,
        can_assign: bool,
        rules: &std::collections::HashMap<scanner::TokenType, ParseRule>,
    ) {
        todo!();
    }
    pub fn variable(
        &mut self,
        vm: &mut vm::VM,
        compiler: &mut Compiler,
        table: &mut table::Table,
        scanner: &mut scanner::Scanner,
        can_assign: bool,
        rules: &std::collections::HashMap<scanner::TokenType, ParseRule>,
    ) {
        let name: scanner::Token = self.previous;
        let get_op: u8;
        let set_op: u8;

        let mut arg: u8 = compiler.resolve_local(name, self);

        if arg != u8::MAX {
            get_op = chunk::OpCode::OpGetLocal as u8;
            set_op = chunk::OpCode::OpSetLocal as u8;
        } else {
            println!("NAME: {:?}", name);
            let value: value::Value = value::Value::create(
                value::ValueType::ValObj,
                value::Union::create_obj(object::copy_string(vm, table, name.get_start(), name.get_length())),
            );
            arg = compiler.make_constant(value, self);
            get_op = chunk::OpCode::OpGetGlobal as u8;
            set_op = chunk::OpCode::OpSetGlobal as u8;
        }

        if can_assign && self.match_to(scanner, scanner::TokenType::TokenEqual) {
            compiler.expression_statement(vm, self, table, scanner, rules);
            compiler.emit_bytes(chunk::OpCode::OpSetGlobal as u8, arg, self);
        } else {
            compiler.emit_bytes(chunk::OpCode::OpGetGlobal as u8, arg, self);
        }
    }
    pub fn grouping(
        &mut self,
        vm: &mut vm::VM,
        compiler: &mut Compiler,
        table: &mut table::Table,
        scanner: &mut scanner::Scanner,
        can_assign: bool,
        rules: &std::collections::HashMap<scanner::TokenType, ParseRule>,
    ) {
        todo!();
    }
    pub fn number(
        &mut self,
        vm: &mut vm::VM,
        compiler: &mut Compiler,
        table: &mut table::Table,
        scanner: &mut scanner::Scanner,
        can_assign: bool,
        rules: &std::collections::HashMap<scanner::TokenType, ParseRule>,
    ) {
        let len = self
            .previous
            .get_start()
            .into_iter()
            .collect::<String>()
            .len();
        let t = self
            .previous
            .get_start()
            .into_iter()
            .collect::<String>()
            .trim()
            .trim_end_matches('\0')
            .parse::<i64>();
        match t {
            Ok(value) => {
                let v: value::Value = value::Value::create(
                    value::ValueType::ValNumber,
                    value::Union::create_num(value),
                );
                compiler.emit_constant(v, self)
            }
            Err(e) => {
                eprintln!("{e:?}");
            }
        }
    }

    pub fn consume(
        &mut self,
        mut scanner: &mut scanner::Scanner,
        token_type: scanner::TokenType,
        message: String,
    ) {
        if self.current.get_type() == token_type {
            self.advance(&mut scanner);
            return;
        }

        self.error_at_current(self.current.get_start());
    }

    pub fn parse_precedence(
        &mut self,
        vm: &mut vm::VM,
        precedence: Precedence,
        compiler: &mut Compiler,
        table: &mut table::Table,
        mut scanner: &mut scanner::Scanner,
        rules: &std::collections::HashMap<scanner::TokenType, ParseRule>,
    ) {
        self.advance(&mut scanner);
        match rules.get(&self.previous.get_type()) {
            Some(previous_prefix_rule) => {
                let can_assign = precedence <= Precedence::PrecAssignment;
                (previous_prefix_rule.prefix)(self, vm, compiler, table, scanner, can_assign, rules);
                match rules.get(&self.current.get_type()) {
                    Some(current_prefix_rule) => {
                        while precedence <= current_prefix_rule.precedence {
                            self.advance(&mut scanner);
                            (previous_prefix_rule.infix)(
                                self, vm, compiler, table, scanner, can_assign, rules,
                            );
                        }
                        if can_assign && self.match_to(scanner, scanner::TokenType::TokenEqual) {
                            self.error_at(self.previous, "Invalid Assignment Target".to_string());
                        }
                    }
                    None => {
                        self.error_at(self.current, "Expect Expression".to_string());
                        return;
                    }
                }
            }
            None => {
                self.error_at(self.previous, "Expect Expression".to_string());
                return;
            }
        }
    }

    fn syncronize(&mut self, scanner: &mut scanner::Scanner) {
        self.panic_mode = false;
        while self.current.get_type() != scanner::TokenType::TokenEof {
            if self.previous.get_type() == scanner::TokenType::TokenSemicolon {
                return;
            }
            match self.current.get_type() {
                scanner::TokenType::TokenClass
                | scanner::TokenType::TokenFun
                | scanner::TokenType::TokenVar
                | scanner::TokenType::TokenFor
                | scanner::TokenType::TokenIf
                | scanner::TokenType::TokenWhile
                | scanner::TokenType::TokenPrint
                | scanner::TokenType::TokenReturn => {
                    return;
                }
                _ => {}
            }
        }
        self.advance(scanner);
    }

    fn error_at(&mut self, token: scanner::Token, message: String) {
        if self.panic_mode {
            return;
        }
        self.panic_mode = true;
        print!("[line {}] Error", token.get_line());

        if token.get_type() == scanner::TokenType::TokenEof {
            println!(" at end: {:?}", message);
        } else if token.get_type() == scanner::TokenType::TokenError {
        } else {
            println!(" at {:?}: {:?}", token.get_length(), message);
        }

        self.had_error = true;
    }

    fn error_at_current(&mut self, message: [char; 256]) {
        self.error_at(self.current, message.into_iter().collect::<String>());
    }

    pub fn advance(&mut self, mut scanner: &mut scanner::Scanner) {
        self.previous = self.current;
        loop {
            let x = scanner.scan_token();
            self.current = x;
            if self.current.get_type() != scanner::TokenType::TokenError {
                break;
            }
            self.error_at_current(self.current.get_start());
        }
    }

    pub fn get_had_error(&self) -> bool {
        return self.had_error;
    }

    pub fn match_to(
        &mut self,
        mut scanner: &mut scanner::Scanner,
        token_type: scanner::TokenType,
    ) -> bool {
        if !(self.current.get_type() == token_type) {
            return false;
        }
        self.advance(&mut scanner);
        return true;
    }
}

#[derive(Debug)]
struct Local {
    name: scanner::Token,
    depth: i64,
}

pub struct Compiler {
    locals: Vec<Local>,
    local_count: i64,
    scope_depth: i64,
    current_chunk: chunk::Chunk,
}

impl Compiler {
    pub fn new() -> Compiler {
        return Compiler {
            locals: Vec::new(),
            local_count: 0,
            scope_depth: 0,
            current_chunk: chunk::Chunk::new(),
        };
    }

    fn resolve_local(&self, name: scanner::Token, parser: &mut Parser) -> u8 {
        for i in (self.local_count - 1)..0 {
            if i != -1 {
                let local: &Local = &self.locals[i as usize];
                if self.identifiers_equal(name, local.name) {
                    if local.depth == -1 {
                        parser.error_at(
                            parser.current,
                            "Cannot read local variable in its own initializer".to_string(),
                        );
                    }
                    return i as u8;
                }
            }
        }
        return u8::MAX;
    }

    fn print_statement(
        &mut self,
        vm: &mut vm::VM,
        scanner: &mut scanner::Scanner,
        table: &mut table::Table,
        mut parser: &mut Parser,
        rules: &std::collections::HashMap<scanner::TokenType, ParseRule>,
    ) {
        parser.parse_precedence(vm, Precedence::PrecAssignment, self, table, scanner, rules);
        parser.consume(
            scanner,
            scanner::TokenType::TokenSemicolon,
            "Expect ';' after value".to_string(),
        );
        self.emit_byte(chunk::OpCode::OpPrint as u8, parser);
    }

    fn expression_statement(
        &mut self,
        vm: &mut vm::VM,
        mut parser: &mut Parser,
        table: &mut table::Table,
        scanner: &mut scanner::Scanner,
        rules: &std::collections::HashMap<scanner::TokenType, ParseRule>,
    ) {
        parser.parse_precedence(vm, Precedence::PrecAssignment, self, table, scanner, rules);
        parser.consume(
            scanner,
            scanner::TokenType::TokenSemicolon,
            "Expect ';' after expression".to_string(),
        );

        self.emit_byte(chunk::OpCode::OpPop as u8, parser);
    }

    fn statement(
        &mut self,
        vm: &mut vm::VM,
        mut parser: &mut Parser,
        table: &mut table::Table,
        scanner: &mut scanner::Scanner,
        rules: &std::collections::HashMap<scanner::TokenType, ParseRule>,
    ) {
        if parser.match_to(scanner, scanner::TokenType::TokenPrint) {
            self.print_statement(vm, scanner, table, parser, rules);
        } else if parser.match_to(scanner, scanner::TokenType::TokenFor) {
            todo!();
        } else if parser.match_to(scanner, scanner::TokenType::TokenIf) {
            todo!();
        } else if parser.match_to(scanner, scanner::TokenType::TokenWhile) {
            todo!();
        } else if parser.match_to(scanner, scanner::TokenType::TokenLeftBrace) {
            todo!();
        } else {
            self.expression_statement(vm, parser, table, scanner, rules);
        }
    }

    fn add_local(&self, name: scanner::Token) {
        todo!();
    }

    fn define_variable(&mut self, global: u8, parser: &mut Parser) {
        if self.scope_depth > 0 {
            self.locals[(self.local_count - 1) as usize].depth = self.scope_depth;
            return;
        }
        self.emit_bytes(chunk::OpCode::OpDefineGlobal as u8, global, parser);
    }

    fn identifiers_equal(&self, a: scanner::Token, b: scanner::Token) -> bool {
        if a.get_length() != b.get_length() {
            return false;
        }
        todo!();
    }

    fn declare_variable(&self, parser: &mut Parser) {
        if self.scope_depth == 0 {
            return;
        }

        let name = parser.previous;
        for i in (self.local_count - 1)..0 {
            let local = &self.locals[i as usize];
            if local.depth != -1 && local.depth < self.scope_depth {
                break;
            }

            if self.identifiers_equal(name, local.name) {
                parser.error_at(
                    parser.current,
                    "Already a varaiable with this name in this scope.".to_string(),
                );
            }
        }

        self.add_local(name);
    }

    fn var_declaration(
        &mut self,
        vm: &mut vm::VM,
        parser: &mut Parser,
        table: &mut table::Table,
        scanner: &mut scanner::Scanner,
        rules: &std::collections::HashMap<scanner::TokenType, ParseRule>,
    ) {
        parser.consume(
            scanner,
            scanner::TokenType::TokenIdentifier,
            "Expect variable name".to_string(),
        );
        self.declare_variable(parser);
        let mut global: u8;
        if self.scope_depth > 0 {
            global = 0;
        } else {
            println!("PARSER: {:?}", parser.previous); 
            let value: value::Value = value::Value::create(
                value::ValueType::ValObj,
                value::Union::create_obj(object::copy_string(
                    vm,
                    table,
                    parser.previous.get_start(),
                    parser.previous.get_length(),
                )),
            );
            global = self.make_constant(value, parser);
        }
        if parser.match_to(scanner, scanner::TokenType::TokenEqual) {
            self.expression_statement(vm, parser, table, scanner, rules);
        } else {
            self.emit_byte(chunk::OpCode::OpNil as u8, parser);
        }

        parser.consume(
            scanner,
            scanner::TokenType::TokenSemicolon,
            "Expect ';' after variable declaration".to_string(),
        );

        self.define_variable(global, parser);
    }

    pub fn declaration(
        &mut self,
        vm: &mut vm::VM,
        mut parser: &mut Parser,
        table: &mut table::Table,
        scanner: &mut scanner::Scanner,
        rules: &std::collections::HashMap<scanner::TokenType, ParseRule>,
    ) {
        if parser.match_to(scanner, scanner::TokenType::TokenVar) {
            self.var_declaration(vm, parser, table, scanner, rules);
        } else {
            self.statement(vm, parser, table, scanner, rules);
        }
        if parser.panic_mode {
            parser.syncronize(scanner);
        }
    }

    fn make_constant(&mut self, value: value::Value, parser: &mut Parser) -> u8 {
        let constant = self.current_chunk.add_constant(value);
        if constant > u8::MAX.into() {
            parser.error_at(
                parser.current,
                "Too many Constants in One Chunk".to_string(),
            );
            return 0;
        }
        return constant as u8;
    }

    fn emit_bytes(&mut self, byte1: u8, byte2: u8, parser: &mut Parser) {

        self.current_chunk
            .write_chunk(byte1, parser.previous.get_line());
        self.current_chunk
            .write_chunk(byte2, parser.previous.get_line());
    }

    fn emit_byte(&mut self, byte: u8, parser: &mut Parser) {
        self.current_chunk
            .write_chunk(byte, parser.previous.get_line());

    }

    fn emit_constant(&mut self, value: value::Value, parser: &mut Parser) {
        let constant = self.make_constant(value, parser);
        self.emit_bytes(
            chunk::OpCode::OpConstant as u8,
            constant,
            parser,
        );
    }

    fn emit_return(&mut self, parser: &mut Parser) {
        self.emit_byte(chunk::OpCode::OpReturn as u8, parser);
    }

    pub fn end_compiler(&mut self, vm: &mut vm::VM, parser: &mut Parser) {
        self.emit_return(parser);
        if !parser.get_had_error() {
            vm.disassemble_chunk(self.current_chunk, "code");
        }
    }
}

pub fn compile(
    source: String,
    vm: &mut vm::VM,
    chunk: &chunk::Chunk,
    rules: &std::collections::HashMap<scanner::TokenType, ParseRule>,
) -> bool {
    let mut compiler: Compiler = Compiler::new();
    let mut parser: &mut Parser = &mut Parser::new();
    let mut scanner: &mut scanner::Scanner = &mut scanner::Scanner::create(source);
    let mut table: &mut table::Table = &mut table::Table::new();

    let compiling_chunk: &chunk::Chunk = chunk;
    
    parser.advance(&mut scanner);

    while !(parser.match_to(scanner, scanner::TokenType::TokenEof)) {
        compiler.declaration(vm, parser, table, scanner, rules);
    }

    compiler.end_compiler(vm, parser);
    
    return !parser.had_error;
}

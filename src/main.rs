mod chunk;
mod compiler;
mod debug;
mod object;
mod scanner;
mod table;
mod value;
mod vm;

fn repl(
    virtual_machine: &mut vm::VM,
    rules: &std::collections::HashMap<scanner::TokenType, compiler::ParseRule>,
) {
    loop {
        println!("->");
        let mut buffer = String::new();
        let lines = std::io::stdin().read_line(&mut buffer);
        match lines {
            Ok(_) => {
                println!("{:#?}", buffer);
            }
            Err(e) => {
                eprintln!("ERROR: {e:?}");
            }
        }
        virtual_machine.interpret(buffer, rules);
    }
}
fn run_file(
    file: &String,

    virtual_machine: &mut vm::VM,
    rules: &std::collections::HashMap<scanner::TokenType, compiler::ParseRule>,
) {
    println!("{}", file);

    let source: String = std::fs::read_to_string(file).expect("Error reading file");

    println!("{}", source);
    
    match virtual_machine.interpret(source, rules) {
        vm::InterpretResult::InterpretOk => {
            println!("EXIT 0");
            std::process::exit(0)
        },
        vm::InterpretResult::InterpretCompileError => {
            println!("EXIT 65");
            std::process::exit(65)
        },
        vm::InterpretResult::InterpretRuntimeError => {
            println!("EXIT 65");
            std::process::exit(70)
        },
    }
}

fn main() {
    let rules: std::collections::HashMap<scanner::TokenType, compiler::ParseRule> =
        std::collections::HashMap::from([
            (
                scanner::TokenType::TokenLeftParen,
                compiler::ParseRule {
                    prefix: compiler::Parser::grouping,
                    infix: compiler::Parser::none,
                    precedence: compiler::Precedence::PrecNone,
                },
            ),
            (
                scanner::TokenType::TokenRightParen,
                compiler::ParseRule {
                    prefix: compiler::Parser::none,
                    infix: compiler::Parser::none,
                    precedence: compiler::Precedence::PrecNone,
                },
            ),
            (
                scanner::TokenType::TokenLeftBrace,
                compiler::ParseRule {
                    prefix: compiler::Parser::none,
                    infix: compiler::Parser::none,
                    precedence: compiler::Precedence::PrecNone,
                },
            ),
            (
                scanner::TokenType::TokenRightBrace,
                compiler::ParseRule {
                    prefix: compiler::Parser::none,
                    infix: compiler::Parser::none,
                    precedence: compiler::Precedence::PrecNone,
                },
            ),
            (
                scanner::TokenType::TokenComma,
                compiler::ParseRule {
                    prefix: compiler::Parser::none,
                    infix: compiler::Parser::none,
                    precedence: compiler::Precedence::PrecNone,
                },
            ),
            (
                scanner::TokenType::TokenDot,
                compiler::ParseRule {
                    prefix: compiler::Parser::none,
                    infix: compiler::Parser::none,
                    precedence: compiler::Precedence::PrecNone,
                },
            ),
            (
                scanner::TokenType::TokenMinus,
                compiler::ParseRule {
                    prefix: compiler::Parser::unary,
                    infix: compiler::Parser::binary,
                    precedence: compiler::Precedence::PrecTerm,
                },
            ),
            (
                scanner::TokenType::TokenPlus,
                compiler::ParseRule {
                    prefix: compiler::Parser::none,
                    infix: compiler::Parser::binary,
                    precedence: compiler::Precedence::PrecTerm,
                },
            ),
            (
                scanner::TokenType::TokenSemicolon,
                compiler::ParseRule {
                    prefix: compiler::Parser::none,
                    infix: compiler::Parser::none,
                    precedence: compiler::Precedence::PrecNone,
                },
            ),
            (
                scanner::TokenType::TokenSlash,
                compiler::ParseRule {
                    prefix: compiler::Parser::none,
                    infix: compiler::Parser::binary,
                    precedence: compiler::Precedence::PrecFactor,
                },
            ),
            (
                scanner::TokenType::TokenNumber,
                compiler::ParseRule {
                    prefix: compiler::Parser::number,
                    infix: compiler::Parser::none,
                    precedence: compiler::Precedence::PrecNone,
                },
            ),
            (
                scanner::TokenType::TokenPrint,
                compiler::ParseRule {
                    prefix: compiler::Parser::none,
                    infix: compiler::Parser::none,
                    precedence: compiler::Precedence::PrecNone,
                },
            ),
            (
                scanner::TokenType::TokenEof,
                compiler::ParseRule {
                    prefix: compiler::Parser::none,
                    infix: compiler::Parser::none,
                    precedence: compiler::Precedence::PrecNone,
                },
            ),
            (
                scanner::TokenType::TokenIdentifier,
                compiler::ParseRule {
                    prefix: compiler::Parser::variable,
                    infix: compiler::Parser::none,
                    precedence: compiler::Precedence::PrecNone,
                },
            ),
        ]);
    let args: Vec<String> = std::env::args().collect();

    let virtual_machine: &mut vm::VM = &mut vm::VM::new();

    if args.len() == 1 {
        repl(virtual_machine, &rules);
    } else if args.len() == 2 {
        run_file(&args[1], virtual_machine, &rules);
    } else {
        eprintln!("Usage: rox [path]");
        std::process::exit(64);
    }
    return;
}

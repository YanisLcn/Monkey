use std::io::{self, Write};

use monkey::{lexer::lexer::Lexer, parser::parser::Parser};

const PROMPT: &str = "@ ";

fn main() {
    println!("Monkey Programming Language !");

    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let input = &mut String::new();

    loop {
        input.clear();
        print!("{PROMPT}");
        let _ = stdout.flush();
        let _ = stdin.read_line(input);

        let lexer = Lexer::new(input.clone());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();

        if parser.errors().len() != 0 {
            parser.errors().iter().for_each(|err| println!("{err}"));
            continue;
        }

        println!("{}", program);
    }
}

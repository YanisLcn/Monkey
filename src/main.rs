use std::io::{self, Write};

use monkey::{lexer::lexer::Lexer, token::token::Token};

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
        print!("{}", input);

        let mut lexer = Lexer::new(input.clone());

        let mut token = lexer.next_token();
        while token != Token::EOF {
            println!("{:?}", token);
            token = lexer.next_token();
        }

        println!();
    }
}

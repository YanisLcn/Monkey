#[cfg(test)]
mod token_test {
    use crate::lexer::lexer::Lexer;
    use crate::token::token::Token;

    #[test]
    fn basics_tokens() {
        let input = "=+(){},;";

        let expected_tokens: Vec<Token> = vec![
            Token::ASSIGN,
            Token::PLUS,
            Token::LPAREN,
            Token::RPAREN,
            Token::LBRACE,
            Token::RBRACE,
            Token::COMMA,
            Token::SEMICOLON,
            Token::EOF,
        ];

        let mut lexer = Lexer::new(input);

        expected_tokens
            .iter()
            .for_each(|token| assert_eq!(lexer.next_token(), *token));
    }

    #[test]
    fn classic_tokens() {
        let input = "let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);";

        let expected_tokens: Vec<Token> = vec![
            Token::LET,
            Token::IDENT(String::from("five")),
            Token::ASSIGN,
            Token::INT(String::from("5")),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT(String::from("ten")),
            Token::ASSIGN,
            Token::INT(String::from("10")),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT(String::from("add")),
            Token::ASSIGN,
            Token::FUNCTION,
            Token::LPAREN,
            Token::IDENT(String::from("x")),
            Token::COMMA,
            Token::IDENT(String::from("y")),
            Token::RPAREN,
            Token::LBRACE,
            Token::IDENT(String::from("x")),
            Token::PLUS,
            Token::IDENT(String::from("y")),
            Token::SEMICOLON,
            Token::RBRACE,
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT(String::from("result")),
            Token::ASSIGN,
            Token::IDENT(String::from("add")),
            Token::LPAREN,
            Token::IDENT(String::from("five")),
            Token::COMMA,
            Token::IDENT(String::from("ten")),
            Token::RPAREN,
            Token::SEMICOLON,
            Token::EOF,
        ];

        let mut lexer = Lexer::new(input);

        expected_tokens
            .iter()
            .for_each(|token| assert_eq!(lexer.next_token(), *token));
    }

    #[test]
    fn tokens_2_char_operators() {
        let input = "let five = 5;
let ten = 10;

let add = fn(x, y) {
  x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
	return true;
} else {
	return false;
}

10 == 10;
10 != 9;
";

        let mut lexer = Lexer::new(input);

        let expected_tokens = vec![

		Token::LET,
		Token::IDENT(String::from("five")),
		Token::ASSIGN,
		Token::INT(String::from("5")),
		Token::SEMICOLON, 
		Token::LET, 
		Token::IDENT(String::from("ten")),
		Token::ASSIGN,
		Token::INT(String::from("10")),
		Token::SEMICOLON, 
		Token::LET,
		Token::IDENT(String::from("add")),
		Token::ASSIGN,
		Token::FUNCTION, 
		Token::LPAREN,
		Token::IDENT(String::from("x")),
		Token::COMMA, 
		Token::IDENT(String::from("y")),
		Token::RPAREN, 
		Token::LBRACE,
		Token::IDENT(String::from("x")),
		Token::PLUS,
		Token::IDENT(String::from("y")),
		Token::SEMICOLON, 
		Token::RBRACE, 
		Token::SEMICOLON, 
		Token::LET, 
		Token::IDENT(String::from("result")),
		Token::ASSIGN, 
		Token::IDENT(String::from("add")),
		Token::LPAREN, 
		Token::IDENT(String::from("five")),
		Token::COMMA, 
		Token::IDENT(String::from("ten")),
		Token::RPAREN, 
		Token::SEMICOLON, 
		Token::BANG, 
		Token::SUB, 
		Token::DIV,
		Token::MUL,
		Token::INT(String::from("5")),
		Token::SEMICOLON,
		Token::INT(String::from("5")),
		Token::LT,
		Token::INT(String::from("10")),
		Token::GT, 
		Token::INT(String::from("5")),
		Token::SEMICOLON, 
		Token::IF, 
		Token::LPAREN, 
		Token::INT(String::from("5")),
		Token::LT,
		Token::INT(String::from("10")),
		Token::RPAREN, 
		Token::LBRACE, 
		Token::RETURN,
		Token::TRUE, 
		Token::SEMICOLON, 
		Token::RBRACE,
		Token::ELSE, 
		Token::LBRACE, 
		Token::RETURN, 
		Token::FALSE, 
		Token::SEMICOLON, 
		Token::RBRACE, 
		Token::INT(String::from("10")),
		Token::EQ, 
		Token::INT(String::from("10")),
		Token::SEMICOLON,
		Token::INT(String::from("10")),
		Token::NE, 
		Token::INT(String::from("9")),
		Token::SEMICOLON, 
		Token::EOF];

        expected_tokens
            .iter()
            .for_each(|token| assert_eq!(lexer.next_token(), *token));

    }

    #[test]
    fn extended_tokens_with_strings() {
        let input = "let five = 5;
let ten = 10;

let add = fn(x, y) {
  x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
	return true;
} else {
	return false;
}

10 == 10;
10 != 9;
\"foobar\"
\"foo bar\"
[1, 2];
{\"foo\": \"bar\"}
";

        let mut lexer = Lexer::new(input);

        let expected_tokens = vec![

		Token::LET,
		Token::IDENT(String::from("five")),
		Token::ASSIGN,
		Token::INT(String::from("5")),
		Token::SEMICOLON, 
		Token::LET, 
		Token::IDENT(String::from("ten")),
		Token::ASSIGN,
		Token::INT(String::from("10")),
		Token::SEMICOLON, 
		Token::LET,
		Token::IDENT(String::from("add")),
		Token::ASSIGN,
		Token::FUNCTION, 
		Token::LPAREN,
		Token::IDENT(String::from("x")),
		Token::COMMA, 
		Token::IDENT(String::from("y")),
		Token::RPAREN, 
		Token::LBRACE,
		Token::IDENT(String::from("x")),
		Token::PLUS,
		Token::IDENT(String::from("y")),
		Token::SEMICOLON, 
		Token::RBRACE, 
		Token::SEMICOLON, 
		Token::LET, 
		Token::IDENT(String::from("result")),
		Token::ASSIGN, 
		Token::IDENT(String::from("add")),
		Token::LPAREN, 
		Token::IDENT(String::from("five")),
		Token::COMMA, 
		Token::IDENT(String::from("ten")),
		Token::RPAREN, 
		Token::SEMICOLON, 
		Token::BANG, 
		Token::SUB, 
		Token::DIV,
		Token::MUL,
		Token::INT(String::from("5")),
		Token::SEMICOLON,
		Token::INT(String::from("5")),
		Token::LT,
		Token::INT(String::from("10")),
		Token::GT, 
		Token::INT(String::from("5")),
		Token::SEMICOLON, 
		Token::IF, 
		Token::LPAREN, 
		Token::INT(String::from("5")),
		Token::LT,
		Token::INT(String::from("10")),
		Token::RPAREN, 
		Token::LBRACE, 
		Token::RETURN,
		Token::TRUE, 
		Token::SEMICOLON, 
		Token::RBRACE,
		Token::ELSE, 
		Token::LBRACE, 
		Token::RETURN, 
		Token::FALSE, 
		Token::SEMICOLON, 
		Token::RBRACE, 
		Token::INT(String::from("10")),
		Token::EQ, 
		Token::INT(String::from("10")),
		Token::SEMICOLON,
		Token::INT(String::from("10")),
		Token::NE, 
		Token::INT(String::from("9")),
		Token::SEMICOLON, 
		Token::STRING(String::from("foobar")),
		Token::STRING(String::from("foo bar")),
		Token::LBRACKET, 
		Token::INT(String::from("1")),
		Token::COMMA, 
		Token::INT(String::from("2")),
		Token::RBRACKET, 
		Token::SEMICOLON, 
		Token::LBRACE, 
		Token::STRING(String::from("foo")),
		Token::COLON, 
		Token::STRING(String::from("bar")),
		Token::RBRACE, 
		Token::EOF];

        expected_tokens
            .iter()
            .for_each(|token| assert_eq!(lexer.next_token(), *token));

    }
}

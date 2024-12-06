use std::iter::Peekable;
use std::str::Chars;

/// An error that can occur when parsing the expression language.
#[derive(PartialEq)]
enum TokenizerError {
    UnexpectedEndOfInput,
    UnexpectedCharacter(char),
    UnexpectedToken(Token),
}

impl std::fmt::Debug for TokenizerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TokenizerError::UnexpectedEndOfInput => write!(f, "Unexpected end of input"),
            TokenizerError::UnexpectedCharacter(c) => write!(f, "Unexpected character '{}'", c),
            TokenizerError::UnexpectedToken(tok) => write!(f, "Unexpected token {:?}", tok),
        }
    }
}

/// An arithmetic operator.
#[derive(Debug, PartialEq, Clone, Copy)]
enum Op {
    Add,
    Sub,
}

/// A token in the expression language.
#[derive(Debug, PartialEq)]
enum Token {
    Number(String),
    Identifier(String),
    Operator(Op),
}

/// An expression in the expression language.
#[derive(Debug, PartialEq)]
enum Expression {
    /// A reference to a variable.
    Var(String),
    /// A literal number.
    Number(u32),
    /// A binary operation.
    Operation(Box<Expression>, Op, Box<Expression>),
}

fn tokenize(input: &str) -> Tokenizer {
    return Tokenizer(input.chars().peekable());
}

struct Tokenizer<'a>(Peekable<Chars<'a>>);

impl<'a> Tokenizer<'a> {
    fn collect_number(&mut self, first_char: char) -> Token {
        let mut num = String::from(first_char);
        while let Some(&c @ '0'..='9') = self.0.peek() {
            num.push(c);
            self.0.next();
        }
        Token::Number(num)
    }

    fn collect_identifier(&mut self, first_char: char) -> Token {
        let mut ident = String::from(first_char);
        while let Some(&c @ ('a'..='z' | '_' | '0'..='9')) = self.0.peek() {
            ident.push(c);
            self.0.next();
        }
        Token::Identifier(ident)
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<Token, TokenizerError>;

    fn next(&mut self) -> Option<Result<Token, TokenizerError>> {
        let c = self.0.next()?;
        match c {
            '0'..='9' => Some(Ok(self.collect_number(c))),
            'a'..='z' => Some(Ok(self.collect_identifier(c))),
            '+' => Some(Ok(Token::Operator(Op::Add))),
            '-' => Some(Ok(Token::Operator(Op::Sub))),
            _ => Some(Err(TokenizerError::UnexpectedCharacter(c))),
        }
    }
}

fn parse(input: &str) -> Result<Expression, TokenizerError> {
    let mut tokens = tokenize(input);

    fn parse_expr<'a>(tokens: &mut Tokenizer<'a>) -> Result<Expression, TokenizerError> {
        let Some(tok) = tokens.next() else {
            return Err(TokenizerError::UnexpectedEndOfInput);
        };
        let expr = match tok {
            Ok(Token::Number(num)) => {
                let v = num.parse().expect("Invalid 32-bit integer");
                Ok(Expression::Number(v))
            }
            Ok(Token::Identifier(ident)) => Ok(Expression::Var(ident)),
            Ok(tok2) => Err(TokenizerError::UnexpectedToken(tok2)), // eg Token::Operator
            Err(err) => Err(err),
        }?;
        // Look ahead to parse a binary operation if present.
        match tokens.next() {
            None => Ok(expr),
            Some(Ok(Token::Operator(op))) => Ok(Expression::Operation(
                Box::new(expr),
                op,
                Box::new(parse_expr(tokens)?),
            )),
            Some(tok2) =>  Err(TokenizerError::UnexpectedToken(tok2?)),
        }
    }

    parse_expr(&mut tokens)
}

#[test]
fn test_tokenize_passes() {
    let mut tokens = tokenize("10+foo+20-30");
    assert_eq!(tokens.next(), Some(Ok(Token::Number("10".to_string()))));
    assert_eq!(tokens.next(), Some(Ok(Token::Operator(Op::Add))));
    assert_eq!(tokens.next(), Some(Ok(Token::Identifier("foo".to_string()))));
    assert_eq!(tokens.next(), Some(Ok(Token::Operator(Op::Add))));
    assert_eq!(tokens.next(), Some(Ok(Token::Number("20".to_string()))));
    assert_eq!(tokens.next(), Some(Ok(Token::Operator(Op::Sub))));
    assert_eq!(tokens.next(), Some(Ok(Token::Number("30".to_string()))));
    assert_eq!(tokens.next(), None);
}

#[test]
fn test_parse_passes() {
    let expr = parse("10+foo+20-30");

    //  left: Ok(Operation(Number(10), Add, Operation(Var("foo"), Add, Operation(Number(20), Sub, Number(30)))))
    assert_eq!(
        expr,
        // 10 + (foo + (20 - 30))
        Ok(Expression::Operation(
            Box::new(Expression::Number(10)),
            Op::Add,
            Box::new(Expression::Operation(
                Box::new(Expression::Var("foo".to_string())),
                Op::Add,
                Box::new(Expression::Operation(
                    Box::new(Expression::Number(20)),
                    Op::Sub,
                    Box::new(Expression::Number(30))
                ))
            ))
        ))
    );
}

#[test]
fn test_parse_fails() {
    let expr = parse("10+foo+20-");
    assert_eq!(expr, Err(TokenizerError::UnexpectedEndOfInput));
}

#[test]
fn test_parse_fails2() {
    let expr = parse("+");
    assert_eq!(expr, Err(TokenizerError::UnexpectedToken(Token::Operator(Op::Add))));
}

#[test]
fn test_parse_fails3() {
    let expr = parse("10?");
    assert_eq!(expr, Err(TokenizerError::UnexpectedCharacter('?')));
}

fn main() {
    let expr = parse("10+foo+20-30");
    if let Err(err) = expr {
        eprintln!("Error: {:?}", err);
    } else {
        let expr = expr.unwrap();
        println!("{expr:?}");
    }
}

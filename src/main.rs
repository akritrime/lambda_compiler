use std::iter::Peekable;
use std::vec::IntoIter;

// THE TOKENIZER
// The first step of any compilation is breaking the input into chunks that can be fed into a parser. This is called lexical analysis.

// Token - the data structure that stores an acharacter as a parseable token
#[derive(Debug)]
enum Token {
    LParen,
    RParen,
    Int(i64),
    Str(String),
    Name(String)
}

fn tokenizer(input: &str) -> Result<Vec<Token>, String> {
    let mut current = 0; // tracks the current position of the cursor
    let mut tokens = vec![]; // the Vec of tokens that we will return
    let mut stream = input.chars().peekable(); // we will peek a character to see if we have reached the end

    // A while loop to iterate through the input and generate an array of tokens.

    while let Some(_) = stream.peek() {
        let ch = stream.next().unwrap();
        current += 1;

        match ch {
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            '"' => {
                let mut temp = String::new();
                let quotes_close_check = |ch: &char| if *ch != '"' {Some(())} else {None};

                while let Some(_) = stream.peek().and_then(quotes_close_check) {
                    let ch = stream.next().unwrap();
                    current += 1;

                    temp.push(ch);
                }

                current += 1;
                match stream.next() {
                    Some(ch) if ch == '"' => tokens.push(Token::Str(temp)),
                    _ => return Err(format!("Expected a \" after {} at {}.", temp, current))
                }

            }
            a if a.is_alphabetic() => {
                let mut temp = String::new();
                temp.push(a);

                let alphanumeric_check = |ch: &char| if ch.is_alphanumeric() { Some(()) } else { None };

                while let Some(_) = stream.peek().and_then(alphanumeric_check) {
                    let a = stream.next().unwrap();
                    current += 1;
                    temp.push(a);
                };
                
                tokens.push(Token::Name(temp))
            }
            n if n.is_digit(10) => {
                let mut temp = String::new();
                temp.push(n);

                let digit_check = |ch: &char| if ch.is_digit(10) { Some(()) } else { None };

                while let Some(_) = stream.peek().and_then(digit_check) {
                    let n = stream.next().unwrap();
                    current += 1;
                    temp.push(n);
                };
                match temp.parse() {
                    Ok(num) => tokens.push(Token::Int(num)),
                    Err(e) => return Err(format!("Error at {}. Expected a number, got {}. [Error from Rust: {}]", current, temp, e))
                }
            },
            s if s.is_whitespace() => (),
            _ => return Err(format!("Unknown character [ {} ] at {}.", ch, current))
        }
    }

    Ok(tokens)
}


// THE PARSER
// The second step is assigning meaning to those tokens, or parsing.

// ASTNode - the type that stores a token with assigned meaning
#[derive(Debug)]
enum ASTNode {
    Program(Program),
    NumberLiterals(Token),
    StringLiterals(Token),
    CallExpression(Call),
    EOF,
    Error(String)
}

#[derive(Debug)]
struct Call {
    name: String,
    params: Vec<ASTNode>
}

#[derive(Debug)]
struct Program {
    body: Vec<ASTNode>
}

fn parser(input: Vec<Token>) -> ASTNode {
    let mut current = 0;
    let mut tokens = input.into_iter().peekable();

    fn walk(current: &mut i32, tokens: &mut Peekable<IntoIter<Token>>) -> ASTNode {

        *current += 1;
        let token = match tokens.next() {
            Some(tok) => tok,
            _ => return ASTNode::EOF
        };

        match token {
            Token::Int(_) => ASTNode::NumberLiterals(token),
            Token::Str(_) => ASTNode::StringLiterals(token),
            Token::LParen => {
                // skipping the LParen
                *current += 1;
                let mut ast_node = match tokens.next() {
                    Some(Token::Name(name)) => Call {
                        name,
                        params: vec![]
                    },

                    _ => return ASTNode::Error(format!("Expected a function name at {}.", current))
                };
                
                loop {
                    match tokens.peek() {
                        Some(Token::RParen) => break,
                        Some(_) => (),
                        None => return ASTNode::Error(format!("Expected a ) at {}.", current))
                    }
                    ast_node.params.push(walk(current, tokens))
                }

                tokens.next();
                *current += 1;
                ASTNode::CallExpression(ast_node)

            }
            _ => ASTNode::Error(format!("Unexpected {:?} at {}", token, current))
        }

    }

    let mut prog = Program {
        body: vec![]
    };

    loop {
        if let None = tokens.peek() {
            break
        };
        prog.body.push(walk(&mut current, &mut tokens))
    }

    ASTNode::Program(prog)
}


fn main() {
    let tokens = tokenizer("(add 24 \"String\")").unwrap();
    println!("{:?}", parser(tokens));
}

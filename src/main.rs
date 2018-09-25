use std::iter::Peekable;
use std::vec::IntoIter;
// use std::collections::HashMap;

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
    Program(Vec<ASTNode>),
    NumberLiterals(Token),
    StringLiterals(Token),
    CallExpression {
        name: String,
        params: Vec<ASTNode>
    },
    EOF,
    Error(String)
}

impl ASTNode {
    fn is_eof(&self) -> bool {
        match self {
            ASTNode::EOF => true,
            _ => false
        }
    }

    // fn enter(&self, parent: &Option<&Self>) {

    // }

    // fn exit(&self, parent: &Option<&Self>) {
        
    // }
}

// #[derive(Debug)]
// struct Call {
//     name: String,
//     params: Vec<ASTNode>
// }

// #[derive(Debug)]
// struct Program {
//     body: Vec<ASTNode>
// }

fn parser(input: Vec<Token>) -> ASTNode {
    let mut current = 0;
    let mut tokens = input.into_iter().peekable();

    fn walk(current: &mut i32, tokens: &mut Peekable<IntoIter<Token>>) -> ASTNode {

        *current += 1;
        let token = match tokens.next() {
            Some(tok) => tok,
            _ => return ASTNode::EOF
        };

        // println!("{:?}", tokens.peek());

        match token {
            Token::Int(_) => ASTNode::NumberLiterals(token),
            Token::Str(_) => ASTNode::StringLiterals(token),
            Token::LParen => {
                // skipping the LParen
                *current += 1;
                let mut name = match tokens.next() {
                    Some(Token::Name(name)) => name,

                    _ => return ASTNode::Error(format!("Expected a function name at {}.", current))
                };
                
                let mut params = vec![];
                loop {
                    match tokens.peek() {
                        Some(Token::RParen) => break,
                        Some(_) => params.push(walk(current, tokens)),
                        None => return ASTNode::Error(format!("Expected a ) at {}.", current))
                    }
                }

                tokens.next();
                *current += 1;
                ASTNode::CallExpression {
                    name,
                    params
                }

            }
            _ => ASTNode::Error(format!("Unexpected {:?} at {}", token, current))
        }

    }

    let mut body = vec![];

    loop {
        let ast_node = walk(&mut current, &mut tokens);
        let end = ast_node.is_eof();
        body.push(ast_node);
        if end {
            break
        }
    }

    ASTNode::Program(body)
}

// THE TRAVERSER
// The third step is traversing through the AST and visit all the nodes. 
// struct Visitor {

// }

// fn traverser(ast: &ASTNode) {
//     fn traverseArray(children: &Vec<ASTNode>, parent: &Option<&ASTNode>) {
//         for child in children {
//             traverseNode(child, parent)
//         }
//     }

//     fn traverseNode(node: &ASTNode, parent: &Option<&ASTNode>) {
//         node.enter(parent);

//         match node {
//             ASTNode::Program(body) => traverseArray(&body, &Some(node)),
//             ASTNode::CallExpression { name: _, params } => traverseArray(&params, &Some(node)),
//             _ => ()
//         }

//         node.exit(parent);
//     }

//     traverseNode(ast, &None)
// }

// THE TRANSFORMER
// The transformer will take the AST that our parser generated, traverse the array and create a new AST.

fn execute(node: ASTNode) -> Result<Vec<isize>, String> {

    

    match node {
        ASTNode::Program(body) => {
            let mut res = vec![];
            for node in body {
                let temp = execute(node);

                match temp {
                    Ok(n) => res.extend_from_slice(&n),
                    _ => return temp
                }
            }

            Ok(res)
        },

        ASTNode::CallExpression { name, params } => {
            let mut params = params.into_iter().map(execute);
            match name.as_str() {
                "add" => {
                    let mut res = 0;

                    for param in params {
                        match param {
                            Ok(temp) => res += temp.first().unwrap_or(&0),
                            _ => return param
                        }
                    }

                    Ok(vec![res])
                },

                "subtract" => {
                    let mut res = match params.next() {
                        Some(temp) => match temp {
                            Ok(temp) => *temp.first().unwrap_or(&0),
                            _ => return temp
                        },
                        _ => return Ok(vec![0])
                    };

                    for param in params {
                        match param {
                            Ok(temp) => res -= temp.first().unwrap_or(&0),
                            _ => return param
                        }
                    }

                    Ok(vec![res])
                },

                _ => Err(format!("Not yet supported {}", name))
            }
        },

        ASTNode::NumberLiterals(Token::Int(n)) => Ok(vec![n as isize]),
        ASTNode::EOF => Ok(vec![]),
        _ => Err(format!("Unexpected node: {:?}", node))
    }
}

fn main() {
    let tokens = tokenizer("(add 24 3 (subtract 3 1 5))").unwrap();
    println!("{:?}", execute(parser(tokens)));
    // for _ in 0..12 {
    //     println!("{:?}", tokens.next());
    // }
}

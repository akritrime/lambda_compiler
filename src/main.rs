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
    let mut input_chars = input.chars().peekable(); // we will peek a character to see if we have reached the end

    // A while loop to iterate through the input and generate an array of tokens.

    while let Some(_) = input_chars.peek() {
        let ch = input_chars.next().unwrap();
        current += 1;

        match ch {
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            '"' => {
                let mut temp = String::new();
                let quotes_close_check = |ch: &char| if *ch != '"' {Some(())} else {None};

                while let Some(_) = input_chars.peek().and_then(quotes_close_check) {
                    let ch = input_chars.next().unwrap();
                    current += 1;

                    temp.push(ch);
                }

                current += 1;
                match input_chars.next() {
                    Some(ch) if ch == '"' => tokens.push(Token::Str(temp)),
                    _ => return Err(format!("Expected a \" after {} at {}.", temp, current))
                }

            }
            a if a.is_alphabetic() => {
                let mut temp = String::new();
                temp.push(a);

                let alphanumeric_check = |ch: &char| if ch.is_alphanumeric() { Some(()) } else { None };

                while let Some(_) = input_chars.peek().and_then(alphanumeric_check) {
                    let a = input_chars.next().unwrap();
                    current += 1;
                    temp.push(a);
                };
                
                tokens.push(Token::Name(temp))
            }
            n if n.is_digit(10) => {
                let mut temp = String::new();
                temp.push(n);

                let digit_check = |ch: &char| if ch.is_digit(10) { Some(()) } else { None };

                while let Some(_) = input_chars.peek().and_then(digit_check) {
                    let n = input_chars.next().unwrap();
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

fn parser(input: Vec<Token>) -> Result<(), String> {
    let mut current = 0;
    let tokens = input.into_iter().peekable();
    
    Ok(())
}


fn main() {
    println!("{:?}", tokenizer("add(24 \"String)"));
}

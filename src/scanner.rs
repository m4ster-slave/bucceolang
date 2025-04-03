pub enum Token {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Star,
    BangEqual,
    Bang,
    EqualEqual,
    Equal,
    LessEqual,
    Less,
    GreaterEqual,
    Greater,
    Slash,
    String,
    And,
    Or,
    BitwiseAnd,
    BitwiseOr,
}

pub fn tokenize(input: &str) -> i32 {
    let mut line_number: u32 = 1;
    let mut exit_code: i32 = 0;

    let mut chars = input.chars();

    while let Some(char) = chars.next() {
        match char {
            '(' => println!("LEFT_PAREN ( null"),
            ')' => println!("RIGHT_PAREN ) null"),
            '{' => println!("LEFT_BRACE {{ null"),
            '}' => println!("RIGHT_BRACE }} null"),
            ',' => println!("COMMA , null"),
            '.' => println!("DOT . null"),
            '-' => println!("MINUS - null"),
            '+' => println!("PLUS + null"),
            ';' => println!("SEMICOLON ; null"),
            '*' => println!("STAR * null"),
            '!' => {
                let mut peek = chars.clone().peekable();
                if peek.next() == Some('=') {
                    println!("BANG_EQUAL != null");
                    chars.next();
                } else {
                    println!("BANG ! null");
                }
            }
            '=' => {
                let mut peek = chars.clone().peekable();
                if peek.next() == Some('=') {
                    println!("EQUAL_EQUAL == null");
                    chars.next();
                } else {
                    println!("EQUAL = null");
                }
            }
            '<' => {
                let mut peek = chars.clone().peekable();
                if peek.next() == Some('=') {
                    println!("LESS_EQUAL <= null");
                    chars.next();
                } else {
                    println!("LESS < null");
                }
            }
            '>' => {
                let mut peek = chars.clone().peekable();
                if peek.next() == Some('=') {
                    println!("GREATER_EQUAL >= null");
                    chars.next();
                } else {
                    println!("GREATER > null");
                }
            }
            '/' => {
                let mut peek = chars.clone().peekable();
                if peek.next() == Some('/') {
                    // if we find a comment  loop until the next line
                    while let Some(char) = chars.next() {
                        if char == '\n' {
                            line_number += 1;
                            break;
                        }
                    }
                } else {
                    println!("SLASH / null");
                }
            }
            '\n' => line_number += 1,
            ' ' => continue,
            '\t' => continue,
            '"' => {
                let mut string_literal = String::new();

                loop {
                    if let Some(char) = chars.next() {
                        if char != '"' {
                            string_literal.push(char.clone());

                            if char == '\n' {
                                line_number += 1;
                            }
                        } else if char == '"' {
                            println!("STRING \"{}\" {}", string_literal, string_literal);
                            break;
                        }
                    } else {
                        eprintln!("[line {}] Error: Unterminated string.", line_number);
                        exit_code = 65; // exit code 65 in case of unterminated string
                        break;
                    }
                }
            }
            '&' => {
                let mut peek = chars.clone().peekable();
                if peek.next() == Some('&') {
                    println!("AND && null");
                    chars.next();
                } else {
                    println!("BITWISE_AND & null");
                }
            }
            '|' => {
                let mut peek = chars.clone().peekable();
                if peek.next() == Some('|') {
                    println!("OR && null");
                    chars.next();
                } else {
                    println!("BITWISE_OR | null");
                }
            }
            '0'..='9' => {
                let mut number_literal = String::from(char);
                let mut had_comma: bool = false;

                while let Some(char) = chars.clone().peekable().peek() {
                    if char.is_digit(10) {
                        number_literal.push(*char);
                        chars.next();
                    } else if *char == '.' && !had_comma {
                        had_comma = true;
                        number_literal.push(*char);
                        chars.next();
                    } else {
                        break;
                    }
                }

                if number_literal.ends_with('.') {
                    // number_literal.push('0');
                    print_number_msg(&number_literal);
                    println!("DOT . null");
                } else {
                    print_number_msg(&number_literal);
                }
            }
            _ => {
                if char.is_alphabetic() || char == '_' {
                    let mut identifier = String::from(char);

                    while let Some(char) = chars.clone().peekable().peek() {
                        if char.is_alphabetic() || *char == '_' || char.is_digit(10) {
                            identifier.push(*char);
                            chars.next();
                        } else {
                            break;
                        }
                    }

                    match identifier.as_str() {
                        "else" => println!("ELSE else null"),
                        "false" => println!("FALSE false null"),
                        "for" => println!("FOR for null"),
                        "fun" => println!("FUN fun null"),
                        "if" => println!("IF if null"),
                        "nil" => println!("NIL nil null"),
                        "or" => println!("OR or null"),
                        "print" => println!("PRINT print null"),
                        "return" => println!("RETURN return null"),
                        "super" => println!("SUPER super null"),
                        "this" => println!("THIS this null"),
                        "true" => println!("TRUE true null"),
                        "var" => println!("VAR var null"),
                        "while" => println!("WHILE while null"),
                        "and" => println!("AND and null"),
                        "class" => println!("CLASS class null"),
                        _ => println!("IDENTIFIER {} null", identifier),
                    }
                } else {
                    eprintln!(
                        "[line {}] Error: Unexpected character: {}",
                        line_number, char
                    );
                    exit_code = 65; // exit code 65 in case of lexical errors
                }
            }
        }
    }

    println!("EOF  null"); // Placeholder, remove this line when implementing the scanner
    exit_code
}

fn print_number_msg(number_literal: &str) {
    if number_literal.ends_with(".0") {
        println!(
            "NUMBER {} {}",
            number_literal.replace(".0", ""),
            number_literal
        )
    } else if !number_literal.contains(".") {
        println!("NUMBER {} {}.0", number_literal, number_literal)
    } else if number_literal.ends_with(".") {
        println!(
            "NUMBER {} {}",
            number_literal.replace(".", ""),
            format_number(number_literal)
        )
    } else {
        println!(
            "NUMBER {} {}",
            number_literal,
            format_number(number_literal)
        )
    }
}

fn format_number(number_literal: &str) -> String {
    let num = number_literal.parse::<f32>().unwrap();

    if num.fract() == 0.0 {
        format!("{:.1}", num)
    } else {
        num.to_string()
    }
}

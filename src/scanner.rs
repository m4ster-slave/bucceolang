use crate::{
    object::Object,
    scanner_error::ScannerError,
    token::{Token, TokenType},
};

pub fn tokenize(input: &str) -> Result<Vec<Token>, ScannerError> {
    let mut out: Vec<Token> = Vec::new();
    let mut line_number: u64 = 1;

    let mut chars = input.chars();
    while let Some(char) = chars.next() {
        match char {
            '(' => out.push(Token::new(TokenType::LeftParen, "(", None, line_number)),
            ')' => out.push(Token::new(TokenType::RightParen, ")", None, line_number)),
            '{' => out.push(Token::new(TokenType::LeftBrace, "{", None, line_number)),
            '}' => out.push(Token::new(TokenType::RightBrace, "}", None, line_number)),
            ',' => out.push(Token::new(TokenType::Comma, ",", None, line_number)),
            '.' => out.push(Token::new(TokenType::Dot, ".", None, line_number)),
            '-' => out.push(Token::new(TokenType::Minus, "-", None, line_number)),
            '+' => out.push(Token::new(TokenType::Plus, "+", None, line_number)),
            ';' => out.push(Token::new(TokenType::Semicolon, ";", None, line_number)),
            '*' => out.push(Token::new(TokenType::Asterisk, "*", None, line_number)),
            '!' => {
                let mut peek = chars.clone().peekable();
                if peek.next() == Some('=') {
                    out.push(Token::new(TokenType::BangEqual, "!=", None, line_number));
                    chars.next();
                } else {
                    out.push(Token::new(TokenType::Bang, "!", None, line_number));
                }
            }
            '=' => {
                let mut peek = chars.clone().peekable();
                if peek.next() == Some('=') {
                    out.push(Token::new(TokenType::EqualEqual, "==", None, line_number));
                    chars.next();
                } else {
                    out.push(Token::new(TokenType::Equal, "=", None, line_number));
                }
            }
            '<' => {
                let mut peek = chars.clone().peekable();
                if peek.next() == Some('=') {
                    out.push(Token::new(TokenType::LessEqual, "<=", None, line_number));
                    chars.next();
                } else {
                    out.push(Token::new(TokenType::Less, "<", None, line_number));
                }
            }
            '>' => {
                let mut peek = chars.clone().peekable();
                if peek.next() == Some('=') {
                    out.push(Token::new(TokenType::GreaterEqual, ">=", None, line_number));
                    chars.next();
                } else {
                    out.push(Token::new(TokenType::Greater, ">", None, line_number));
                }
            }
            '/' => {
                let mut peek = chars.clone().peekable();
                if peek.next() == Some('/') {
                    // if we find a comment  loop until the next line
                    for char in chars.by_ref() {
                        if char == '\n' {
                            line_number += 1;
                            break;
                        }
                    }
                } else {
                    out.push(Token::new(TokenType::Slash, "/", None, line_number));
                }
            }
            '\n' => line_number += 1,
            ' ' => continue,
            '\t' => continue,
            '"' => {
                let mut string_literal = String::new();
                let mut escaped = false;

                loop {
                    if let Some(char) = chars.next() {
                        if escaped {
                            match char {
                                '"' => string_literal.push('"'),
                                '\\' => string_literal.push('\\'),
                                'n' => string_literal.push('\n'),
                                't' => string_literal.push('\t'),
                                'r' => string_literal.push('\r'),
                                _ => {
                                    string_literal.push('\\');
                                    string_literal.push(char);
                                }
                            }
                            escaped = false;
                        } else if char == '\\' {
                            escaped = true;
                        } else if char == '"' {
                            // Create string object here when we have Object impl
                            let value = Some(Object::String(string_literal.clone()));
                            out.push(Token::new(
                                TokenType::String,
                                &format!("\"{}\"", string_literal),
                                value,
                                line_number,
                            ));
                            break;
                        } else {
                            string_literal.push(char);

                            if char == '\n' {
                                line_number += 1;
                            }
                        }
                    } else {
                        return Err(ScannerError::InvalidSyntax(
                            line_number,
                            "Unterminated string".to_string(),
                        ));
                    }
                }
            }
            '&' => {
                let mut peek = chars.clone().peekable();
                if peek.next() == Some('&') {
                    out.push(Token::new(TokenType::And, "&&", None, line_number));
                    chars.next();
                } else {
                    out.push(Token::new(TokenType::BitwiseAnd, "&", None, line_number));
                }
            }
            '|' => {
                let mut peek = chars.clone().peekable();
                if peek.next() == Some('|') {
                    out.push(Token::new(TokenType::Or, "||", None, line_number));
                    chars.next();
                } else {
                    out.push(Token::new(TokenType::BitwiseOr, "|", None, line_number));
                }
            }
            '0'..='9' => {
                let mut number_literal = String::from(char);
                let mut had_comma: bool = false;

                while let Some(char) = chars.clone().peekable().peek() {
                    if char.is_ascii_digit() {
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

                // Create number object here when we have Object impl
                let value = Some(Object::Number(number_literal.parse().unwrap_or(0.0)));
                out.push(Token::new(
                    TokenType::Number,
                    &number_literal,
                    value,
                    line_number,
                ));
            }
            _ => {
                if char.is_alphabetic() || char == '_' {
                    let mut identifier = String::from(char);

                    while let Some(char) = chars.clone().peekable().peek() {
                        if char.is_alphabetic() || *char == '_' || char.is_ascii_digit() {
                            identifier.push(*char);
                            chars.next();
                        } else {
                            break;
                        }
                    }

                    match identifier.as_str() {
                        "else" => out.push(Token::new(TokenType::Else, "else", None, line_number)),
                        "false" => {
                            let value = Some(Object::Boolean(false));
                            out.push(Token::new(TokenType::False, "false", value, line_number))
                        }
                        "for" => out.push(Token::new(TokenType::For, "for", None, line_number)),
                        "fun" => out.push(Token::new(TokenType::Fn, "fun", None, line_number)),
                        "if" => out.push(Token::new(TokenType::If, "if", None, line_number)),
                        "nil" => {
                            let value = Some(Object::Nil);
                            out.push(Token::new(TokenType::Nil, "nil", value, line_number))
                        }
                        "print" => {
                            out.push(Token::new(TokenType::Print, "print", None, line_number))
                        }
                        "return" => {
                            out.push(Token::new(TokenType::Return, "return", None, line_number))
                        }
                        "super" => {
                            out.push(Token::new(TokenType::Super, "super", None, line_number))
                        }
                        "this" => out.push(Token::new(TokenType::This, "this", None, line_number)),
                        "true" => {
                            let value = Some(Object::Boolean(true));
                            out.push(Token::new(TokenType::True, "true", value, line_number))
                        }
                        "var" => {
                            out.push(Token::new(TokenType::VarKeyword, "var", None, line_number));

                            let mut var = String::new();
                            //skip whitespace to find variable name
                            let mut var_char_opt = chars.next();

                            // Skip any whitespace before the variable name
                            while let Some(var_char) = var_char_opt {
                                if var_char == ' ' || var_char == '\t' {
                                    var_char_opt = chars.next();
                                } else {
                                    break;
                                }
                            }

                            // If we have a character after the whitespace, it's the start of the variable name
                            if let Some(var_char) = var_char_opt {
                                if is_valid_variable_char(var_char, true) {
                                    var.push(var_char);

                                    // Parse the rest of the variable name
                                    while let Some(char) = chars.clone().peekable().peek() {
                                        if is_valid_variable_char(*char, false) {
                                            var.push(*char);
                                            chars.next();
                                        } else {
                                            break;
                                        }
                                    }

                                    if is_keyword(&var) {
                                        return Err(ScannerError::InvalidVariableName(
                                            line_number,
                                            format!("Variable name is a keyword: {}", var),
                                        ));
                                    }

                                    out.push(Token::new(TokenType::Var, &var, None, line_number));
                                } else {
                                    return Err(ScannerError::InvalidVariableName(
                                        line_number,
                                        format!(
                                            "Invalid variable name start character: {}",
                                            var_char
                                        ),
                                    ));
                                }
                            } else {
                                return Err(ScannerError::InvalidSyntax(
                                    line_number,
                                    "Expected variable name after 'var' keyword".to_string(),
                                ));
                            }
                        }
                        "while" => {
                            out.push(Token::new(TokenType::While, "while", None, line_number))
                        }
                        "class" => {
                            out.push(Token::new(TokenType::Class, "class", None, line_number))
                        }
                        _ => {
                            out.push(Token::new(TokenType::Var, &identifier, None, line_number));
                        }
                    }
                } else {
                    let mut identifier = String::from(char);

                    while let Some(char) = chars.clone().peekable().peek() {
                        if *char != ' ' && *char != '\t' && *char != '\n' {
                            identifier.push(*char);
                            chars.next();
                        } else {
                            break;
                        }
                    }

                    return Err(ScannerError::InvalidSyntax(
                        line_number,
                        format!("Unexpected identifier: {}", identifier),
                    ));
                }
            }
        }
    }
    out.push(Token::new(TokenType::EOF, "", None, line_number));

    Ok(out)
}

fn is_valid_variable_char(c: char, is_first_char: bool) -> bool {
    match c {
        'a'..='z' | 'A'..='Z' | '_' => true,
        '0'..='9' => !is_first_char,
        _ => false,
    }
}

fn is_keyword(var: &str) -> bool {
    vec![
        "else", "false", "for", "fun", "if", "nil", "print", "return", "super", "this", "true",
        "var", "while", "class",
    ]
    .contains(&var)
}

#[cfg(test)]
mod tests {
    use crate::scanner::*;
    use crate::token::TokenType;

    #[test]
    fn test_is_keyword() {
        assert!(is_keyword("while"));
        assert!(!is_keyword("num0"));
    }

    #[test]
    fn test_is_valid_variabl_char() {
        assert!(is_valid_variable_char('a', true));
        assert!(is_valid_variable_char('A', false));
        assert!(!is_valid_variable_char('0', true));
        assert!(is_valid_variable_char('1', false));
        assert!(!is_valid_variable_char('&', false));
        assert!(is_valid_variable_char('_', false));
    }

    #[test]
    fn test_simple_tokens() {
        let input = "(){},.+-;*";
        let tokens = tokenize(input).unwrap();
        assert_eq!(tokens.len(), 11); // +1 for EOF
        assert!(matches!(tokens[0].token_type(), TokenType::LeftParen));
        assert!(matches!(tokens[1].token_type(), TokenType::RightParen));
        assert!(matches!(tokens[2].token_type(), TokenType::LeftBrace));
        assert!(matches!(tokens[3].token_type(), TokenType::RightBrace));
        assert!(matches!(tokens[4].token_type(), TokenType::Comma));
        assert!(matches!(tokens[5].token_type(), TokenType::Dot));
        assert!(matches!(tokens[6].token_type(), TokenType::Plus));
        assert!(matches!(tokens[7].token_type(), TokenType::Minus));
        assert!(matches!(tokens[8].token_type(), TokenType::Semicolon));
        assert!(matches!(tokens[9].token_type(), TokenType::Asterisk));
    }

    #[test]
    fn test_comparison_operators() {
        let input = "< <= > >= == != = !";
        let tokens = tokenize(input).unwrap();
        assert_eq!(tokens.len(), 9); // +1 for EOF
        assert!(matches!(tokens[0].token_type(), TokenType::Less));
        assert!(matches!(tokens[1].token_type(), TokenType::LessEqual));
        assert!(matches!(tokens[2].token_type(), TokenType::Greater));
        assert!(matches!(tokens[3].token_type(), TokenType::GreaterEqual));
        assert!(matches!(tokens[4].token_type(), TokenType::EqualEqual));
        assert!(matches!(tokens[5].token_type(), TokenType::BangEqual));
        assert!(matches!(tokens[6].token_type(), TokenType::Equal));
        assert!(matches!(tokens[7].token_type(), TokenType::Bang));
    }

    #[test]
    fn test_logical_operators() {
        let input = "&& || & |";
        let tokens = tokenize(input).unwrap();
        assert_eq!(tokens.len(), 5); // +1 for EOF
        assert!(matches!(tokens[0].token_type(), TokenType::And));
        assert!(matches!(tokens[1].token_type(), TokenType::Or));
        assert!(matches!(tokens[2].token_type(), TokenType::BitwiseAnd));
        assert!(matches!(tokens[3].token_type(), TokenType::BitwiseOr));
    }

    #[test]
    fn test_string_literals() {
        let input = "\"hello world\" \"test\"";
        let tokens = tokenize(input).unwrap();
        assert_eq!(tokens.len(), 3); // +1 for EOF
        assert!(matches!(tokens[0].token_type(), TokenType::String));
        assert_eq!(tokens[0].lexeme(), "\"hello world\"");
        assert!(matches!(tokens[1].token_type(), TokenType::String));
        assert_eq!(tokens[1].lexeme(), "\"test\"");
    }

    #[test]
    fn test_number_literals() {
        let input = "123 45.67 0 9.0";
        let tokens = tokenize(input).unwrap();
        assert_eq!(tokens.len(), 5); // +1 for EOF
        assert!(matches!(tokens[0].token_type(), TokenType::Number));
        assert_eq!(tokens[0].lexeme(), "123");
        assert!(matches!(tokens[1].token_type(), TokenType::Number));
        assert_eq!(tokens[1].lexeme(), "45.67");
        assert!(matches!(tokens[2].token_type(), TokenType::Number));
        assert_eq!(tokens[2].lexeme(), "0");
        assert!(matches!(tokens[3].token_type(), TokenType::Number));
        assert_eq!(tokens[3].lexeme(), "9.0");
    }

    #[test]
    fn test_keywords() {
        let input = "else false for fun if nil print return super this true while class";
        let tokens = tokenize(input).unwrap();
        assert_eq!(tokens.len(), 14); // +1 for EOF
        assert!(matches!(tokens[0].token_type(), TokenType::Else));
        assert!(matches!(tokens[1].token_type(), TokenType::False));
        assert!(matches!(tokens[2].token_type(), TokenType::For));
        assert!(matches!(tokens[3].token_type(), TokenType::Fn));
        assert!(matches!(tokens[4].token_type(), TokenType::If));
        assert!(matches!(tokens[5].token_type(), TokenType::Nil));
        assert!(matches!(tokens[6].token_type(), TokenType::Print));
        assert!(matches!(tokens[7].token_type(), TokenType::Return));
        assert!(matches!(tokens[8].token_type(), TokenType::Super));
        assert!(matches!(tokens[9].token_type(), TokenType::This));
        assert!(matches!(tokens[10].token_type(), TokenType::True));
        assert!(matches!(tokens[11].token_type(), TokenType::While));
        assert!(matches!(tokens[12].token_type(), TokenType::Class));
    }

    #[test]
    fn test_var_declaration() {
        let input = "var myVar";
        let tokens = tokenize(input).unwrap();
        assert_eq!(tokens.len(), 3); // +1 for EOF
        assert!(matches!(tokens[0].token_type(), TokenType::VarKeyword));
        assert!(matches!(tokens[1].token_type(), TokenType::Var));
        assert_eq!(tokens[1].lexeme(), "myVar");
    }

    #[test]
    fn test_var_reference() {
        let input = "var myVar\nmyVar";
        let tokens = tokenize(input).unwrap();
        assert_eq!(tokens.len(), 4); // +1 for EOF
        assert!(matches!(tokens[0].token_type(), TokenType::VarKeyword));
        assert!(matches!(tokens[1].token_type(), TokenType::Var));
        assert_eq!(tokens[1].lexeme(), "myVar");
        assert!(matches!(tokens[2].token_type(), TokenType::Var));
        assert_eq!(tokens[2].lexeme(), "myVar");
    }

    #[test]
    fn test_comments() {
        let input = "// This is a comment\n123";
        let tokens = tokenize(input).unwrap();
        assert_eq!(tokens.len(), 2); // +1 for EOF
        assert!(matches!(tokens[0].token_type(), TokenType::Number));
        assert_eq!(tokens[0].lexeme(), "123");
    }

    #[test]
    fn test_whitespace() {
        let input = "  \t\n123\n  456  ";
        let tokens = tokenize(input).unwrap();
        assert_eq!(tokens.len(), 3); // +1 for EOF
        assert!(matches!(tokens[0].token_type(), TokenType::Number));
        assert_eq!(tokens[0].lexeme(), "123");
        assert!(matches!(tokens[1].token_type(), TokenType::Number));
        assert_eq!(tokens[1].lexeme(), "456");
    }

    #[test]
    fn test_complex_program() {
        let input = r#"
        // A simple program
        var answer = 42;
        if (answer == 42) {
            print "The answer!";
        } else {
            print "Not the answer!";
        }
        "#;

        let tokens = tokenize(input).unwrap();
        // Check for expected tokens without asserting the exact count
        // Just verify that important tokens are present in correct order
        let mut iter = tokens.iter();

        assert!(matches!(
            iter.next().unwrap().token_type(),
            TokenType::VarKeyword
        ));
        let var_token = iter.next().unwrap();
        assert!(matches!(var_token.token_type(), TokenType::Var));
        assert_eq!(var_token.lexeme(), "answer");
        assert!(matches!(
            iter.next().unwrap().token_type(),
            TokenType::Equal
        ));
        let num_token = iter.next().unwrap();
        assert!(matches!(num_token.token_type(), TokenType::Number));
        assert_eq!(num_token.lexeme(), "42");
        assert!(matches!(
            iter.next().unwrap().token_type(),
            TokenType::Semicolon
        ));
        assert!(matches!(iter.next().unwrap().token_type(), TokenType::If));
        assert!(matches!(
            iter.next().unwrap().token_type(),
            TokenType::LeftParen
        ));
        let var_token = iter.next().unwrap();
        assert!(matches!(var_token.token_type(), TokenType::Var));
        assert_eq!(var_token.lexeme(), "answer");
        assert!(matches!(
            iter.next().unwrap().token_type(),
            TokenType::EqualEqual
        ));
        let num_token = iter.next().unwrap();
        assert!(matches!(num_token.token_type(), TokenType::Number));
        assert_eq!(num_token.lexeme(), "42");
        assert!(matches!(
            iter.next().unwrap().token_type(),
            TokenType::RightParen
        ));
        assert!(matches!(
            iter.next().unwrap().token_type(),
            TokenType::LeftBrace
        ));
        assert!(matches!(
            iter.next().unwrap().token_type(),
            TokenType::Print
        ));
        let str_token = iter.next().unwrap();
        assert!(matches!(str_token.token_type(), TokenType::String));
        assert_eq!(str_token.lexeme(), "\"The answer!\"");
        assert!(matches!(
            iter.next().unwrap().token_type(),
            TokenType::Semicolon
        ));
        assert!(matches!(
            iter.next().unwrap().token_type(),
            TokenType::RightBrace
        ));
        assert!(matches!(iter.next().unwrap().token_type(), TokenType::Else));
        assert!(matches!(
            iter.next().unwrap().token_type(),
            TokenType::LeftBrace
        ));
        assert!(matches!(
            iter.next().unwrap().token_type(),
            TokenType::Print
        ));
        let str_token = iter.next().unwrap();
        assert!(matches!(str_token.token_type(), TokenType::String));
        assert_eq!(str_token.lexeme(), "\"Not the answer!\"");
        assert!(matches!(
            iter.next().unwrap().token_type(),
            TokenType::Semicolon
        ));
        assert!(matches!(
            iter.next().unwrap().token_type(),
            TokenType::RightBrace
        ));
        assert!(matches!(iter.next().unwrap().token_type(), TokenType::EOF));

        assert!(iter.next().is_none());
    }

    #[test]
    fn test_unterminated_string() {
        let input = "\"unterminated string";
        let result = tokenize(input);
        assert!(result.is_err());
        match result {
            Err(ScannerError::InvalidSyntax(_, msg)) => {
                assert!(msg.contains("Unterminated string"));
            }
            _ => panic!("Expected InvalidSyntax error"),
        }
    }

    #[test]
    fn test_var_keyword_name() {
        let input = "var if";
        let result = tokenize(input);
        assert!(result.is_err());
        match result {
            Err(ScannerError::InvalidVariableName(_, msg)) => {
                assert!(msg.contains("Variable name is a keyword"));
            }
            _ => panic!("Expected InvalidVariableName error"),
        }
    }

    #[test]
    fn test_unexpected_identifier() {
        let input = "@invalid";
        let result = tokenize(input);
        assert!(result.is_err());
        match result {
            Err(ScannerError::InvalidSyntax(_, msg)) => {
                assert!(msg.contains("Unexpected identifier"));
            }
            _ => panic!("Expected InvalidSyntax error"),
        }
    }

    #[test]
    fn test_consecutive_var_declarations() {
        let input = "var x\nvar y";
        let tokens = tokenize(input).unwrap();
        assert_eq!(tokens.len(), 5); // +1 for EOF
        assert!(matches!(tokens[0].token_type(), TokenType::VarKeyword));
        let var_token = &tokens[1];
        assert!(matches!(var_token.token_type(), TokenType::Var));
        assert_eq!(var_token.lexeme(), "x");
        assert!(matches!(tokens[2].token_type(), TokenType::VarKeyword));
        let var_token = &tokens[3];
        assert!(matches!(var_token.token_type(), TokenType::Var));
        assert_eq!(var_token.lexeme(), "y");
    }

    #[test]
    fn test_string_with_escaped_quotes() {
        let input = r#""String with \"quotes\"";"#;
        let tokens = tokenize(input).unwrap();
        assert!(matches!(tokens[0].token_type(), TokenType::String));
        assert_eq!(tokens[0].lexeme(), "\"String with \"quotes\"\"");
    }

    #[test]
    fn test_multiline_string() {
        let input = "\"String with\nmultiple\nlines\"";
        let tokens = tokenize(input).unwrap();
        assert_eq!(tokens.len(), 2); // +1 for EOF
        assert!(matches!(tokens[0].token_type(), TokenType::String));
        assert_eq!(tokens[0].lexeme(), "\"String with\nmultiple\nlines\"");
    }

    #[test]
    fn test_has_eof() {
        let input = "\"Test\"";
        let tokens = tokenize(input).unwrap();
        assert_eq!(tokens.len(), 2);
        assert!(matches!(tokens[1].token_type(), TokenType::EOF));
    }

    #[test]
    fn test_missing_var_name() {
        let input = "var ";
        let result = tokenize(input);
        assert!(result.is_err());
        match result {
            Err(ScannerError::InvalidSyntax(_, msg)) => {
                assert!(msg.contains("Expected variable name after 'var' keyword"));
            }
            _ => panic!("Expected InvalidSyntax error"),
        }
    }

    #[test]
    fn test_invalid_var_name_start() {
        let input = "var 1invalid";
        let result = tokenize(input);
        assert!(result.is_err());
        match result {
            Err(ScannerError::InvalidVariableName(_, msg)) => {
                assert!(msg.contains("Invalid variable name start character"));
            }
            _ => panic!("Expected InvalidVariableName error"),
        }
    }
}

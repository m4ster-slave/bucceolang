pub mod scanner {
    #[derive(Debug, Clone, PartialEq)]
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
        Asterisk,
        BangEqual,
        Bang,
        EqualEqual,
        Equal,
        LessEqual,
        Less,
        GreaterEqual,
        Greater,
        Slash,
        And,
        Or,
        BitwiseAnd,
        BitwiseOr,
        String(String),
        Number(String),
        Else,
        False,
        For,
        Fn,
        If,
        Nil,
        Print,
        Return,
        Super,
        This,
        True,
        VarKeyword,
        Var(String),
        While,
        Class,
        EOF,
    }

    pub fn tokenize(input: &str) -> Vec<Token> {
        let mut out = Vec::new();
        let mut line_number: u32 = 1;

        let mut chars = input.chars();
        while let Some(char) = chars.next() {
            match char {
                '(' => out.push(Token::LeftParen),
                ')' => out.push(Token::RightParen),
                '{' => out.push(Token::LeftBrace),
                '}' => out.push(Token::RightBrace),
                ',' => out.push(Token::Comma),
                '.' => out.push(Token::Dot),
                '-' => out.push(Token::Minus),
                '+' => out.push(Token::Plus),
                ';' => out.push(Token::Semicolon),
                '*' => out.push(Token::Asterisk),
                '!' => {
                    let mut peek = chars.clone().peekable();
                    if peek.next() == Some('=') {
                        out.push(Token::BangEqual);
                        chars.next();
                    } else {
                        out.push(Token::Bang);
                    }
                }
                '=' => {
                    let mut peek = chars.clone().peekable();
                    if peek.next() == Some('=') {
                        out.push(Token::EqualEqual);
                        chars.next();
                    } else {
                        out.push(Token::Equal);
                    }
                }
                '<' => {
                    let mut peek = chars.clone().peekable();
                    if peek.next() == Some('=') {
                        out.push(Token::LessEqual);
                        chars.next();
                    } else {
                        out.push(Token::Less);
                    }
                }
                '>' => {
                    let mut peek = chars.clone().peekable();
                    if peek.next() == Some('=') {
                        out.push(Token::GreaterEqual);
                        chars.next();
                    } else {
                        out.push(Token::Greater);
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
                        out.push(Token::Slash);
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
                                out.push(Token::String(string_literal));
                                break;
                            } else {
                                string_literal.push(char);

                                if char == '\n' {
                                    line_number += 1;
                                }
                            }
                        } else {
                            panic!("[line {}] Error: Unterminated string.", line_number);
                        }
                    }
                }
                '&' => {
                    let mut peek = chars.clone().peekable();
                    if peek.next() == Some('&') {
                        out.push(Token::And);
                        chars.next();
                    } else {
                        out.push(Token::BitwiseAnd);
                    }
                }
                '|' => {
                    let mut peek = chars.clone().peekable();
                    if peek.next() == Some('|') {
                        out.push(Token::Or);
                        chars.next();
                    } else {
                        out.push(Token::BitwiseOr);
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
                    out.push(Token::Number(number_literal));
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
                            "else" => out.push(Token::Else),
                            "false" => out.push(Token::False),
                            "for" => out.push(Token::For),
                            "fun" => out.push(Token::Fn),
                            "if" => out.push(Token::If),
                            "nil" => out.push(Token::Nil),
                            "print" => out.push(Token::Print),
                            "return" => out.push(Token::Return),
                            "super" => out.push(Token::Super),
                            "this" => out.push(Token::This),
                            "true" => out.push(Token::True),
                            "var" => {
                                out.push(Token::VarKeyword);

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
                                    if is_valid_variable_char(var_char, false) {
                                        var.push(var_char);

                                        // Parse the rest of the variable name
                                        while let Some(char) = chars.clone().peekable().peek() {
                                            if is_valid_variable_char(*char, true) {
                                                var.push(*char);
                                                chars.next();
                                            } else {
                                                break;
                                            }
                                        }

                                        if is_keyword(&var) {
                                            panic!(
                                                "[line {}] Error: Variable name is a keyword: {}",
                                                line_number, var
                                            );
                                        }

                                        out.push(Token::Var(var));
                                    } else {
                                        panic!(
                                            "[line {}] Error: Invalid variable name start character: {}",
                                            line_number, var_char
                                        );
                                    }
                                } else {
                                    panic!(
                                        "[line {}] Error: Expected variable name after 'var' keyword",
                                        line_number
                                    );
                                }
                            }
                            "while" => out.push(Token::While),
                            "class" => out.push(Token::Class),
                            _ => {
                                out.push(Token::Var(identifier));
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

                        panic!(
                            "[line {}] Error: Unexpected identifier: {}",
                            line_number, identifier
                        );
                    }
                }
            }
        }
        out.push(Token::EOF);

        out
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
        use crate::scanner::scanner::*;

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
            let tokens = tokenize(input);
            assert_eq!(tokens.len(), 11);
            assert!(matches!(tokens[0], Token::LeftParen));
            assert!(matches!(tokens[1], Token::RightParen));
            assert!(matches!(tokens[2], Token::LeftBrace));
            assert!(matches!(tokens[3], Token::RightBrace));
            assert!(matches!(tokens[4], Token::Comma));
            assert!(matches!(tokens[5], Token::Dot));
            assert!(matches!(tokens[6], Token::Plus));
            assert!(matches!(tokens[7], Token::Minus));
            assert!(matches!(tokens[8], Token::Semicolon));
            assert!(matches!(tokens[9], Token::Asterisk));
        }

        #[test]
        fn test_comparison_operators() {
            let input = "< <= > >= == != = !";
            let tokens = tokenize(input);
            assert_eq!(tokens.len(), 9);
            assert!(matches!(tokens[0], Token::Less));
            assert!(matches!(tokens[1], Token::LessEqual));
            assert!(matches!(tokens[2], Token::Greater));
            assert!(matches!(tokens[3], Token::GreaterEqual));
            assert!(matches!(tokens[4], Token::EqualEqual));
            assert!(matches!(tokens[5], Token::BangEqual));
            assert!(matches!(tokens[6], Token::Equal));
            assert!(matches!(tokens[7], Token::Bang));
        }

        #[test]
        fn test_logical_operators() {
            let input = "&& || & |";
            let tokens = tokenize(input);
            assert_eq!(tokens.len(), 5);
            assert!(matches!(tokens[0], Token::And));
            assert!(matches!(tokens[1], Token::Or));
            assert!(matches!(tokens[2], Token::BitwiseAnd));
            assert!(matches!(tokens[3], Token::BitwiseOr));
        }

        #[test]
        fn test_string_literals() {
            let input = "\"hello world\" \"test\"";
            let tokens = tokenize(input);
            assert_eq!(tokens.len(), 3);
            assert!(matches!(&tokens[0], Token::String(s) if s == "hello world"));
            assert!(matches!(&tokens[1], Token::String(s) if s == "test"));
        }

        #[test]
        fn test_number_literals() {
            let input = "123 45.67 0 9.0";
            let tokens = tokenize(input);
            assert_eq!(tokens.len(), 5);
            assert!(matches!(&tokens[0], Token::Number(s) if s == "123"));
            assert!(matches!(&tokens[1], Token::Number(s) if s == "45.67"));
            assert!(matches!(&tokens[2], Token::Number(s) if s == "0"));
            assert!(matches!(&tokens[3], Token::Number(s) if s == "9.0"));
        }

        #[test]
        fn test_keywords() {
            let input = "else false for fun if nil print return super this true while class";
            let tokens = tokenize(input);
            assert_eq!(tokens.len(), 14);
            assert!(matches!(tokens[0], Token::Else));
            assert!(matches!(tokens[1], Token::False));
            assert!(matches!(tokens[2], Token::For));
            assert!(matches!(tokens[3], Token::Fn));
            assert!(matches!(tokens[4], Token::If));
            assert!(matches!(tokens[5], Token::Nil));
            assert!(matches!(tokens[6], Token::Print));
            assert!(matches!(tokens[7], Token::Return));
            assert!(matches!(tokens[8], Token::Super));
            assert!(matches!(tokens[9], Token::This));
            assert!(matches!(tokens[10], Token::True));
            assert!(matches!(tokens[11], Token::While));
            assert!(matches!(tokens[12], Token::Class));
        }

        #[test]
        fn test_var_declaration() {
            let input = "var myVar";
            let tokens = tokenize(input);
            assert_eq!(tokens.len(), 3);
            assert!(matches!(tokens[0], Token::VarKeyword));
            assert!(matches!(&tokens[1], Token::Var(s) if s == "myVar"));
        }

        #[test]
        fn test_var_reference() {
            let input = "var myVar\nmyVar";
            let tokens = tokenize(input);
            assert_eq!(tokens.len(), 4);
            assert!(matches!(tokens[0], Token::VarKeyword));
            assert!(matches!(&tokens[1], Token::Var(s) if s == "myVar"));
            assert!(matches!(&tokens[2], Token::Var(s) if s == "myVar"));
        }

        #[test]
        fn test_comments() {
            let input = "// This is a comment\n123";
            let tokens = tokenize(input);
            assert_eq!(tokens.len(), 2);
            assert!(matches!(&tokens[0], Token::Number(s) if s == "123"));
        }

        #[test]
        fn test_whitespace() {
            let input = "  \t\n123\n  456  ";
            let tokens = tokenize(input);
            assert_eq!(tokens.len(), 3);
            assert!(matches!(&tokens[0], Token::Number(s) if s == "123"));
            assert!(matches!(&tokens[1], Token::Number(s) if s == "456"));
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

            let tokens = tokenize(input);
            // Check for expected tokens without asserting the exact count
            // Just verify that important tokens are present in correct order
            let mut iter = tokens.iter();

            assert!(matches!(iter.next(), Some(Token::VarKeyword)));
            assert!(matches!(iter.next(), Some(Token::Var(s)) if s == "answer"));
            assert!(matches!(iter.next(), Some(Token::Equal)));
            assert!(matches!(iter.next(), Some(Token::Number(s)) if s == "42"));
            assert!(matches!(iter.next(), Some(Token::Semicolon)));
            assert!(matches!(iter.next(), Some(Token::If)));
            assert!(matches!(iter.next(), Some(Token::LeftParen)));
            assert!(matches!(iter.next(), Some(Token::Var(s)) if s == "answer"));
            assert!(matches!(iter.next(), Some(Token::EqualEqual)));
            assert!(matches!(iter.next(), Some(Token::Number(s)) if s == "42"));
            assert!(matches!(iter.next(), Some(Token::RightParen)));
            assert!(matches!(iter.next(), Some(Token::LeftBrace)));
            assert!(matches!(iter.next(), Some(Token::Print)));
            assert!(matches!(iter.next(), Some(Token::String(s)) if s == "The answer!"));
            assert!(matches!(iter.next(), Some(Token::Semicolon)));
            assert!(matches!(iter.next(), Some(Token::RightBrace)));
            assert!(matches!(iter.next(), Some(Token::Else)));
            assert!(matches!(iter.next(), Some(Token::LeftBrace)));
            assert!(matches!(iter.next(), Some(Token::Print)));
            assert!(matches!(iter.next(), Some(Token::String(s)) if s == "Not the answer!"));
            assert!(matches!(iter.next(), Some(Token::Semicolon)));
            assert!(matches!(iter.next(), Some(Token::RightBrace)));
            assert!(matches!(iter.next(), Some(Token::EOF)));

            assert!(iter.next().is_none());
        }

        #[test]
        #[should_panic(expected = "Error: Unterminated string")]
        fn test_unterminated_string() {
            let input = "\"unterminated string";
            tokenize(input);
        }

        #[test]
        #[should_panic(expected = "Error: Variable name is a keyword")]
        fn test_var_keyword_name() {
            let input = "var if";
            tokenize(input);
        }

        #[test]
        #[should_panic(expected = "Error: Unexpected identifier")]
        fn test_unexpected_identifier() {
            let input = "@invalid";
            tokenize(input);
        }

        #[test]
        fn test_consecutive_var_declarations() {
            let input = "var x\nvar y";
            let tokens = tokenize(input);
            assert_eq!(tokens.len(), 5);
            assert!(matches!(tokens[0], Token::VarKeyword));
            assert!(matches!(&tokens[1], Token::Var(s) if s == "x"));
            assert!(matches!(tokens[2], Token::VarKeyword));
            assert!(matches!(&tokens[3], Token::Var(s) if s == "y"));
        }

        #[test]
        fn test_string_with_escaped_quotes() {
            let input = r#""String with \"quotes\"";"#;
            tokenize(input);
        }

        #[test]
        fn test_multiline_string() {
            let input = "\"String with\nmultiple\nlines\"";
            let tokens = tokenize(input);
            assert_eq!(tokens.len(), 2);
            assert!(matches!(&tokens[0], Token::String(s) if s == "String with\nmultiple\nlines"));
        }

        #[test]
        fn test_has_eof() {
            let input = "\"Test\"";
            let tokens = tokenize(input);
            assert_eq!(tokens.len(), 2);
            assert!(matches!(&tokens[1], Token::EOF));
        }
    }
}

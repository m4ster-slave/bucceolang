#[cfg(test)]
mod test {
    use crate::interpreter::Interpreter;
    use crate::parser::parse;
    use crate::resolver::Resolver;
    use crate::scanner::tokenize;

    use std::{cell::RefCell, rc::Rc, str};

    #[test]
    fn test_io_functions() {
        let source = r#"
        "#;

        let tokens = tokenize(source).expect("Tokenization failed");
        let mut stmts = parse(tokens).expect("Parsing failed");

        let output: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(Vec::new()));
        let output_for_interp = output.clone();

        let mut interpreter = Interpreter::new_with_output(output_for_interp);
        let mut resolver = Resolver::new(&mut interpreter);
        resolver.resolve(&mut stmts).expect("Resolving failed");

        let interpreter_result = interpreter.interprete(&mut stmts);
        assert!(
            interpreter_result.is_ok(),
            "Interpreter failed: {:?}",
            interpreter_result.err()
        );
        match interpreter_result {
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        };

        let bytes: std::cell::Ref<'_, Vec<u8>> = output.borrow();
        let result = match str::from_utf8(&bytes) {
            Ok(v) => v.to_owned(),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        assert_eq!(result, "");
    }

    #[test]
    fn test_math_functions() {
        let source = r#"
        "#;

        let tokens = tokenize(source).expect("Tokenization failed");
        let mut stmts = parse(tokens).expect("Parsing failed");

        let output: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(Vec::new()));
        let output_for_interp = output.clone();

        let mut interpreter = Interpreter::new_with_output(output_for_interp);
        let mut resolver = Resolver::new(&mut interpreter);
        resolver.resolve(&mut stmts).expect("Resolving failed");

        let interpreter_result = interpreter.interprete(&mut stmts);
        assert!(
            interpreter_result.is_ok(),
            "Interpreter failed: {:?}",
            interpreter_result.err()
        );
        match interpreter_result {
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        };

        let bytes: std::cell::Ref<'_, Vec<u8>> = output.borrow();
        let result = match str::from_utf8(&bytes) {
            Ok(v) => v.to_owned(),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        assert_eq!(result, "");
    }

    #[test]
    fn test_network_functions() {
        let source = r#"
        "#;

        let tokens = tokenize(source).expect("Tokenization failed");
        let mut stmts = parse(tokens).expect("Parsing failed");

        let output: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(Vec::new()));
        let output_for_interp = output.clone();

        let mut interpreter = Interpreter::new_with_output(output_for_interp);
        let mut resolver = Resolver::new(&mut interpreter);
        resolver.resolve(&mut stmts).expect("Resolving failed");

        let interpreter_result = interpreter.interprete(&mut stmts);
        assert!(
            interpreter_result.is_ok(),
            "Interpreter failed: {:?}",
            interpreter_result.err()
        );
        match interpreter_result {
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        };

        let bytes: std::cell::Ref<'_, Vec<u8>> = output.borrow();
        let result = match str::from_utf8(&bytes) {
            Ok(v) => v.to_owned(),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        assert_eq!(result, "");
    }

    #[test]
    fn test_string_functions() {
        let source = r#"
        "#;

        let tokens = tokenize(source).expect("Tokenization failed");
        let mut stmts = parse(tokens).expect("Parsing failed");

        let output: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(Vec::new()));
        let output_for_interp = output.clone();

        let mut interpreter = Interpreter::new_with_output(output_for_interp);
        let mut resolver = Resolver::new(&mut interpreter);
        resolver.resolve(&mut stmts).expect("Resolving failed");

        let interpreter_result = interpreter.interprete(&mut stmts);
        assert!(
            interpreter_result.is_ok(),
            "Interpreter failed: {:?}",
            interpreter_result.err()
        );
        match interpreter_result {
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        };

        let bytes: std::cell::Ref<'_, Vec<u8>> = output.borrow();
        let result = match str::from_utf8(&bytes) {
            Ok(v) => v.to_owned(),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        assert_eq!(result, "");
    }

    #[test]
    fn test_system_functions() {
        let source = r#"
        "#;

        let tokens = tokenize(source).expect("Tokenization failed");
        let mut stmts = parse(tokens).expect("Parsing failed");

        let output: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(Vec::new()));
        let output_for_interp = output.clone();

        let mut interpreter = Interpreter::new_with_output(output_for_interp);
        let mut resolver = Resolver::new(&mut interpreter);
        resolver.resolve(&mut stmts).expect("Resolving failed");

        let interpreter_result = interpreter.interprete(&mut stmts);
        assert!(
            interpreter_result.is_ok(),
            "Interpreter failed: {:?}",
            interpreter_result.err()
        );
        match interpreter_result {
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        };

        let bytes: std::cell::Ref<'_, Vec<u8>> = output.borrow();
        let result = match str::from_utf8(&bytes) {
            Ok(v) => v.to_owned(),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        assert_eq!(result, "");
    }

    #[test]
    fn test_time_functions() {
        let source = r#"
        "#;

        let tokens = tokenize(source).expect("Tokenization failed");
        let mut stmts = parse(tokens).expect("Parsing failed");

        let output: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(Vec::new()));
        let output_for_interp = output.clone();

        let mut interpreter = Interpreter::new_with_output(output_for_interp);
        let mut resolver = Resolver::new(&mut interpreter);
        resolver.resolve(&mut stmts).expect("Resolving failed");

        let interpreter_result = interpreter.interprete(&mut stmts);
        assert!(
            interpreter_result.is_ok(),
            "Interpreter failed: {:?}",
            interpreter_result.err()
        );
        match interpreter_result {
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        };

        let bytes: std::cell::Ref<'_, Vec<u8>> = output.borrow();
        let result = match str::from_utf8(&bytes) {
            Ok(v) => v.to_owned(),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        assert_eq!(result, "");
    }
}

#[cfg(test)]
mod test {
    use crate::interpreter::Interpreter;
    use crate::parser::parse;
    use crate::resolver::Resolver;
    use crate::scanner::tokenize;

    use std::{cell::RefCell, collections::HashMap, rc::Rc, str};

    #[test]
    fn test_recursive_function_fibonacci() {
        let source = r#"
            fn fib(n) {
                if (n <= 1) return n;
                return fib(n - 2) + fib(n - 1);
            }

            for (var i = 0; i < 10; i = i + 1) {
                print fib(i);
            }
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

        assert_eq!(result, "0\n1\n1\n2\n3\n5\n8\n13\n21\n34\n");
    }

    #[test]
    fn test_for_loop() {
        let source = r#"
            for (var i = 0; i < 5; i = i + 1) {
                print i;
            }
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

        assert_eq!(result, "0\n1\n2\n3\n4\n");
    }

    #[test]
    fn test_variable_resolution() {
        let source = r#"
            var a = "global";
            {
              fn showA() {
                print a;
              }

              showA();
              var a = "block";
              showA();
            }
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

        assert_eq!(result, "global\nglobal\n");
    }

    #[test]
    fn test_closures() {
        let source = r#"
            fn makeCounter() {
              var i = 0;
              fn count() {
                i = i + 1;
                print i;
              }
              return count;
            }
            var counter = makeCounter();
            counter(); 
            counter();
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

        assert_eq!(result, "1\n2\n");
    }

    #[test]
    fn test_function_arguments() {
        let source = r#"
            fn print_var(some_variable) {
              print some_variable;
            }
            print_var("ARGUMENT"); 
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

        assert_eq!(result, "ARGUMENT\n");
    }

    #[test]
    fn test_break_continue() {
        let source = r#"
            var i = 0;
            while(true) {
              i = i + 1;
              
              if (i == 1) {
                continue;
              }
              if (i == 3) {
                continue;
              }
              if ( i > 5) {
                break;
              }

                print i;

            }

            print "broken";
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

        assert_eq!(result, "2\n4\n5\nbroken\n");
    }

    #[test]
    fn test_if_inside_for() {
        let source = r#"
        for(var i = 0; i < 5; i = i + 1) {
            if (i == 3) {
                print "PASSED";
            }

        }
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

        assert_eq!(result, "PASSED\n");
    }

    #[test]
    fn test_classes() {
        let source = r#"
        class Bagel {
          fn eat() {
            print "eating bagel";
          }

          fn eating_n_bagels(n) {
            print "eating " + n + " bagels!";
          }
        }
        var bagel = Bagel();
        print bagel; // prints "Bagel instance"
        print Bagel;
        bagel.eat(); // prints "eating bagel"
        bagel.eating_n_bagels(10);
        bagel.flavour = "everything";
        print bagel.flavour;
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

        assert_eq!(
            result,
            "Bagel instance\nBagel\neating bagel\neating 10 bagels!\neverything\n"
        );
    }
}

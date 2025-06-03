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
            var fname = "test_io_file.txt";
            var dname = "test_io_dir";
            print IO.write_file(fname, "abc");
            print IO.read_file(fname);
            print IO.append_file(fname, "def");
            print IO.read_file(fname);
            print IO.exists(fname);
            print IO.is_file(fname);
            print IO.is_dir(fname);
            print IO.mkdir(dname);
            print IO.is_dir(dname);
            print IO.list_dir(".");
            print IO.remove_file(fname);
            print IO.exists(fname);
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

        let mut lines = result.lines();
        assert_eq!(lines.next().unwrap().trim(), "Nil"); // write_file returns Nil
        assert_eq!(lines.next().unwrap().trim(), "abc"); // read_file
        assert_eq!(lines.next().unwrap().trim(), "Nil"); // append_file returns Nil
        assert_eq!(lines.next().unwrap().trim(), "abcdef"); // read_file after append
        assert_eq!(lines.next().unwrap().trim(), "true"); // exists
        assert_eq!(lines.next().unwrap().trim(), "true"); // is_file
        assert_eq!(lines.next().unwrap().trim(), "false"); // is_dir
        assert_eq!(lines.next().unwrap().trim(), "Nil"); // mkdir returns Nil
        assert_eq!(lines.next().unwrap().trim(), "true"); // is_dir for created dir
        let dir_listing = lines.next().unwrap().trim(); // list_dir
        assert!(dir_listing.contains("test_io_file.txt") || dir_listing.contains("test_io_dir"));
        assert_eq!(lines.next().unwrap().trim(), "Nil"); // remove_file returns Nil
        assert_eq!(lines.next().unwrap().trim(), "false"); // exists after remove
    }

    #[test]
    fn test_math_functions() {
        let source = r#"
            print Math.abs(-5);
            print Math.sqrt(9);
            print Math.pow(2, 3);
            print Math.exp(1);
            print Math.log(8, 2);
            print Math.log10(100);
            print Math.sin(0);
            print Math.cos(0);
            print Math.tan(0);
            print Math.asin(1);
            print Math.acos(1);
            print Math.atan(1);
            print Math.atan2(1, 1);
            print Math.floor(3.7);
            print Math.ceil(3.2);
            print Math.round(3.5);
            print Math.trunc(3.9);
            print Math.degrees(3.141592653589793);
            print Math.radians(180);
            print Math.min(2, -1);
            print Math.max(3, 1);
            print Math.clamp(5, 1, 10);
            print Math.clamp(-5, 1, 10);
            print Math.clamp(15, 1, 10);
            print Math.random(10); // can't assert value, just check it's a number
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

        let mut lines = result.lines();
        assert_eq!(lines.next().unwrap().trim(), "5"); // abs
        assert_eq!(lines.next().unwrap().trim(), "3"); // sqrt
        assert_eq!(lines.next().unwrap().trim(), "8"); // pow
        assert!(
            (lines.next().unwrap().trim().parse::<f64>().unwrap() - std::f64::consts::E).abs()
                < 1e-10
        ); // exp
        assert_eq!(lines.next().unwrap().trim(), "3"); // log base 2
        assert_eq!(lines.next().unwrap().trim(), "2"); // log10
        assert_eq!(lines.next().unwrap().trim(), "0"); // sin(0)
        assert_eq!(lines.next().unwrap().trim(), "1"); // cos(0)
        assert_eq!(lines.next().unwrap().trim(), "0"); // tan(0)
        assert!(
            (lines.next().unwrap().trim().parse::<f64>().unwrap() - std::f64::consts::FRAC_PI_2)
                .abs()
                < 1e-10
        ); // asin(1)
        assert_eq!(lines.next().unwrap().trim(), "0"); // acos(1)
        assert!(
            (lines.next().unwrap().trim().parse::<f64>().unwrap() - std::f64::consts::FRAC_PI_4)
                .abs()
                < 1e-10
        ); // atan(1)
        assert!(
            (lines.next().unwrap().trim().parse::<f64>().unwrap() - std::f64::consts::FRAC_PI_4)
                .abs()
                < 1e-10
        ); // atan2(1,1)
        assert_eq!(lines.next().unwrap().trim(), "3"); // floor(3.7)
        assert_eq!(lines.next().unwrap().trim(), "4"); // ceil(3.2)
        assert_eq!(lines.next().unwrap().trim(), "4"); // round(3.5)
        assert_eq!(lines.next().unwrap().trim(), "3"); // trunc(3.9)
        assert!((lines.next().unwrap().trim().parse::<f64>().unwrap() - 180.0).abs() < 1e-10); // degrees(pi)
        assert!(
            (lines.next().unwrap().trim().parse::<f64>().unwrap() - std::f64::consts::PI).abs()
                < 1e-10
        ); // radians(180)
        assert_eq!(lines.next().unwrap().trim(), "-1"); // min
        assert_eq!(lines.next().unwrap().trim(), "3"); // max
        assert_eq!(lines.next().unwrap().trim(), "5"); // clamp(5,1,10)
        assert_eq!(lines.next().unwrap().trim(), "1"); // clamp(-5,1,10)
        assert_eq!(lines.next().unwrap().trim(), "10"); // clamp(15,1,10)
                                                        // random(10) - just check it's a number in [0,10)
        let rand_val = lines.next().unwrap().trim().parse::<f64>().unwrap();
        assert!((0.0..10.0).contains(&rand_val));
    }

    #[test]
    #[ignore] // keeping this for now because this test is really flaky
    fn test_network_functions() {
        let source = r#"
            print String.contains(Network.http_get("https://httpbin.org/get"), "url");
            print String.contains(Network.http_post("https://httpbin.org/post", "foo=bar"), "foo");
            print Network.download_file("https://httpbin.org/robots.txt", "/tmp/test_download.txt");
            print IO.exists("/tmp/test_download.txt");
            print Network.ping("8.8.8.8");
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

        let mut lines = result.lines();
        assert_eq!(lines.next().unwrap().trim(), "true"); // http_get contains url
        assert_eq!(lines.next().unwrap().trim(), "true"); // http_post contains foo
        assert_eq!(lines.next().unwrap().trim(), "Nil"); // download_file returns Nil
        assert_eq!(lines.next().unwrap().trim(), "true"); // file exists
        let ping_result = lines.next().unwrap().trim();
        assert!(ping_result == "true" || ping_result == "false"); // ping may fail
    }

    #[test]
    fn test_string_functions() {
        let source = r#"
            print String.len("hello");
            print String.split("a,b,c", ",");
            print String.join("-", "a,b,c");
            print String.replace("hello world", "world", "rust");
            print String.lower("HeLLo");
            print String.upper("HeLLo");
            print String.strip("  hi  ");
            print String.startswith("foobar", "foo");
            print String.endswith("foobar", "bar");
            print String.find("hello world", "world");
            print String.find("hello world", "rust");
            print String.contains("hello world", "world");
            print String.contains("hello world", "rust");
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

        let mut lines = result.lines();
        assert_eq!(lines.next().unwrap().trim(), "5"); // len
        assert_eq!(lines.next().unwrap().trim(), "a,b,c"); // split
        assert_eq!(lines.next().unwrap().trim(), "a-b-c"); // join
        assert_eq!(lines.next().unwrap().trim(), "hello rust"); // replace
        assert_eq!(lines.next().unwrap().trim(), "hello"); // lower
        assert_eq!(lines.next().unwrap().trim(), "HELLO"); // upper
        assert_eq!(lines.next().unwrap().trim(), "hi"); // strip
        assert_eq!(lines.next().unwrap().trim(), "true"); // startswith
        assert_eq!(lines.next().unwrap().trim(), "true"); // endswith
        assert_eq!(lines.next().unwrap().trim(), "6"); // find
        assert_eq!(lines.next().unwrap().trim(), "Nil"); // find not found
        assert_eq!(lines.next().unwrap().trim(), "true"); // contains
        assert_eq!(lines.next().unwrap().trim(), "false"); // contains not found
    }

    #[test]
    fn test_system_functions() {
        let source = r#"
            print System.platform();
            print System.env("PATH") != nil;
            print System.args() != nil;
            print System.exec("echo hi");
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

        let mut lines = result.lines();
        let platform = lines.next().unwrap().trim();
        assert!(platform == "linux" || platform == "windows" || platform == "macos");
        assert_eq!(lines.next().unwrap().trim(), "true"); // env PATH exists
        assert_eq!(lines.next().unwrap().trim(), "true"); // args returns string
        assert_eq!(lines.next().unwrap().trim(), "hi"); // exec echo hi
    }

    #[test]
    fn test_time_functions() {
        let source = r#"
            var t = Time.time();
            print t > 1748982291; // time taken when writing this text
            Time.sleep(0.1);
            print "slept";
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

        let mut lines = result.lines();
        assert_eq!(lines.next().unwrap().trim(), "true"); // t > 0
        assert_eq!(lines.next().unwrap().trim(), "slept");
    }
}

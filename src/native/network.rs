use crate::callable::Callable;
use crate::class::ClassObject;
use crate::interpreter::Interpreter;
use crate::object::Object;
use crate::runtime_error::RuntimeError;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct HttpGetFn;
#[derive(Debug, Clone)]
pub struct HttpPostFn;
#[derive(Debug, Clone)]
pub struct DownloadFileFn;
#[derive(Debug, Clone)]
pub struct PingFn;

impl Callable for HttpGetFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        if _arguments.len() != 1 {
            return Err(RuntimeError::argument_error(0, format!("Expected 1 argument but got {}", _arguments.len())));
        }
        let url = match &_arguments[0] {
            Object::String(s) => s,
            _ => return Err(RuntimeError::argument_error(0, "http_get(url): argument must be a string")),
        };
        match reqwest::blocking::get(url) {
            Ok(resp) => {
                match resp.text() {
                    Ok(text) => Ok(Object::String(text)),
                    Err(e) => Err(RuntimeError::other(0, format!("http_get: failed to read response text: {}", e))),
                }
            },
            Err(e) => Err(RuntimeError::other(0, format!("http_get: request failed: {}", e))),
        }
    }
    fn arity(&self) -> usize {
        1
    }
}
impl Display for HttpGetFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn http_get>")
    }
}

impl Callable for HttpPostFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        if _arguments.len() != 2 {
            return Err(RuntimeError::argument_error(0, format!("Expected 2 arguments but got {}", _arguments.len())));
        }
        let url = match &_arguments[0] {
            Object::String(s) => s,
            _ => return Err(RuntimeError::argument_error(0, "http_post(url, body): first argument must be a string")),
        };
        let body = match &_arguments[1] {
            Object::String(s) => s,
            _ => return Err(RuntimeError::argument_error(0, "http_post(url, body): second argument must be a string")),
        };
        let client = reqwest::blocking::Client::new();
        match client.post(url).body(body.clone()).send() {
            Ok(resp) => {
                match resp.text() {
                    Ok(text) => Ok(Object::String(text)),
                    Err(e) => Err(RuntimeError::other(0, format!("http_post: failed to read response text: {}", e))),
                }
            },
            Err(e) => Err(RuntimeError::other(0, format!("http_post: request failed: {}", e))),
        }
    }
    fn arity(&self) -> usize {
        2
    }
}
impl Display for HttpPostFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn http_post>")
    }
}

impl Callable for DownloadFileFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        if _arguments.len() != 2 {
            return Err(RuntimeError::argument_error(0, format!("Expected 2 arguments but got {}", _arguments.len())));
        }
        let url = match &_arguments[0] {
            Object::String(s) => s,
            _ => return Err(RuntimeError::argument_error(0, "download_file(url, path): first argument must be a string")),
        };
        let path = match &_arguments[1] {
            Object::String(s) => s,
            _ => return Err(RuntimeError::argument_error(0, "download_file(url, path): second argument must be a string")),
        };
        match reqwest::blocking::get(url) {
            Ok(mut resp) => {
                let mut file = match std::fs::File::create(path) {
                    Ok(f) => f,
                    Err(e) => return Err(RuntimeError::other(0, format!("download_file: failed to create file: {}", e))),
                };
                match std::io::copy(&mut resp, &mut file) {
                    Ok(_) => Ok(Object::Nil),
                    Err(e) => Err(RuntimeError::other(0, format!("download_file: failed to write file: {}", e))),
                }
            },
            Err(e) => Err(RuntimeError::other(0, format!("download_file: request failed: {}", e))),
        }
    }
    fn arity(&self) -> usize {
        2
    }
}
impl Display for DownloadFileFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn download_file>")
    }
}

impl Callable for PingFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        if _arguments.len() != 1 {
            return Err(RuntimeError::argument_error(0, format!("Expected 1 argument but got {}", _arguments.len())));
        }
        let host = match &_arguments[0] {
            Object::String(s) => s,
            _ => return Err(RuntimeError::argument_error(0, "ping(host): argument must be a string")),
        };
        let output = std::process::Command::new("ping")
            .arg("-c").arg("1")
            .arg(host)
            .output();
        match output {
            Ok(out) => Ok(Object::Boolean(out.status.success())),
            Err(_) => Ok(Object::Boolean(false)),
        }
    }
    fn arity(&self) -> usize {
        1
    }
}
impl Display for PingFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn ping>")
    }
}

pub fn create_class() -> ClassObject {
    let methods = HashMap::new();
    let mut static_methods = HashMap::new();
    static_methods.insert(
        "http_get".to_string(),
        Rc::new(RefCell::new(Box::new(HttpGetFn) as Box<dyn Callable>)),
    );
    static_methods.insert(
        "http_post".to_string(),
        Rc::new(RefCell::new(Box::new(HttpPostFn) as Box<dyn Callable>)),
    );
    static_methods.insert(
        "download_file".to_string(),
        Rc::new(RefCell::new(Box::new(DownloadFileFn) as Box<dyn Callable>)),
    );
    static_methods.insert(
        "ping".to_string(),
        Rc::new(RefCell::new(Box::new(PingFn) as Box<dyn Callable>)),
    );
    ClassObject {
        name: "Network".to_string(),
        superclass: None,
        methods,
        static_methods,
    }
}


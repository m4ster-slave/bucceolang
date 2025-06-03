use crate::callable::Callable;
use crate::interpreter::Interpreter;
use crate::object::Object;
use crate::runtime_error::RuntimeError;
use crate::class::ClassObject;
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct AbsFn;
#[derive(Debug, Clone)]
pub struct SqrtFn;
#[derive(Debug, Clone)]
pub struct PowFn;
#[derive(Debug, Clone)]
pub struct ExpFn;
#[derive(Debug, Clone)]
pub struct LogFn;
#[derive(Debug, Clone)]
pub struct Log10Fn;
#[derive(Debug, Clone)]
pub struct SinFn;
#[derive(Debug, Clone)]
pub struct CosFn;
#[derive(Debug, Clone)]
pub struct TanFn;
#[derive(Debug, Clone)]
pub struct AsinFn;
#[derive(Debug, Clone)]
pub struct AcosFn;
#[derive(Debug, Clone)]
pub struct AtanFn;
#[derive(Debug, Clone)]
pub struct Atan2Fn;
#[derive(Debug, Clone)]
pub struct FloorFn;
#[derive(Debug, Clone)]
pub struct CeilFn;
#[derive(Debug, Clone)]
pub struct RoundFn;
#[derive(Debug, Clone)]
pub struct TruncFn;
#[derive(Debug, Clone)]
pub struct DegreesFn;
#[derive(Debug, Clone)]
pub struct RadiansFn;
#[derive(Debug, Clone)]
pub struct MinFn;
#[derive(Debug, Clone)]
pub struct MaxFn;
#[derive(Debug, Clone)]
pub struct ClampFn;

impl Callable for AbsFn {
    fn call(&self, _interpreter: &mut Interpreter, _arguments: Vec<Object>) -> Result<Object, RuntimeError> {
        unimplemented!("AbsFn native logic not implemented yet")
    }
    fn arity(&self) -> usize { 1 }
}
impl Display for AbsFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn abs>")
    }
}

impl Callable for SqrtFn {
    fn call(&self, _interpreter: &mut Interpreter, _arguments: Vec<Object>) -> Result<Object, RuntimeError> {
        unimplemented!("SqrtFn native logic not implemented yet")
    }
    fn arity(&self) -> usize { 1 }
}
impl Display for SqrtFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn sqrt>")
    }
}

impl Callable for PowFn {
    fn call(&self, _interpreter: &mut Interpreter, _arguments: Vec<Object>) -> Result<Object, RuntimeError> {
        unimplemented!("PowFn native logic not implemented yet")
    }
    fn arity(&self) -> usize { 2 }
}
impl Display for PowFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn pow>")
    }
}

impl Callable for ExpFn {
    fn call(&self, _interpreter: &mut Interpreter, _arguments: Vec<Object>) -> Result<Object, RuntimeError> {
        unimplemented!("ExpFn native logic not implemented yet")
    }
    fn arity(&self) -> usize { 1 }
}
impl Display for ExpFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn exp>")
    }
}

impl Callable for LogFn {
    fn call(&self, _interpreter: &mut Interpreter, _arguments: Vec<Object>) -> Result<Object, RuntimeError> {
        unimplemented!("LogFn native logic not implemented yet")
    }
    fn arity(&self) -> usize { 2 }
}
impl Display for LogFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn log>")
    }
}

impl Callable for Log10Fn {
    fn call(&self, _interpreter: &mut Interpreter, _arguments: Vec<Object>) -> Result<Object, RuntimeError> {
        unimplemented!("Log10Fn native logic not implemented yet")
    }
    fn arity(&self) -> usize { 1 }
}
impl Display for Log10Fn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn log10>")
    }
}

impl Callable for SinFn {
    fn call(&self, _interpreter: &mut Interpreter, _arguments: Vec<Object>) -> Result<Object, RuntimeError> {
        unimplemented!("SinFn native logic not implemented yet")
    }
    fn arity(&self) -> usize { 1 }
}
impl Display for SinFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn sin>")
    }
}

impl Callable for CosFn {
    fn call(&self, _interpreter: &mut Interpreter, _arguments: Vec<Object>) -> Result<Object, RuntimeError> {
        unimplemented!("CosFn native logic not implemented yet")
    }
    fn arity(&self) -> usize { 1 }
}
impl Display for CosFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn cos>")
    }
}

impl Callable for TanFn {
    fn call(&self, _interpreter: &mut Interpreter, _arguments: Vec<Object>) -> Result<Object, RuntimeError> {
        unimplemented!("TanFn native logic not implemented yet")
    }
    fn arity(&self) -> usize { 1 }
}
impl Display for TanFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn tan>")
    }
}

impl Callable for AsinFn {
    fn call(&self, _interpreter: &mut Interpreter, _arguments: Vec<Object>) -> Result<Object, RuntimeError> {
        unimplemented!("AsinFn native logic not implemented yet")
    }
    fn arity(&self) -> usize { 1 }
}
impl Display for AsinFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn asin>")
    }
}

impl Callable for AcosFn {
    fn call(&self, _interpreter: &mut Interpreter, _arguments: Vec<Object>) -> Result<Object, RuntimeError> {
        unimplemented!("AcosFn native logic not implemented yet")
    }
    fn arity(&self) -> usize { 1 }
}
impl Display for AcosFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn acos>")
    }
}

impl Callable for AtanFn {
    fn call(&self, _interpreter: &mut Interpreter, _arguments: Vec<Object>) -> Result<Object, RuntimeError> {
        unimplemented!("AtanFn native logic not implemented yet")
    }
    fn arity(&self) -> usize { 1 }
}
impl Display for AtanFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn atan>")
    }
}

impl Callable for Atan2Fn {
    fn call(&self, _interpreter: &mut Interpreter, _arguments: Vec<Object>) -> Result<Object, RuntimeError> {
        unimplemented!("Atan2Fn native logic not implemented yet")
    }
    fn arity(&self) -> usize { 2 }
}
impl Display for Atan2Fn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn atan2>")
    }
}

impl Callable for FloorFn {
    fn call(&self, _interpreter: &mut Interpreter, _arguments: Vec<Object>) -> Result<Object, RuntimeError> {
        unimplemented!("FloorFn native logic not implemented yet")
    }
    fn arity(&self) -> usize { 1 }
}
impl Display for FloorFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn floor>")
    }
}

impl Callable for CeilFn {
    fn call(&self, _interpreter: &mut Interpreter, _arguments: Vec<Object>) -> Result<Object, RuntimeError> {
        unimplemented!("CeilFn native logic not implemented yet")
    }
    fn arity(&self) -> usize { 1 }
}
impl Display for CeilFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn ceil>")
    }
}

impl Callable for RoundFn {
    fn call(&self, _interpreter: &mut Interpreter, _arguments: Vec<Object>) -> Result<Object, RuntimeError> {
        unimplemented!("RoundFn native logic not implemented yet")
    }
    fn arity(&self) -> usize { 1 }
}
impl Display for RoundFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn round>")
    }
}

impl Callable for TruncFn {
    fn call(&self, _interpreter: &mut Interpreter, _arguments: Vec<Object>) -> Result<Object, RuntimeError> {
        unimplemented!("TruncFn native logic not implemented yet")
    }
    fn arity(&self) -> usize { 1 }
}
impl Display for TruncFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn trunc>")
    }
}

impl Callable for DegreesFn {
    fn call(&self, _interpreter: &mut Interpreter, _arguments: Vec<Object>) -> Result<Object, RuntimeError> {
        unimplemented!("DegreesFn native logic not implemented yet")
    }
    fn arity(&self) -> usize { 1 }
}
impl Display for DegreesFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn degrees>")
    }
}

impl Callable for RadiansFn {
    fn call(&self, _interpreter: &mut Interpreter, _arguments: Vec<Object>) -> Result<Object, RuntimeError> {
        unimplemented!("RadiansFn native logic not implemented yet")
    }
    fn arity(&self) -> usize { 1 }
}
impl Display for RadiansFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn radians>")
    }
}

impl Callable for MinFn {
    fn call(&self, _interpreter: &mut Interpreter, _arguments: Vec<Object>) -> Result<Object, RuntimeError> {
        unimplemented!("MinFn native logic not implemented yet")
    }
    fn arity(&self) -> usize { 1 }
}
impl Display for MinFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn min>")
    }
}

impl Callable for MaxFn {
    fn call(&self, _interpreter: &mut Interpreter, _arguments: Vec<Object>) -> Result<Object, RuntimeError> {
        unimplemented!("MaxFn native logic not implemented yet")
    }
    fn arity(&self) -> usize { 1 }
}
impl Display for MaxFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn max>")
    }
}

impl Callable for ClampFn {
    fn call(&self, _interpreter: &mut Interpreter, _arguments: Vec<Object>) -> Result<Object, RuntimeError> {
        unimplemented!("ClampFn native logic not implemented yet")
    }
    fn arity(&self) -> usize { 3 }
}
impl Display for ClampFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn clamp>")
    }
}

pub fn create_class() -> ClassObject {
    let methods = HashMap::new();
    let mut static_methods = HashMap::new();
    static_methods.insert("abs".to_string(), Rc::new(RefCell::new(Box::new(AbsFn) as Box<dyn Callable>)));
    static_methods.insert("sqrt".to_string(), Rc::new(RefCell::new(Box::new(SqrtFn) as Box<dyn Callable>)));
    static_methods.insert("pow".to_string(), Rc::new(RefCell::new(Box::new(PowFn) as Box<dyn Callable>)));
    static_methods.insert("exp".to_string(), Rc::new(RefCell::new(Box::new(ExpFn) as Box<dyn Callable>)));
    static_methods.insert("log".to_string(), Rc::new(RefCell::new(Box::new(LogFn) as Box<dyn Callable>)));
    static_methods.insert("log10".to_string(), Rc::new(RefCell::new(Box::new(Log10Fn) as Box<dyn Callable>)));
    static_methods.insert("sin".to_string(), Rc::new(RefCell::new(Box::new(SinFn) as Box<dyn Callable>)));
    static_methods.insert("cos".to_string(), Rc::new(RefCell::new(Box::new(CosFn) as Box<dyn Callable>)));
    static_methods.insert("tan".to_string(), Rc::new(RefCell::new(Box::new(TanFn) as Box<dyn Callable>)));
    static_methods.insert("asin".to_string(), Rc::new(RefCell::new(Box::new(AsinFn) as Box<dyn Callable>)));
    static_methods.insert("acos".to_string(), Rc::new(RefCell::new(Box::new(AcosFn) as Box<dyn Callable>)));
    static_methods.insert("atan".to_string(), Rc::new(RefCell::new(Box::new(AtanFn) as Box<dyn Callable>)));
    static_methods.insert("atan2".to_string(), Rc::new(RefCell::new(Box::new(Atan2Fn) as Box<dyn Callable>)));
    static_methods.insert("floor".to_string(), Rc::new(RefCell::new(Box::new(FloorFn) as Box<dyn Callable>)));
    static_methods.insert("ceil".to_string(), Rc::new(RefCell::new(Box::new(CeilFn) as Box<dyn Callable>)));
    static_methods.insert("round".to_string(), Rc::new(RefCell::new(Box::new(RoundFn) as Box<dyn Callable>)));
    static_methods.insert("trunc".to_string(), Rc::new(RefCell::new(Box::new(TruncFn) as Box<dyn Callable>)));
    static_methods.insert("degrees".to_string(), Rc::new(RefCell::new(Box::new(DegreesFn) as Box<dyn Callable>)));
    static_methods.insert("radians".to_string(), Rc::new(RefCell::new(Box::new(RadiansFn) as Box<dyn Callable>)));
    static_methods.insert("min".to_string(), Rc::new(RefCell::new(Box::new(MinFn) as Box<dyn Callable>)));
    static_methods.insert("max".to_string(), Rc::new(RefCell::new(Box::new(MaxFn) as Box<dyn Callable>)));
    static_methods.insert("clamp".to_string(), Rc::new(RefCell::new(Box::new(ClampFn) as Box<dyn Callable>)));
    ClassObject {
        name: "Math".to_string(),
        superclass: None,
        methods,
        static_methods,
    }
}



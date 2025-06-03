use crate::callable::Callable;
use crate::class::ClassObject;
use crate::interpreter::Interpreter;
use crate::object::Object;
use crate::runtime_error::RuntimeError;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

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
#[derive(Debug, Clone)]
pub struct RandomFn;

impl Callable for AbsFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        let arg = _arguments.last().ok_or_else(|| {
            RuntimeError::other(
                0,
                "not enough arguments in function".to_string() + &self.to_string(),
            )
        })?;
        if let Object::Number(num) = arg {
            Ok(Object::Number(num.abs()))
        } else {
            Err(RuntimeError::other(
                0,
                "argument must be a number".to_string(),
            ))
        }
    }
    fn arity(&self) -> usize {
        1
    }
}
impl Display for AbsFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn abs>")
    }
}

impl Callable for SqrtFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        let arg = _arguments.last().ok_or_else(|| {
            RuntimeError::other(
                0,
                "not enough arguments in function".to_string() + &self.to_string(),
            )
        })?;

        if let Object::Number(num) = arg {
            Ok(Object::Number(num.sqrt()))
        } else {
            Err(RuntimeError::other(
                0,
                "argument must be a number".to_string(),
            ))
        }
    }
    fn arity(&self) -> usize {
        1
    }
}
impl Display for SqrtFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn sqrt>")
    }
}

impl Callable for PowFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        if _arguments.len() < 2 {
            return Err(RuntimeError::other(
                0,
                "not enough arguments in function".to_string() + &self.to_string(),
            ));
        }
        let base = &_arguments[_arguments.len() - 2];
        let exp = &_arguments[_arguments.len() - 1];
        if let (Object::Number(b), Object::Number(e)) = (base, exp) {
            Ok(Object::Number(b.powf(*e)))
        } else {
            Err(RuntimeError::other(
                0,
                "arguments must be numbers".to_string(),
            ))
        }
    }
    fn arity(&self) -> usize {
        2
    }
}
impl Display for PowFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn pow>")
    }
}

impl Callable for ExpFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        let arg = _arguments.last().ok_or_else(|| {
            RuntimeError::other(
                0,
                "not enough arguments in function".to_string() + &self.to_string(),
            )
        })?;
        if let Object::Number(num) = arg {
            Ok(Object::Number(num.exp()))
        } else {
            Err(RuntimeError::other(
                0,
                "argument must be a number".to_string(),
            ))
        }
    }
    fn arity(&self) -> usize {
        1
    }
}
impl Display for ExpFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn exp>")
    }
}

impl Callable for LogFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        if _arguments.len() < 2 {
            return Err(RuntimeError::other(
                0,
                "not enough arguments in function".to_string() + &self.to_string(),
            ));
        }
        let value = &_arguments[_arguments.len() - 2];
        let base = &_arguments[_arguments.len() - 1];
        if let (Object::Number(v), Object::Number(b)) = (value, base) {
            if *v <= 0.0 || *b <= 0.0 {
                return Err(RuntimeError::other(
                    0,
                    "logarithm arguments must be positive".to_string(),
                ));
            }
            Ok(Object::Number(v.log(*b)))
        } else {
            Err(RuntimeError::other(
                0,
                "arguments must be numbers".to_string(),
            ))
        }
    }
    fn arity(&self) -> usize {
        2
    }
}
impl Display for LogFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn log>")
    }
}

impl Callable for Log10Fn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        let arg = _arguments.last().ok_or_else(|| {
            RuntimeError::other(
                0,
                "not enough arguments in function".to_string() + &self.to_string(),
            )
        })?;
        if let Object::Number(num) = arg {
            if *num <= 0.0 {
                return Err(RuntimeError::other(
                    0,
                    "log10 argument must be positive".to_string(),
                ));
            }
            Ok(Object::Number(num.log10()))
        } else {
            Err(RuntimeError::other(
                0,
                "argument must be a number".to_string(),
            ))
        }
    }
    fn arity(&self) -> usize {
        1
    }
}
impl Display for Log10Fn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn log10>")
    }
}

impl Callable for SinFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        let arg = _arguments.last().ok_or_else(|| {
            RuntimeError::other(
                0,
                "not enough arguments in function".to_string() + &self.to_string(),
            )
        })?;

        if let Object::Number(num) = arg {
            // sin is rad
            Ok(Object::Number(num.sin()))
        } else {
            Err(RuntimeError::other(
                0,
                "argument must be a number".to_string(),
            ))
        }
    }
    fn arity(&self) -> usize {
        1
    }
}
impl Display for SinFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn sin>")
    }
}

impl Callable for CosFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        let arg = _arguments.last().ok_or_else(|| {
            RuntimeError::other(
                0,
                "not enough arguments in function".to_string() + &self.to_string(),
            )
        })?;
        if let Object::Number(num) = arg {
            Ok(Object::Number(num.cos()))
        } else {
            Err(RuntimeError::other(
                0,
                "argument must be a number".to_string(),
            ))
        }
    }
    fn arity(&self) -> usize {
        1
    }
}
impl Display for CosFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn cos>")
    }
}

impl Callable for TanFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        let arg = _arguments.last().ok_or_else(|| {
            RuntimeError::other(
                0,
                "not enough arguments in function".to_string() + &self.to_string(),
            )
        })?;
        if let Object::Number(num) = arg {
            Ok(Object::Number(num.tan()))
        } else {
            Err(RuntimeError::other(
                0,
                "argument must be a number".to_string(),
            ))
        }
    }
    fn arity(&self) -> usize {
        1
    }
}
impl Display for TanFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn tan>")
    }
}

impl Callable for AsinFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        let arg = _arguments.last().ok_or_else(|| {
            RuntimeError::other(
                0,
                "not enough arguments in function".to_string() + &self.to_string(),
            )
        })?;
        if let Object::Number(num) = arg {
            Ok(Object::Number(num.asin()))
        } else {
            Err(RuntimeError::other(
                0,
                "argument must be a number".to_string(),
            ))
        }
    }
    fn arity(&self) -> usize {
        1
    }
}
impl Display for AsinFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn asin>")
    }
}

impl Callable for AcosFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        let arg = _arguments.last().ok_or_else(|| {
            RuntimeError::other(
                0,
                "not enough arguments in function".to_string() + &self.to_string(),
            )
        })?;
        if let Object::Number(num) = arg {
            Ok(Object::Number(num.acos()))
        } else {
            Err(RuntimeError::other(
                0,
                "argument must be a number".to_string(),
            ))
        }
    }
    fn arity(&self) -> usize {
        1
    }
}
impl Display for AcosFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn acos>")
    }
}

impl Callable for AtanFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        let arg = _arguments.last().ok_or_else(|| {
            RuntimeError::other(
                0,
                "not enough arguments in function".to_string() + &self.to_string(),
            )
        })?;
        if let Object::Number(num) = arg {
            Ok(Object::Number(num.atan()))
        } else {
            Err(RuntimeError::other(
                0,
                "argument must be a number".to_string(),
            ))
        }
    }
    fn arity(&self) -> usize {
        1
    }
}
impl Display for AtanFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn atan>")
    }
}

impl Callable for Atan2Fn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        if _arguments.len() < 2 {
            return Err(RuntimeError::other(
                0,
                "not enough arguments in function".to_string() + &self.to_string(),
            ));
        }
        let y = &_arguments[_arguments.len() - 2];
        let x = &_arguments[_arguments.len() - 1];
        if let (Object::Number(y), Object::Number(x)) = (y, x) {
            Ok(Object::Number(y.atan2(*x)))
        } else {
            Err(RuntimeError::other(
                0,
                "arguments must be numbers".to_string(),
            ))
        }
    }
    fn arity(&self) -> usize {
        2
    }
}
impl Display for Atan2Fn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn atan2>")
    }
}

impl Callable for FloorFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        let arg = _arguments.last().ok_or_else(|| {
            RuntimeError::other(
                0,
                "not enough arguments in function".to_string() + &self.to_string(),
            )
        })?;
        if let Object::Number(num) = arg {
            Ok(Object::Number(num.floor()))
        } else {
            Err(RuntimeError::other(
                0,
                "argument must be a number".to_string(),
            ))
        }
    }
    fn arity(&self) -> usize {
        1
    }
}
impl Display for FloorFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn floor>")
    }
}

impl Callable for CeilFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        let arg = _arguments.last().ok_or_else(|| {
            RuntimeError::other(
                0,
                "not enough arguments in function".to_string() + &self.to_string(),
            )
        })?;
        if let Object::Number(num) = arg {
            Ok(Object::Number(num.ceil()))
        } else {
            Err(RuntimeError::other(
                0,
                "argument must be a number".to_string(),
            ))
        }
    }
    fn arity(&self) -> usize {
        1
    }
}
impl Display for CeilFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn ceil>")
    }
}

impl Callable for RoundFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        let arg = _arguments.last().ok_or_else(|| {
            RuntimeError::other(
                0,
                "not enough arguments in function".to_string() + &self.to_string(),
            )
        })?;
        if let Object::Number(num) = arg {
            Ok(Object::Number(num.round()))
        } else {
            Err(RuntimeError::other(
                0,
                "argument must be a number".to_string(),
            ))
        }
    }
    fn arity(&self) -> usize {
        1
    }
}
impl Display for RoundFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn round>")
    }
}

impl Callable for TruncFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        let arg = _arguments.last().ok_or_else(|| {
            RuntimeError::other(
                0,
                "not enough arguments in function".to_string() + &self.to_string(),
            )
        })?;
        if let Object::Number(num) = arg {
            Ok(Object::Number(num.trunc()))
        } else {
            Err(RuntimeError::other(
                0,
                "argument must be a number".to_string(),
            ))
        }
    }
    fn arity(&self) -> usize {
        1
    }
}
impl Display for TruncFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn trunc>")
    }
}

impl Callable for DegreesFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        let arg = _arguments.last().ok_or_else(|| {
            RuntimeError::other(
                0,
                "not enough arguments in function".to_string() + &self.to_string(),
            )
        })?;
        if let Object::Number(num) = arg {
            Ok(Object::Number(num.to_degrees()))
        } else {
            Err(RuntimeError::other(
                0,
                "argument must be a number".to_string(),
            ))
        }
    }
    fn arity(&self) -> usize {
        1
    }
}
impl Display for DegreesFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn degrees>")
    }
}

impl Callable for RadiansFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        let arg = _arguments.last().ok_or_else(|| {
            RuntimeError::other(
                0,
                "not enough arguments in function".to_string() + &self.to_string(),
            )
        })?;
        if let Object::Number(num) = arg {
            Ok(Object::Number(num.to_radians()))
        } else {
            Err(RuntimeError::other(
                0,
                "argument must be a number".to_string(),
            ))
        }
    }
    fn arity(&self) -> usize {
        1
    }
}
impl Display for RadiansFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn radians>")
    }
}

impl Callable for MinFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        let mut min: Option<f64> = None;
        for arg in &_arguments {
            if let Object::Number(num) = arg {
                min = Some(match min {
                    Some(m) => m.min(*num),
                    None => *num,
                });
            } else {
                return Err(RuntimeError::other(
                    0,
                    "all arguments must be numbers".to_string(),
                ));
            }
        }
        min.map(Object::Number)
            .ok_or_else(|| RuntimeError::other(0, "no arguments provided".to_string()))
    }
    fn arity(&self) -> usize {
        2
    }
}
impl Display for MinFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn min>")
    }
}

impl Callable for MaxFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        let mut max: Option<f64> = None;
        for arg in &_arguments {
            if let Object::Number(num) = arg {
                max = Some(match max {
                    Some(m) => m.max(*num),
                    None => *num,
                });
            } else {
                return Err(RuntimeError::other(
                    0,
                    "all arguments must be numbers".to_string(),
                ));
            }
        }
        max.map(Object::Number)
            .ok_or_else(|| RuntimeError::other(0, "no arguments provided".to_string()))
    }
    fn arity(&self) -> usize {
        2
    }
}
impl Display for MaxFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn max>")
    }
}

impl Callable for ClampFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        if _arguments.len() < 3 {
            return Err(RuntimeError::other(
                0,
                "not enough arguments in function".to_string() + &self.to_string(),
            ));
        }
        let value = &_arguments[_arguments.len() - 3];
        let min = &_arguments[_arguments.len() - 2];
        let max = &_arguments[_arguments.len() - 1];
        if let (Object::Number(v), Object::Number(minv), Object::Number(maxv)) = (value, min, max) {
            Ok(Object::Number(v.max(*minv).min(*maxv)))
        } else {
            Err(RuntimeError::other(
                0,
                "arguments must be numbers".to_string(),
            ))
        }
    }
    fn arity(&self) -> usize {
        3
    }
}
impl Display for ClampFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn clamp>")
    }
}

impl Callable for RandomFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        let range_obj = _arguments.last().ok_or_else(|| {
            RuntimeError::other(
                0,
                "not enough arguments in function".to_string() + &self.to_string(),
            )
        })?;

        if let Object::Number(max) = range_obj {
            if *max <= 0.0 {
                return Err(RuntimeError::other(0, "range must be positive".to_string()));
            }
            let rand_value = Self::lcg_next();
            let result = (rand_value as f64 % max).floor();
            Ok(Object::Number(result))
        } else {
            Err(RuntimeError::other(
                0,
                "argument must be a number".to_string(),
            ))
        }
    }
    fn arity(&self) -> usize {
        1
    }
}
impl Display for RandomFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn random>")
    }
}
thread_local! {
    static RNG_STATE: RefCell<u64> = RefCell::new(seed_from_time());
}

fn seed_from_time() -> u64 {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    (nanos & 0x7fffffff) as u64
}
impl RandomFn {
    fn lcg_next() -> u64 {
        RNG_STATE.with(|state| {
            let mut s = state.borrow_mut();
            // constants for LCG: same as glibc
            *s = s.wrapping_mul(1103515245).wrapping_add(12345) & 0x7fffffff;
            *s
        })
    }
}

pub fn create_class() -> ClassObject {
    let methods = HashMap::new();
    let mut static_methods = HashMap::new();
    static_methods.insert(
        "abs".to_string(),
        Rc::new(RefCell::new(Box::new(AbsFn) as Box<dyn Callable>)),
    );
    static_methods.insert(
        "sqrt".to_string(),
        Rc::new(RefCell::new(Box::new(SqrtFn) as Box<dyn Callable>)),
    );
    static_methods.insert(
        "pow".to_string(),
        Rc::new(RefCell::new(Box::new(PowFn) as Box<dyn Callable>)),
    );
    static_methods.insert(
        "exp".to_string(),
        Rc::new(RefCell::new(Box::new(ExpFn) as Box<dyn Callable>)),
    );
    static_methods.insert(
        "log".to_string(),
        Rc::new(RefCell::new(Box::new(LogFn) as Box<dyn Callable>)),
    );
    static_methods.insert(
        "log10".to_string(),
        Rc::new(RefCell::new(Box::new(Log10Fn) as Box<dyn Callable>)),
    );
    static_methods.insert(
        "sin".to_string(),
        Rc::new(RefCell::new(Box::new(SinFn) as Box<dyn Callable>)),
    );
    static_methods.insert(
        "cos".to_string(),
        Rc::new(RefCell::new(Box::new(CosFn) as Box<dyn Callable>)),
    );
    static_methods.insert(
        "tan".to_string(),
        Rc::new(RefCell::new(Box::new(TanFn) as Box<dyn Callable>)),
    );
    static_methods.insert(
        "asin".to_string(),
        Rc::new(RefCell::new(Box::new(AsinFn) as Box<dyn Callable>)),
    );
    static_methods.insert(
        "acos".to_string(),
        Rc::new(RefCell::new(Box::new(AcosFn) as Box<dyn Callable>)),
    );
    static_methods.insert(
        "atan".to_string(),
        Rc::new(RefCell::new(Box::new(AtanFn) as Box<dyn Callable>)),
    );
    static_methods.insert(
        "atan2".to_string(),
        Rc::new(RefCell::new(Box::new(Atan2Fn) as Box<dyn Callable>)),
    );
    static_methods.insert(
        "floor".to_string(),
        Rc::new(RefCell::new(Box::new(FloorFn) as Box<dyn Callable>)),
    );
    static_methods.insert(
        "ceil".to_string(),
        Rc::new(RefCell::new(Box::new(CeilFn) as Box<dyn Callable>)),
    );
    static_methods.insert(
        "round".to_string(),
        Rc::new(RefCell::new(Box::new(RoundFn) as Box<dyn Callable>)),
    );
    static_methods.insert(
        "trunc".to_string(),
        Rc::new(RefCell::new(Box::new(TruncFn) as Box<dyn Callable>)),
    );
    static_methods.insert(
        "degrees".to_string(),
        Rc::new(RefCell::new(Box::new(DegreesFn) as Box<dyn Callable>)),
    );
    static_methods.insert(
        "radians".to_string(),
        Rc::new(RefCell::new(Box::new(RadiansFn) as Box<dyn Callable>)),
    );
    static_methods.insert(
        "min".to_string(),
        Rc::new(RefCell::new(Box::new(MinFn) as Box<dyn Callable>)),
    );
    static_methods.insert(
        "max".to_string(),
        Rc::new(RefCell::new(Box::new(MaxFn) as Box<dyn Callable>)),
    );
    static_methods.insert(
        "clamp".to_string(),
        Rc::new(RefCell::new(Box::new(ClampFn) as Box<dyn Callable>)),
    );
    static_methods.insert(
        "random".to_string(),
        Rc::new(RefCell::new(Box::new(RandomFn) as Box<dyn Callable>)),
    );

    ClassObject {
        name: "Math".to_string(),
        superclass: None,
        methods,
        static_methods,
    }
}

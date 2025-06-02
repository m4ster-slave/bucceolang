use crate::{
    callable::Callable, interpreter::Interpreter, object::Object, runtime_error::RuntimeError,
};
use std::fmt::Display;
use std::time::{SystemTime, UNIX_EPOCH};

use std::cell::RefCell;
use std::io;
use std::io::prelude::*;

/// Returns the current time in seconds since the Unix epoch as a floating point number.
///
/// # Usage
/// ```lox
/// var t = clock(); // t will be a number representing seconds since 1970-01-01 00:00:00 UTC
/// ```
///
/// # Returns
/// - `Number`: The current time in seconds (f64).
#[derive(Clone, Debug)]
pub struct ClockFn;

impl Callable for ClockFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        Ok(Object::Number(now))
    }

    fn arity(&self) -> usize {
        0
    }
}

impl Display for ClockFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn clock>")
    }
}

/// Reads a single line from standard input and returns it as a string (with whitespace trimmed).
///
/// # Usage
/// ```lox
/// var input = read(); // Waits for user input, then assigns the trimmed string to `input`
/// ```
///
/// # Returns
/// - `String`: The line read from stdin, trimmed of whitespace.
#[derive(Clone, Debug)]
pub struct ReadFn;
impl Callable for ReadFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        let mut s = String::new();
        match io::stdin().lock().read_line(&mut s) {
            Ok(_) => Ok(Object::String(s.trim().into())),
            Err(e) => Err(RuntimeError::other(
                0,
                format!("error when reading from stdin: {e}"),
            )),
        }
    }

    fn arity(&self) -> usize {
        0
    }
}

impl Display for ReadFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn read>")
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

/// Returns a pseudo-random integer in the range `[0, max)`.
///
/// Uses a simple linear congruential generator (LCG) seeded from the current system time.
/// The result is always less than the provided `max` value.
///
/// # Usage
/// ```lox
/// var r = random(10); // r will be an integer between 0 and 9
/// ```
///
/// # Arguments
/// - `max` (`Number`): The exclusive upper bound for the random number. Must be positive.
///
/// # Returns
/// - `Number`: A random integer in `[0, max)`.
///
/// # Errors
/// - If `max` is not a positive number, or not a number, a runtime error is returned.
#[derive(Clone, Debug)]
pub struct RandomFn;
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
impl Callable for RandomFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        mut arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        let range_obj = arguments.pop().ok_or_else(|| {
            RuntimeError::other(
                0,
                "not enough arguments in function".to_string() + &self.to_string(),
            )
        })?;

        if let Object::Number(max) = range_obj {
            if max <= 0.0 {
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

/// Returns the sine of a number (in radians).
///
/// # Usage
/// ```bucceolang
/// var s = sin(3.1415); // s will be approximately 0.0
/// ```
///
/// # Arguments
/// - `num` (`Number`): The value (in radians) whose sine is to be calculated.
///
/// # Returns
/// - `Number`: The sine of the input value.
///
/// # Errors
/// - If the argument is not a number, a runtime error is returned.
#[derive(Clone, Debug)]
pub struct SinFn;
impl Callable for SinFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        mut arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        let arg = arguments.pop().ok_or_else(|| {
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

/// Returns the square root of a number.
///
/// # Usage
/// ```bucceolang
/// var r = sqrt(9); // r will be 3.0
/// ```
///
/// # Arguments
/// - `num` (`Number`): The value whose square root is to be calculated.
///
/// # Returns
/// - `Number`: The square root of the input value.
///
/// # Errors
/// - If the argument is not a number, a runtime error is returned.
#[derive(Clone, Debug)]
pub struct SqrtFn;
impl Callable for SqrtFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        mut arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        let arg = arguments.pop().ok_or_else(|| {
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

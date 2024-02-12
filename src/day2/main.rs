use std::fmt::Display;

/// An operation to perform on two subexpressions.
#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

/// An expression, in tree form.
#[derive(Debug)]
enum Expression {
    /// An operation on two subexpressions.
    Op { op: Operation, left: Box<Expression>, right: Box<Expression> },

    /// A literal value
    Value(i64),
}

fn eval(e: Expression) -> Result<i64, String> {
    match e {
        Expression::Value(v) => Ok(v),
        Expression::Op { op, left, right } => {
            match op {
                Operation::Add => Ok(eval(*left)? + eval(*right)?),
                Operation::Sub => Ok(eval(*left)? - eval(*right)?),
                Operation::Mul => Ok(eval(*left)? * eval(*right)?),
                Operation::Div => {
                    let right = eval(*right)?;
                    if right == 0 {
                        Err(String::from("division by zero"))
                    } else {
                        Ok(eval(*left)? / right)
                    }
                }
            }
        }
    }
}

#[test]
fn test_value() {
    assert_eq!(eval(Expression::Value(19)), Ok(19));
}

#[test]
fn test_sum() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(Expression::Value(10)),
            right: Box::new(Expression::Value(20)),
        }),
        Ok(30)
    );
}

#[test]
fn test_recursion() {
    let term1 = Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Value(10)),
        right: Box::new(Expression::Value(9)),
    };
    let term2 = Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Op {
            op: Operation::Sub,
            left: Box::new(Expression::Value(3)),
            right: Box::new(Expression::Value(4)),
        }),
        right: Box::new(Expression::Value(5)),
    };
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(term1),
            right: Box::new(term2),
        }),
        Ok(85)
    );
}

#[test]
fn test_error() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Div,
            left: Box::new(Expression::Value(99)),
            right: Box::new(Expression::Value(0)),
        }),
        Err(String::from("division by zero"))
    );
}

pub trait Logger {
    /// Log a message at the given verbosity level.
    fn log(&self, verbosity: u8, message: impl Display);
}

struct StderrLogger;
struct VerbosityFilter {
    max_verbosity: u8,
    inner: StderrLogger,
}

impl Logger for VerbosityFilter {
    fn log(&self, verbosity: u8, message: impl Display) {
        if self.max_verbosity >= verbosity {
            self.inner.log(verbosity, message);
        }
    }
}

impl Logger for StderrLogger {
    fn log(&self, verbosity: u8, message: impl Display) {
        eprintln!("verbosity={verbosity}: {message}");
    }
}

fn do_things(logger: &impl Logger) {
    logger.log(5, "FYI");
    logger.log(2, "Uhoh");
}
trait LessThan {
    /// Return true if self is less than other.
    fn less_than(&self, other: &Self) -> bool;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Citation {
    author: &'static str,
    year: u32,
}

impl LessThan for Citation {
    fn less_than(&self, other: &Self) -> bool {
        if self.author < other.author {
            true
        } else if self.author > other.author {
            false
        } else {
            self.year < other.year
        }
    }
}

fn min<T: LessThan>(a: T, b: T) -> T {
    if a.less_than(&b) {
        a
    } else {
        b
    }
}

fn main() {
    println!("{}", eval(Expression::Op { op: (Operation::Add), left: (Box::new(Expression::Value(11))), right: (Box::new(Expression::Value(11))) }).unwrap());
    println!("{}", eval(Expression::Op { op: (Operation::Sub), left: (Box::new(Expression::Value(11))), right: (Box::new(Expression::Value(11))) }).unwrap());
    println!("{}", eval(Expression::Op { op: (Operation::Mul), left: (Box::new(Expression::Value(11))), right: (Box::new(Expression::Value(11))) }).unwrap());
    println!("{}", eval(Expression::Op { op: (Operation::Div), left: (Box::new(Expression::Value(11))), right: (Box::new(Expression::Value(11))) }).unwrap());

    // TODO: Define and implement `VerbosityFilter`.
    // Methods and Traits
    let l = VerbosityFilter { max_verbosity: 3, inner: StderrLogger };
    do_things(&l);

    // Generic
    // TODO: implement the `min` function used in `main`.
    let cit1 = Citation { author: "Shapiro", year: 2011 };
    let cit2 = Citation { author: "Baumann", year: 2010 };
    let cit3 = Citation { author: "Baumann", year: 2019 };
    debug_assert_eq!(min(cit1, cit2), cit2);
    debug_assert_eq!(min(cit2, cit3), cit2);
    debug_assert_eq!(min(cit1, cit3), cit3);
}

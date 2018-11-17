use std::fmt;

use error::Result;
use value::Scalar;
use value::Value;

use super::Context;
use variable::Variable;

/// An un-evaluated `Value`.
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    /// Un-evaluated.
    Variable(Variable),
    /// Evaluated.
    Literal(Value),
}

impl Expression {
    /// Create an expression from a scalar literal.
    pub fn with_literal<S: Into<Scalar>>(literal: S) -> Self {
        Expression::Literal(Value::scalar(literal))
    }

    /// Convert into a literal if possible.
    pub fn into_literal(self) -> Option<Value> {
        match self {
            Expression::Literal(x) => Some(x),
            Expression::Variable(_) => None,
        }
    }

    /// Convert into a variable, if possible.
    pub fn into_variable(self) -> Option<Variable> {
        match self {
            Expression::Literal(_) => None,
            Expression::Variable(x) => Some(x),
        }
    }

    /// Convert to a `Value`.
    pub fn evaluate(&self, context: &Context) -> Result<Value> {
        let val = match *self {
            Expression::Literal(ref x) => x.clone(),
            Expression::Variable(ref x) => {
                let path = x.evaluate(context)?;
                context.stack().get(&path)?.clone()
            }
        };
        Ok(val)
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expression::Literal(ref x) => write!(f, "{}", x),
            Expression::Variable(ref x) => write!(f, "{}", x),
        }
    }
}
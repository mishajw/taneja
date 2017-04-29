use std::fmt::Display;
use std::fmt;
use number::Number;
use std::boxed::Box;

#[derive(Clone)]
pub enum Expression<T> {
    Value(T),
    Add(Box<Expression<T>>, Box<Expression<T>>),
    Subtract(Box<Expression<T>>, Box<Expression<T>>),
    Multiply(Box<Expression<T>>, Box<Expression<T>>),
    Divide(Box<Expression<T>>, Box<Expression<T>>),
    Power(Box<Expression<T>>, Box<Expression<T>>),
    Concat(Box<Expression<T>>, Box<Expression<T>>),
}

impl<T: Number + Clone> Expression<T> {
    pub fn evaluate(&self) -> Option<T> {
        match self {
            &Expression::Value(ref a) =>
                Some(a.clone()),
            &Expression::Add(ref a, ref b) =>
                match (a.evaluate(), b.evaluate()) {
                    (Some(a), Some(b)) => a.add(&b),
                    _ => None,
                },
            &Expression::Subtract(ref a, ref b) =>
                match (a.evaluate(), b.evaluate()) {
                    (Some(a), Some(b)) => a.subtract(&b),
                    _ => None,
                },
            &Expression::Multiply(ref a, ref b) =>
                match (a.evaluate(), b.evaluate()) {
                    (Some(a), Some(b)) => a.multiply(&b),
                    _ => None,
                },
            &Expression::Divide(ref a, ref b) =>
                match (a.evaluate(), b.evaluate()) {
                    (Some(a), Some(b)) => a.divide(&b),
                    _ => None,
                },
            &Expression::Power(ref a, ref b) =>
                match (a.evaluate(), b.evaluate()) {
                    (Some(a), Some(b)) => a.power(&b),
                    _ => None,
                },
            &Expression::Concat(ref a, ref b) =>
                match (a.evaluate(), b.evaluate()) {
                    (Some(a), Some(b)) => a.concat(&b),
                    _ => None,
                },
        }
    }
}

impl<T: Display> Display for Expression<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Expression::Value(ref a) => write!(f, "{}", a),
            &Expression::Add(ref a, ref b) => write!(f, "({} + {})", a, b),
            &Expression::Subtract(ref a, ref b) => write!(f, "({} - {})", a, b),
            &Expression::Multiply(ref a, ref b) => write!(f, "({} * {})", a, b),
            &Expression::Divide(ref a, ref b) => write!(f, "({} / {})", a, b),
            &Expression::Power(ref a, ref b) => write!(f, "({} ^ {})", a, b),
            &Expression::Concat(ref a, ref b) => write!(f, "({} | {})", a, b),
        }
    }
}


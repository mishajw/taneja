extern crate num;

use std::boxed::Box;
use num::{CheckedAdd, CheckedMul};

#[derive(Debug)]
enum Expression<T> {
    Value(T),
    Add(Box<Expression<T>>, Box<Expression<T>>),
    Multiply(Box<Expression<T>>, Box<Expression<T>>),
}

impl<T: CheckedAdd + CheckedMul + Clone> Expression<T> {
    fn evaluate(&self) -> Option<T> {
        match self {
            &Expression::Value(ref a) =>
                Some(a.clone()),
            &Expression::Add(ref a, ref b) =>
                match (a.evaluate(), b.evaluate()) {
                    (Some(a), Some(b)) => a.checked_add(&b),
                    _ => None,
                },
            &Expression::Multiply(ref a, ref b) =>
                match (a.evaluate(), b.evaluate()) {
                    (Some(a), Some(b)) => a.checked_mul(&b),
                    _ => None,
                },
        }
    }
}

fn main() {
    let a = Expression::Value(1);
    let b = Expression::Value(2);
    let c = Expression::Value(3);
    let add = Expression::Add(Box::new(a), Box::new(b));
    let multiply = Expression::Multiply(Box::new(add), Box::new(c));
    let final_expression = multiply;

    println!("final_expression: {:?}", final_expression);
    println!("final_expression.evaluate(): {:?}", final_expression.evaluate());
}


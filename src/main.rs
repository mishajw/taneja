extern crate num;

use std::boxed::Box;
use num::CheckedAdd;

#[derive(Debug)]
enum Expression<T> {
    Value(T),
    Add(Box<Expression<T>>, Box<Expression<T>>),
}

impl<T: CheckedAdd + Clone> Expression<T> {
    fn evaluate(&self) -> Option<T> {
        match self {
            &Expression::Value(ref a) =>
                Some(a.clone()),
            &Expression::Add(ref a, ref b) =>
                match (a.evaluate(), b.evaluate()) {
                    (Some(a), Some(b)) => a.checked_add(&b),
                    _ => None,
                }
        }
    }
}

fn main() {
    let a = Expression::Value(1);
    let b = Expression::Value(2);
    let c = Expression::Add(Box::new(a), Box::new(b));

    println!("c: {:?}", c);
    println!("c.evaluate(): {:?}", c.evaluate());
}


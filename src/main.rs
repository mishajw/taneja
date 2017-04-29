mod number;

use number::Number;
use std::boxed::Box;
use std::fmt::Debug;
use std::fmt;

#[derive(Clone)]
enum Expression<T> {
    Value(T),
    Add(Box<Expression<T>>, Box<Expression<T>>),
    Subtract(Box<Expression<T>>, Box<Expression<T>>),
    Multiply(Box<Expression<T>>, Box<Expression<T>>),
    Divide(Box<Expression<T>>, Box<Expression<T>>),
}

impl<T: Number + Clone> Expression<T> {
    fn evaluate(&self) -> Option<T> {
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
        }
    }
}

impl<T: Debug> Debug for Expression<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Expression::Value(ref a) => write!(f, "{:?}", a),
            &Expression::Add(ref a, ref b) => write!(f, "({:?} + {:?})", a, b),
            &Expression::Subtract(ref a, ref b) => write!(f, "({:?} - {:?})", a, b),
            &Expression::Multiply(ref a, ref b) => write!(f, "({:?} * {:?})", a, b),
            &Expression::Divide(ref a, ref b) => write!(f, "({:?} / {:?})", a, b),
        }
    }
}


fn get_initial_expression_list(length: usize) -> Vec<Expression<f64>> {
    (1..length + 1)
        .map(|x| x as f64)
        .map(Expression::Value)
        .collect::<Vec<Expression<f64>>>()
}

fn get_possible_combinations<T: Clone>(a: &Expression<T>, b: &Expression<T>) -> Vec<Expression<T>> {
    vec![
        Expression::Add(Box::new(a.clone()), Box::new(b.clone())),
        Expression::Subtract(Box::new(a.clone()), Box::new(b.clone())),
        Expression::Multiply(Box::new(a.clone()), Box::new(b.clone())),
        Expression::Divide(Box::new(a.clone()), Box::new(b.clone()))]
}

fn create_tree<T: Clone>(es: Vec<Expression<T>>) -> Vec<Expression<T>> {
    if es.len() < 2 {
        return es
    }

    let mut ess: Vec<Expression<T>> = Vec::new();

    for i in 0..es.len() - 1 {
        let a = &es[i];
        let b = &es[i + 1];
        
        let combinations = get_possible_combinations(a, b);

        for c in combinations {
            let mut new_es = es[0..i].to_vec();
            new_es.push(c);
            new_es.extend(es[i + 2..es.len()].to_vec());

            ess.extend(create_tree(new_es))
        }
    }

    return ess
}

fn main() {
    let es = get_initial_expression_list(3);
    
    for e in create_tree(es) {
        println!("{:?} = {:?}", e, e.evaluate());
    }
}


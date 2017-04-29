mod expression;
mod number;

use expression::Expression;
use number::Number;
use std::env;
use std::collections::BTreeMap;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <amount of numbers>", args[0]);
        return
    }

    match args[1].parse::<usize>() {
        Ok(length) => run_with_length(length),
        Err(err) => println!("Couldn't parse {} as integer: {}", args[1], err),
    }
}

fn run_with_length(length: usize) {
    let initial_expressions = get_initial_expression_list(length);

    let mut evaluations: BTreeMap<i32, Expression<f64>> = BTreeMap::new();

    {
        // Must be scoped so the mutable reference to `expressions` dies before we want to use the list again
        let callback = &mut |e: Expression<f64>| {
            if let Some(evaluation) = e.evaluate() {
                if evaluation.is_integer() {
                    evaluations.insert(evaluation as i32, e.clone());
                }
            }
        };

        make_possible_expressions(initial_expressions, callback);
    }

    println!("Expressions:");
    for (evaluation, expression) in evaluations {
        println!("{} = {}", expression, evaluation)
    }
}

fn make_possible_expressions<T: Clone>(mut es: Vec<Expression<T>>, callback: &mut FnMut(Expression<T>)) {
    if es.len() < 2 {
        callback(es.swap_remove(0));
        return
    }

    for i in 0..es.len() - 1 {
        let a = &es[i];
        let b = &es[i + 1];
        
        let combinations = get_possible_combinations(a, b);

        for c in combinations {
            let mut new_es = es[0..i].to_vec();
            new_es.push(c);
            new_es.extend(es[i + 2..es.len()].to_vec());

            make_possible_expressions(new_es, callback)
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
        Expression::Divide(Box::new(a.clone()), Box::new(b.clone())),
        Expression::Power(Box::new(a.clone()), Box::new(b.clone())),
        Expression::Concat(Box::new(a.clone()), Box::new(b.clone()))]
}


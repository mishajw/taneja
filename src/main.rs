mod expression;
mod number;
mod possible_expressions;

use expression::Expression;
use number::Number;
use possible_expressions::make_possible_expressions;
use std::collections::BTreeMap;
use std::env;

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

fn get_initial_expression_list(length: usize) -> Vec<Expression<f64>> {
    (1..length + 1)
        .map(|x| x as f64)
        .map(Expression::Value)
        .collect::<Vec<Expression<f64>>>()
}


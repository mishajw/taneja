mod expression;
mod number;

use expression::Expression;
use number::Number;

fn main() {
    let initial_expressions = get_initial_expression_list(3);

    let mut expressions: Vec<Expression<f64>> = Vec::new();

    {
        // Must be scoped so the mutable reference to `expressions` dies before we want to use the list again
        let callback = &mut |e| {
            expressions.push(e)
        };

        make_possible_expressions(initial_expressions, callback);
    }

    for e in expressions {
        println!("{}", e)
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
        Expression::Divide(Box::new(a.clone()), Box::new(b.clone()))]
}


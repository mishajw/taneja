mod expression;
mod number;

use expression::Expression;
use number::Number;

fn main() {
    let es = get_initial_expression_list(3);
    
    for e in create_tree(es) {
        match e.evaluate() {
            Some(evaluation) if evaluation.is_integer() =>
                println!("{:?} = {:?}", e, evaluation),
            _ => {}
        }
    }
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


use expression::Expression;

pub fn make_possible_expressions<T: Clone>(mut es: Vec<Expression<T>>, callback: &mut FnMut(Expression<T>)) {
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

fn get_possible_combinations<T: Clone>(a: &Expression<T>, b: &Expression<T>) -> Vec<Expression<T>> {
    let mut combinations = vec![
        Expression::Add(Box::new(a.clone()), Box::new(b.clone())),
        Expression::Subtract(Box::new(a.clone()), Box::new(b.clone())),
        Expression::Multiply(Box::new(a.clone()), Box::new(b.clone())),
        Expression::Divide(Box::new(a.clone()), Box::new(b.clone())),
        Expression::Power(Box::new(a.clone()), Box::new(b.clone()))];

    match (a, b) {
        (&Expression::Value(_), &Expression::Value(_)) =>
            combinations.push(Expression::Concat(Box::new(a.clone()), Box::new(b.clone()))),
        _ => {}
    }

    return combinations
}


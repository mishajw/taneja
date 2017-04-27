use std::boxed::Box;

#[derive(Debug)]
enum Expression<T> {
    Value(T),
    Add(Box<Expression<T>>, Box<Expression<T>>),
}

fn main() {
    let a = Expression::Value(1);
    let b = Expression::Value(2);
    let c = Expression::Add(Box::new(a), Box::new(b));

    println!("c: {:?}", c);
}

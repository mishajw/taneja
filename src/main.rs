#[derive(Debug)]
enum Expression<'a, T: 'a> {
    Value(T),
    Add(&'a Expression<'a, T>, &'a Expression<'a, T>),
}

fn main() {
    let a = Expression::Value(1);
    let b = Expression::Value(2);
    let c = Expression::Add(&a, &b);

    println!("c: {:?}", c);
}

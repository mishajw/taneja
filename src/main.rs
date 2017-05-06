mod expression;
mod number;
mod possible_expressions;

use expression::Expression;
use number::Number;
use possible_expressions::make_possible_expressions;
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::io::BufWriter;
use std::io::Error;
use std::io::Write;

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
    let mut found = 0i64;

    {
        // Must be scoped so the mutable reference to `expressions` dies before we want to use the list again
        let callback = &mut |e: Expression<f64>| {
            if let Some(evaluation) = e.evaluate() {
                if evaluation.is_integer() {
                    evaluations.insert(evaluation as i32, e.clone());
                }

                if found % 100000 == 0 {
                    println!("Found {} expressions", found)
                }

                found += 1
            }
        };

        println!("Starting to make expressions...");
        make_possible_expressions(initial_expressions, callback);
    }

    println!("Done. Found {} expressions", &found);

    if let Err(err) = write_evaluations(&evaluations) {
        println!("Error when creating evaluations file: {}", err);
    }

    print_streak_information(&evaluations)
}

fn get_initial_expression_list(length: usize) -> Vec<Expression<f64>> {
    (1..length + 1)
        .map(|x| x as f64)
        .map(Expression::Value)
        .collect::<Vec<Expression<f64>>>()
}

fn write_evaluations(evaluations: &BTreeMap<i32, Expression<f64>>) -> Result<(), Error> {
    fs::create_dir_all("output")?;
    let file = fs::File::create("output/evaluations.txt")?;
    let mut writer = BufWriter::new(file);

    for (evaluation, expression) in evaluations {
        writeln!(&mut writer, "{} = {}", expression, evaluation)?;
    }

    Ok(())
}

fn print_streak_information(evaluations: &BTreeMap<i32, Expression<f64>>) {
    let mut longest_streak = 0i32;
    let mut longest_streak_start: Option<&i32> = None;
    let mut longest_streak_end: Option<&i32> = None;
    let mut streak_beginning_opt: Option<&i32> = None;
    let mut previous_opt: Option<&i32> = None;

    for evaluation in evaluations.keys() {
        match previous_opt {
            Some(previous) => {
                let difference = evaluation - previous;

                if difference == 1 {
                    if let Some(streak_beginning) = streak_beginning_opt {
                        let streak_length = evaluation - streak_beginning;

                        if longest_streak < streak_length {
                            longest_streak = streak_length;
                            longest_streak_start = Some(streak_beginning);
                            longest_streak_end = Some(previous)
                        }
                    } else {
                        streak_beginning_opt = Some(previous)
                    }
                } else if difference == 2 {
                    println!("Missing {}", previous + 1);
                    streak_beginning_opt = None
                } else if difference > 2 {
                    println!("Missing {} to {}", previous + 1, evaluation - 1);
                    streak_beginning_opt = None
                }
            }
            None => {}
        }

        previous_opt = Some(evaluation)
    }

    match (longest_streak_start, longest_streak_end) {
        (Some(start), Some(end)) =>
            println!("Longest streak from {} to {} with length of {}", start, end + 1, longest_streak),
        _ => {}
    }
}


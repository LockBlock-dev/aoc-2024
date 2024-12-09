use itertools::Itertools;
use std::iter::{repeat, zip};

fn evaluate_with_operators(integers: &[&str], operators: &[&str], test_value: i64) -> Option<i64> {
    let operator_combinations = repeat(operators)
        .take(integers.len() - 1)
        .multi_cartesian_product();

    for operator_sequence in operator_combinations {
        let mut result: i64 = integers.first().unwrap().parse().unwrap();
        let mut second_result = result;

        for (&operator, &num) in zip(operator_sequence, &integers[1..]) {
            let num: i64 = num.parse().unwrap();

            match operator {
                "+" => {
                    result += num;
                    second_result += num;
                }
                "*" => {
                    result *= num;
                    second_result *= num;
                }
                "||" => {
                    second_result = (second_result.to_string() + &num.to_string())
                        .parse()
                        .unwrap();
                }
                _ => {}
            }
        }

        if result == test_value {
            return Some(result);
        }
        if second_result == test_value {
            return Some(second_result);
        }
    }

    return None;
}

fn main() {
    let mut total_calibration_result = 0;
    let mut total_second_calibration_result = 0;

    let operators = ["+", "*"];
    let operators_with_pipe = ["+", "*", "||"];

    for line in util::read_input_to_iter("./inputs/day7") {
        if let Some((test_value, integers_string)) = line.split(": ").collect_tuple() {
            let test_value: i64 = test_value.parse().unwrap();
            let integers: Vec<&str> = integers_string.split(" ").collect();

            if let Some(result) = evaluate_with_operators(&integers, &operators, test_value) {
                total_calibration_result += result;
            }

            if let Some(result) =
                evaluate_with_operators(&integers, &operators_with_pipe, test_value)
            {
                total_second_calibration_result += result;
            }
        }
    }

    println!("Day 7 part 1 solution: {}", total_calibration_result);
    println!("Day 7 part 2 solution: {}", total_second_calibration_result);
}

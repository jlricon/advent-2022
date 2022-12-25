use std::collections::HashMap;
#[derive(Debug)]
enum MonkeyType<'a> {
    Num(f64),
    Multiply(&'a str, &'a str),
    Divide(&'a str, &'a str),
    Sum(&'a str, &'a str),
    Substract(&'a str, &'a str),
    Root(&'a str, &'a str),
    Human,
}
#[derive(Debug)]
enum Operation {
    Multiply(Box<Operation>, Box<Operation>),
    Divide(Box<Operation>, Box<Operation>),
    Sum(Box<Operation>, Box<Operation>),
    Substract(Box<Operation>, Box<Operation>),
    Num(f64),
    HumanNum,
}
impl Operation {
    fn operate(&self, human_value: f64) -> f64 {
        // dbg!("Operating {}", self);
        match self {
            Operation::Num(num) => *num,
            Operation::Multiply(a, b) => a.operate(human_value) * b.operate(human_value),
            Operation::Divide(a, b) => a.operate(human_value) / b.operate(human_value),
            Operation::Sum(a, b) => a.operate(human_value) + b.operate(human_value),
            Operation::Substract(a, b) => a.operate(human_value) - b.operate(human_value),
            Operation::HumanNum => human_value,
        }
    }
}
fn monkey_to_op(m: &HashMap<String, MonkeyType>, monkey: &str) -> Operation {
    match m.get(monkey).unwrap() {
        MonkeyType::Num(num) => Operation::Num(*num),
        MonkeyType::Multiply(a, b) => {
            Operation::Multiply(Box::new(monkey_to_op(m, a)), Box::new(monkey_to_op(m, b)))
        }
        MonkeyType::Divide(a, b) => {
            Operation::Divide(Box::new(monkey_to_op(m, a)), Box::new(monkey_to_op(m, b)))
        }
        MonkeyType::Sum(a, b) => {
            Operation::Sum(Box::new(monkey_to_op(m, a)), Box::new(monkey_to_op(m, b)))
        }
        MonkeyType::Substract(a, b) => {
            Operation::Substract(Box::new(monkey_to_op(m, a)), Box::new(monkey_to_op(m, b)))
        }

        MonkeyType::Human => Operation::HumanNum,

        _ => panic!(),
    }
}
fn call_number(m: &HashMap<String, MonkeyType>, monkey: &str) -> f64 {
    match m.get(monkey).unwrap() {
        MonkeyType::Num(num) => *num,
        MonkeyType::Multiply(a, b) => call_number(m, a) * call_number(m, b),
        MonkeyType::Divide(a, b) => call_number(m, a) / call_number(m, b),
        MonkeyType::Sum(a, b) => call_number(m, a) + call_number(m, b),
        MonkeyType::Substract(a, b) => call_number(m, a) - call_number(m, b),
        _ => panic!("Unknown op"),
    }
}
// #[allow(dead_code)]
// fn part1() {
//     let input: HashMap<String, MonkeyType> = include_str!("../../data/day21.txt")
//         .lines()
//         .map(|l| {
//             let mut parts = l.split(": ");
//             let name = parts.next().unwrap().to_string();
//             let rest = parts.next().unwrap();
//             let monkey = match rest.parse::<i64>() {
//                 Ok(num) => MonkeyType::Num(num),
//                 Err(_) => {
//                     let ops = rest.split_whitespace().collect::<Vec<_>>();
//                     match ops[1] {
//                         "*" => MonkeyType::Multiply(ops[0], ops[2]),
//                         "-" => MonkeyType::Substract(ops[0], ops[2]),
//                         "/" => MonkeyType::Divide(ops[0], ops[2]),
//                         "+" => MonkeyType::Sum(ops[0], ops[2]),
//                         _ => panic!("Unknown op"),
//                     }
//                 }
//             };
//             (name, monkey)
//         })
//         .collect();
//     dbg!(call_number(&input, "root"));
// }
fn part2() {
    let input: HashMap<String, MonkeyType> = include_str!("../../data/day21.txt")
        .lines()
        .map(|l| {
            let mut parts = l.split(": ");
            let name = parts.next().unwrap().to_string();
            let rest = parts.next().unwrap();
            if name == "humn" {
                return (name, MonkeyType::Human);
            }
            let monkey = match rest.parse::<f64>() {
                Ok(num) => MonkeyType::Num(num),
                Err(_) => {
                    let ops = rest.split_whitespace().collect::<Vec<_>>();
                    if name == "root" {
                        return (name, MonkeyType::Root(ops[0], ops[2]));
                    }
                    match ops[1] {
                        "*" => MonkeyType::Multiply(ops[0], ops[2]),
                        "-" => MonkeyType::Substract(ops[0], ops[2]),
                        "/" => MonkeyType::Divide(ops[0], ops[2]),
                        "+" => MonkeyType::Sum(ops[0], ops[2]),
                        _ => panic!("Unknown op"),
                    }
                }
            };
            (name, monkey)
        })
        .collect();
    let (stack1, stack2) = match input.get("root").unwrap() {
        MonkeyType::Root(a, b) => (a, b),
        _ => panic!("Unknown op"),
    };
    let v1 = monkey_to_op(&input, stack1);
    let v2 = monkey_to_op(&input, stack2);

    let mut human_value: f64 = 0.0;
    // let mut human_value_prev: f64 = -1.0;
    // let mut human_value_next = 1;
    let eval = |human_value: f64| {
        let v1 = monkey_to_op(&input, stack1);
        let v2 = monkey_to_op(&input, stack2);
        (v1.operate(human_value) - v2.operate(human_value)) as f64
    };
    let mut nit = 10;
    while (eval(human_value).abs() > 0.01) && (nit != 0) {
        // Calculate gradient of the function at human_value
        let gradient = (eval(human_value + 1.0) - eval(human_value - 1.0)) / 2.0;

        dbg!(human_value, eval(human_value));
        // human_value_prev = human_value;
        human_value = human_value - eval(human_value) / gradient;
        nit -= 1;
    }
    dbg!(
        human_value,
        v1.operate(human_value),
        v2.operate(human_value)
    );
    // dbg!(v1.operate(human_value));
}
fn main() {
    part2();
}

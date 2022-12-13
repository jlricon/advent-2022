use itertools::Itertools;
use std::collections::VecDeque;
#[derive(Clone, Debug)]
enum Operation {
    Plus(usize),
    Multiply(usize),
    OldOld,
}
impl Operation {
    fn apply(&self, value: usize) -> usize {
        match self {
            Operation::Plus(x) => value + x,
            Operation::Multiply(x) => value * x,
            Operation::OldOld => value * value,
        }
    }
}
#[derive(Clone, Debug)]
struct Test {
    divisible: usize,
    true_monkey: usize,
    false_monkey: usize,
}
impl Test {
    fn apply_test(&self, value: usize) -> usize {
        // Returns true_monkey if value is divisible by divisible
        // Returns false_monkey otherwise
        if value % self.divisible == 0 {
            self.true_monkey as usize
        } else {
            self.false_monkey as usize
        }
    }
}
type MonkeyN = usize;
type ItemWorry = usize;
#[derive(Clone, Debug)]
struct Monkey {
    items: VecDeque<ItemWorry>,
    operation: Operation,
    test: Test,
    inspected: ItemWorry,
}
impl Monkey {
    fn inspect_items(&mut self) -> Vec<(MonkeyN, ItemWorry)> {
        let mut result = Vec::new();
        while let Some(item) = self.items.pop_front() {
            let new_item_value = self.operation.apply(item);
            let new_item_value2 = new_item_value / 3;

            let monkey = self.test.apply_test(new_item_value2);
            self.inspected += 1;
            result.push((monkey, new_item_value2));
        }
        result
    }
    fn inspect_items_nodivide(&mut self, common_divisor: usize) -> Vec<(MonkeyN, ItemWorry)> {
        let mut result = Vec::new();
        while let Some(item) = self.items.pop_front() {
            let new_item_value = self.operation.apply(item);

            let monkey = self.test.apply_test(new_item_value);
            let new_item_value = new_item_value % common_divisor;
            self.inspected += 1;
            result.push((monkey, new_item_value));
        }
        result
    }
}
fn part1(monkeys: Vec<Monkey>) {
    let mut monkeys = monkeys;
    for _ in 0..20 {
        for monkey in 0..monkeys.len() {
            let result = monkeys[monkey].inspect_items();
            println!("Monkey {}", monkey);

            // Assign the items to the monkeys
            result
                .into_iter()
                .for_each(|f| monkeys[f.0].items.push_back(f.1));
        }
    }
    let res: usize = monkeys
        .into_iter()
        .map(|m| m.inspected)
        .sorted()
        .rev()
        .take(2)
        .product();
    dbg!(res);
}
fn part2(monkeys: Vec<Monkey>) {
    let mut monkeys = monkeys;
    let common_divisor: usize = monkeys.iter().map(|m| m.test.divisible).product();
    for round in 0..10_000 {
        for monkey in 0..monkeys.len() {
            monkeys[monkey]
                .inspect_items_nodivide(common_divisor)
                .into_iter()
                .for_each(|f| monkeys[f.0].items.push_back(f.1));
        }
        if round % 10 == 0 {
            println!("Round {}", round);
        }
    }
    let res: usize = monkeys
        .into_iter()
        .map(|m| m.inspected)
        .sorted()
        .rev()
        .take(2)
        .product();
    dbg!(res);
}
fn main() {
    let input: Vec<Monkey> = include_str!("../../data/day11.txt")
        .split("\n\n")
        .map(|m| {
            let mut lines = m.split('\n');
            let _monkey = lines.next().unwrap();
            let number_string = lines.next().unwrap().split(": ").last().unwrap();
            let starting_numbers = if number_string.contains(',') {
                number_string
                    .split(", ")
                    .map(|m| m.parse::<usize>().unwrap())
                    .collect::<VecDeque<_>>()
            } else {
                vec![number_string.parse::<usize>().unwrap()].into()
            };
            let operation: Vec<&str> = lines
                .next()
                .unwrap()
                .split("new = old ")
                .last()
                .unwrap()
                .split(' ')
                .collect();

            let operation = match (operation[0], operation[1]) {
                ("+", x) => Operation::Plus(x.parse::<usize>().unwrap()),
                ("*", x) if x != "old" => Operation::Multiply(x.parse::<usize>().unwrap()),
                ("*", x) if x == "old" => Operation::OldOld,
                _ => panic!("Unknown operation"),
            };

            let test = lines
                .next()
                .unwrap()
                .split("by ")
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let monkey1 = lines
                .next()
                .unwrap()
                .split("monkey ")
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let monkey2 = lines
                .next()
                .unwrap()
                .split("monkey ")
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap();
            Monkey {
                items: starting_numbers,
                operation,
                test: Test {
                    divisible: test,
                    true_monkey: monkey1,
                    false_monkey: monkey2,
                },
                inspected: 0,
            }
        })
        .collect();

    part1(input.clone());

    part2(input);
}

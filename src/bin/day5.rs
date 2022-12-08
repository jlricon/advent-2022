type Stacks = Vec<Vec<char>>;
#[derive(Debug)]
struct Move {
    from: usize,
    to: usize,
    number: usize,
}
fn parse_box_stack(input: &str) -> Stacks {
    let mut lines = input.split("\n").collect::<Vec<&str>>();

    let n_stacks = lines
        .pop()
        .unwrap()
        .split("   ")
        .last()
        .unwrap()
        .trim_end()
        .chars()
        .nth(0)
        .unwrap()
        .to_digit(10)
        .unwrap() as usize;

    // The lengths of the lines equals n_stacks*3+(n_stacks-1)
    // Init the vector of vectors
    let mut res = Vec::new();
    for _ in 0..n_stacks {
        res.push(Vec::new());
    }
    lines.iter().for_each(|line| {
        (0..n_stacks).for_each(|pos| {
            let candidate = line.chars().skip(pos * 4 + 1).nth(0).unwrap();

            if candidate.is_ascii_uppercase() {
                res[pos].push(candidate);
            }
        });
    });
    res.iter_mut().for_each(|v| v.reverse());
    return res;
}
fn part1(inp: &Stacks, moves: &Vec<Move>) -> String {
    let mut stacks = inp.clone();
    moves.iter().for_each(|m| {
        for _ in (0..m.number) {
            let popped = stacks[m.from - 1].pop().unwrap();
            stacks[m.to - 1].push(popped);
        }
    });
    stacks.iter().map(|v| v.last().unwrap()).collect()
}
fn main() {
    let input = include_str!("../../data/day5.txt")
        .split("\n\n")
        .collect::<Vec<&str>>();
    let stacks = parse_box_stack(input[0]);
    let moves: Vec<Move> = input[1]
        .split("\n")
        .map(|line| {
            let part1 = line.split("move ").nth(1).unwrap();
            let part1 = part1.split(" from ").collect::<Vec<&str>>();
            let number = part1[0].parse::<usize>().unwrap();
            let part2 = part1[1].split(" to ").collect::<Vec<&str>>();
            let from = part2[0].parse::<usize>().unwrap();
            let to = part2[1].parse::<usize>().unwrap();
            Move { from, to, number }
        })
        .collect();

    dbg!(part1(&stacks, &moves));
    // dbg!(part2(input).unwrap());
}

fn get_calories_per_elf(input: &str) -> Vec<u32> {
    input
        .split("\n\n")
        .map(|elf_food| {
            elf_food
                .split("\n")
                .map(|line| line.parse::<u32>().unwrap())
                .sum()
        })
        .collect()
}
fn part1(input: &str) -> u32 {
    get_calories_per_elf(input).into_iter().max().unwrap()
}
fn part2(input: &str) -> u32 {
    let mut calories_elves = get_calories_per_elf(input);
    calories_elves.sort();
    calories_elves.reverse();
    calories_elves.iter().take(3).sum()
}
fn main() {
    let input = include_str!("../../data/day1.txt");
    dbg!(part1(input));
    dbg!(part2(input));
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test_data = include_str!("../../data/test1.txt");
        assert_eq!(part1(test_data), 24000);
    }
}

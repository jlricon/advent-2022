use std::num::ParseIntError;

fn get_calories_per_elf(input: &str) -> Result<Vec<u32>, ParseIntError> {
    input
        .split("\n\n")
        .map(|elf_food| elf_food.split("\n").map(|line| line.parse::<u32>()).sum())
        .collect()
}
fn part1(input: &str) -> Result<u32, ParseIntError> {
    Ok(get_calories_per_elf(input)?.into_iter().max().unwrap_or(0))
}
fn part2(input: &str) -> Result<u32, ParseIntError> {
    let mut calories_elves = get_calories_per_elf(input)?;
    calories_elves.sort();
    calories_elves.reverse();
    Ok(calories_elves.iter().take(3).sum())
}
fn main() {
    let input = include_str!("../../data/day1.txt");
    dbg!(part1(input).unwrap());
    dbg!(part2(input).unwrap());
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test_data = include_str!("../../data/test1.txt");
        assert_eq!(part1(test_data).unwrap(), 24000);
    }
}

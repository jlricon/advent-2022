use std::collections::HashSet;

use itertools::Itertools;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Item {
    char: char,
}
impl Item {
    fn priority(&self) -> u32 {
        // Lowercase item types a through z have priorities 1 through 26.
        // Uppercase item types A through Z have priorities 27 through 52.
        match self.char {
            'a'..='z' => self.char as u32 - 'a' as u32 + 1,
            'A'..='Z' => self.char as u32 - 'A' as u32 + 27,
            _ => panic!("Invalid item type"),
        }
    }
}
struct RuckSack {
    first_half: HashSet<Item>,
    second_half: HashSet<Item>,
}
impl RuckSack {
    fn as_single(&self) -> HashSet<&Item> {
        self.first_half.union(&self.second_half).collect()
    }
    fn find_common_element_priority_in_both_halves(&self) -> u32 {
        // Find the element that is present in first_half and second_half
        self.first_half
            .iter()
            .filter(|item| self.second_half.contains(item))
            .nth(0)
            .unwrap()
            .priority()
    }
    fn new(items: &str) -> RuckSack {
        let len = items.len();
        assert_eq!(len % 2, 0);
        let it = items.chars();
        let first_half = it.clone().take(len / 2).map(|c| Item { char: c }).collect();
        let second_half = it.skip(len / 2).map(|c| Item { char: c }).collect();
        RuckSack {
            first_half,
            second_half,
        }
    }
}
fn part1(input: &str) -> u32 {
    let rucksacks = input
        .split("\n")
        .map(|items| RuckSack::new(items))
        .collect::<Vec<RuckSack>>();
    rucksacks
        .iter()
        .map(|rucksack| rucksack.find_common_element_priority_in_both_halves())
        .sum()
}
fn part2(input: &str) -> u32 {
    let elf_groups = input
        .split("\n")
        .map(|items| RuckSack::new(items))
        .chunks(3)
        .into_iter()
        .map(|mut group| {
            let first_group = group.next().unwrap();
            let second_group = group.next().unwrap();
            let third_group = group.next().unwrap();
            first_group
                .as_single()
                .iter()
                .filter_map(|item| {
                    if second_group.as_single().contains(item)
                        && third_group.as_single().contains(item)
                    {
                        Some(item.priority())
                    } else {
                        None
                    }
                })
                .nth(0)
                .unwrap()
        })
        .sum();
    return elf_groups;
}
fn main() {
    let input = include_str!("../../data/day3.txt");
    dbg!(part1(input));
    dbg!(part2(input));
}

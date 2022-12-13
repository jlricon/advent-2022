use std::{cmp::Ordering, iter::Peekable};

use itertools::Itertools;
#[derive(Debug, PartialEq, Eq, Clone)]
enum Element {
    Num(usize),
    List(Vec<Element>),
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Element::Num(a), Element::Num(b)) => a.cmp(b),
            (Element::List(_), Element::Num(_)) => self.cmp(&Element::List(vec![other.clone()])),
            (Element::Num(_), Element::List(_)) => Element::List(vec![self.clone()]).cmp(other),
            (Element::List(a), Element::List(b)) => a.cmp(b),
        }
    }
}
impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
fn parse_element(s: &str) -> Element {
    parse_item(&mut s.chars().peekable())
}

fn parse_item<T: Iterator<Item = char>>(s: &mut Peekable<T>) -> Element {
    let c = s.next().unwrap();
    match c {
        '[' => parse_list(s),
        _ => {
            let mut item: String = c.into();
            while let Some(c_item) = s.next_if(|&c| c.is_ascii_digit()) {
                item.push(c_item);
            }
            Element::Num(item.parse::<usize>().unwrap())
        }
    }
}

fn parse_list<T: Iterator<Item = char>>(s: &mut Peekable<T>) -> Element {
    let mut list: Vec<Element> = Vec::new();
    loop {
        if let Some(c) = s.next_if(|&c| c == ',' || c == ']') {
            match c {
                ',' => {}
                ']' => break,
                _ => unreachable!(),
            }
        }
        list.push(parse_item(s));
    }
    Element::List(list)
}
// fn parse_element(list_str: &str) -> Element {
//     let mut elements = Vec::new();
//     let mut i = 1; // skip the opening [
//     while i < list_str.len() - 1 {
//         // stop at the closing ]
//         match list_str[i..].chars().next() {
//             Some(c) if c.is_digit(10) => {
//                 // parse number
//                 let end = list_str[i..]
//                     .find(|c: char| !c.is_digit(10))
//                     .unwrap_or(list_str.len() - i);
//                 let num_str = &list_str[i..i + end];
//                 let num = num_str.parse::<usize>().unwrap();
//                 elements.push(Element::Num(num));
//                 i += end;
//             }
//             Some('[') => {
//                 // parse nested list
//                 let end = list_str[i..].find(']').unwrap();
//                 let nested_list_str = &list_str[i..i + end + 1];
//                 elements.push(parse_element(nested_list_str));
//                 i += end + 1;
//             }
//             _ => {
//                 // skip any other characters (like commas)
//                 i += 1;
//             }
//         }
//     }
//     Element::List(elements)
// }

fn part1(inp: Vec<(Element, Element)>) {
    let ordered: usize = inp
        .iter()
        .map(|(a, b)| a.cmp(b))
        .enumerate()
        .filter(|v| v.1 == Ordering::Less)
        .map(|v| v.0 + 1)
        .sum();
    dbg!(ordered);
}
fn part2(inp: Vec<(Element, Element)>) {
    let dividers = vec![
        Element::List(vec![Element::List(vec![Element::Num(2)])]),
        Element::List(vec![Element::List(vec![Element::Num(6)])]),
    ];
    let all_elements: usize = inp
        .into_iter()
        .flat_map(|f| [f.0, f.1])
        .chain(dividers.clone())
        .sorted()
        .enumerate()
        .filter(|el| dividers.contains(&el.1))
        .map(|f| f.0 + 1)
        .product();
    dbg!(all_elements);
}
fn main() {
    let input: Vec<(Element, Element)> = include_str!("../../data/day13.txt")
        .split("\n\n")
        .map(|l| {
            let mut lines = l.split('\n');
            let first = parse_element(lines.next().unwrap());
            let second = parse_element(lines.next().unwrap());

            (first, second)
        })
        .collect();

    part1(input.clone());
    part2(input);
}
// Test with [[1],[2,3,4]]
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part3() {
        let test_data = "[93]";
        assert_eq!(
            parse_element(test_data),
            Element::List(vec![Element::Num(93)])
        );
    }
    #[test]
    fn test_part2() {
        let test_data = "[1,1,3,1,111]";
        assert_eq!(
            parse_element(test_data),
            Element::List(vec![
                Element::Num(1),
                Element::Num(1),
                Element::Num(3),
                Element::Num(1),
                Element::Num(111)
            ])
        );
    }
    #[test]
    fn test_part1() {
        let test_data = "[[1],[2,3,44]]";
        assert_eq!(
            parse_element(test_data),
            Element::List(vec![
                Element::List(vec![Element::Num(1)]),
                Element::List(vec![Element::Num(2), Element::Num(3), Element::Num(44)])
            ])
        );
    }
}

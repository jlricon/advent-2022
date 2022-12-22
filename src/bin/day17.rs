use std::collections::HashMap;

#[derive(Clone, Copy)]
enum JetDirection {
    Left,
    Right,
}
#[derive(PartialEq)]
enum MapItem {
    Rock,
    Floor,
}
impl JetDirection {
    fn from_char(c: char) -> Self {
        match c {
            '<' => Self::Left,
            '>' => Self::Right,
            _ => panic!(),
        }
    }
    fn get_direction(&self) -> (i64, i64) {
        match self {
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
        }
    }
}

struct RockCycle {
    rocks: Vec<Rock>,
    current: usize,
}

struct JetCycle {
    jets: Vec<JetDirection>,
    current: usize,
}
impl JetCycle {
    fn new(jets: Vec<JetDirection>) -> Self {
        Self { jets, current: 0 }
    }
    fn yield_next(&mut self) -> JetDirection {
        let jet = self.jets[self.current];
        self.current = (self.current + 1) % self.jets.len();
        jet
    }
}
impl RockCycle {
    fn new() -> Self {
        Self {
            rocks: vec![Rock::Bar, Rock::Cross, Rock::L, Rock::I, Rock::Square],
            current: 0,
        }
    }
    fn yield_next(&mut self) -> Rock {
        let rock = self.rocks[self.current];
        self.current = (self.current + 1) % self.rocks.len();
        rock
    }
}
#[derive(Copy, Clone, Debug, PartialEq)]
enum Rock {
    Bar,
    Cross,
    L,
    I,
    Square,
}
impl Rock {
    fn get_rock_points(&self) -> Vec<(i64, i64)> {
        match self {
            Self::I => vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            Self::Bar => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            Self::L => vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
            Self::Cross => vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
            Self::Square => vec![(0, 0), (0, 1), (1, 0), (1, 1)],
        }
    }
}
struct Map {
    map: HashMap<(i64, i64), MapItem>,
    height: i64,
    rock_cycle: RockCycle,
    jet_cycle: JetCycle,
}
impl Map {
    fn new(jet_cycle: Vec<JetDirection>) -> Self {
        let mut ret = Self {
            map: HashMap::new(),
            height: 0,
            rock_cycle: RockCycle::new(),
            jet_cycle: JetCycle::new(jet_cycle),
        };
        for i in 0..=7 {
            ret.map.insert((i, 0), MapItem::Floor);
        }
        return ret;
    }
    fn add_rock(&mut self) -> Rock {
        let rock = self.rock_cycle.yield_next();
        // println!("Adding rock: {:?}", rock);
        let rock_points = rock.get_rock_points();
        let mut positioned_rock_points: Vec<(i64, i64)> = rock_points
            .iter()
            .map(|(x, y)| (x + 2, y + self.height + 4))
            .collect();
        positioned_rock_points.iter().for_each(|(x, y)| {
            self.map.insert((*x, *y), MapItem::Rock);
        });
        loop {
            // Try jet pushing
            let jet = self.jet_cycle.yield_next();
            let candidate_points: Vec<(i64, i64)> = positioned_rock_points
                .iter()
                .map(|(x, y)| (x + jet.get_direction().0, y + jet.get_direction().1))
                .collect();

            // Check if all candidate points are empty
            let map_is_empty = candidate_points
                .iter()
                .filter(|f| !positioned_rock_points.contains(f))
                .map(|p| self.map.get(p))
                .all(|item| item == None);
            // Check if all candidate points are within range
            let map_is_in_range = candidate_points
                .iter()
                .map(|(x, _y)| *x >= 0 && *x < 7)
                .all(|b| b);
            if map_is_empty && map_is_in_range {
                positioned_rock_points.iter().for_each(|(x, y)| {
                    self.map.remove(&(*x, *y));
                });
                candidate_points.iter().for_each(|(x, y)| {
                    self.map.insert((*x, *y), MapItem::Rock);
                });
                positioned_rock_points = candidate_points;
            }
            // Apply gravity, maybe
            let candidate_points: Vec<(i64, i64)> = positioned_rock_points
                .iter()
                .map(|(x, y)| (*x, *y - 1))
                .collect();
            // // Check if all candidate points are empty
            let map_is_empty = candidate_points
                .iter()
                .filter(|f| !positioned_rock_points.contains(f))
                .map(|p| self.map.get(p))
                .all(|item| item == None);
            // // Check if all candidate points are within range

            if map_is_empty {
                positioned_rock_points.iter().for_each(|(x, y)| {
                    self.map.remove(&(*x, *y));
                });
                candidate_points.iter().for_each(|(x, y)| {
                    self.map.insert((*x, *y), MapItem::Rock);
                });
                positioned_rock_points = candidate_points;
            }
            if !map_is_empty {
                self.height = positioned_rock_points
                    .iter()
                    .map(|(_x, y)| *y)
                    .max()
                    .unwrap()
                    .max(self.height);
                break;
            }
        }
        rock
    }
    fn print(&self) {
        println!();
        for y in (0..self.height + 4).rev() {
            for x in 0..7 {
                match self.map.get(&(x, y)) {
                    Some(MapItem::Rock) => print!("#"),
                    Some(MapItem::Floor) => print!("+"),
                    _ => print!("."),
                }
            }
            println!();
        }
    }
}

fn main() {
    let input: Vec<JetDirection> = include_str!("../../data/day17.txt")
        .chars()
        .map(|c| JetDirection::from_char(c))
        .collect();

    fn loop_for_rocks(input: &Vec<JetDirection>, n_rocks: usize) -> i64 {
        let mut map = Map::new(input.clone());
        let mut pattern = 0;

        for i in 0..n_rocks {
            let rock = map.add_rock();
            // Check if the last rock is in the right position
            if map.map.contains_key(&(3, map.height))
                & map.map.contains_key(&(2, map.height))
                & map.map.contains_key(&(4, map.height))
                & map.map.contains_key(&(5, map.height))
                & (rock == Rock::Bar)
            {
                println!(
                    "Found it! Height of block: {}. Diff: {} at iteration {}",
                    map.height,
                    map.height - pattern,
                    i
                );
                pattern = map.height;
            }
        }
        return map.height;
    }
    // Part 1
    dbg!(loop_for_rocks(&input, 2022));
    // Part 2

    const N_ROCKS: usize = 1000000000;
    // Get the height of a block
    // Empirically, we know block starts at 220
    const BLOCK_START: usize = 220;
    let mut map = Map::new(input);
    let mut heights: Vec<i64> = Vec::new();
    for i in 0..1960 {
        map.add_rock();
        heights.push(map.height)
    }
    let block = heights.iter().skip(BLOCK_START).collect::<Vec<&i64>>();
    dbg!(block.len());
    let pre_block_height: i64 = *heights.iter().nth(BLOCK_START - 1).unwrap();
    dbg!(pre_block_height);
    let block_height: i64 = **block.iter().max().unwrap() - **block.iter().min().unwrap();
    dbg!(block_height);
    // How many blocks?
    let nblocks: i64 = (N_ROCKS as i64 - BLOCK_START as i64) as i64 / block.len() as i64;
    dbg!(nblocks);
    // How many more numbers do we need to draw from a block?
    let remainder: i64 =
        ((N_ROCKS as i64 - BLOCK_START as i64) as i64) % (nblocks * block.len() as i64);

    dbg!(remainder);
    let remainder_block: i64 = block.iter().nth(remainder as usize).map_or(0, |f| **f) - block[0];
    dbg!(remainder_block);
    let res = pre_block_height + (nblocks as i64 * block_height) + remainder_block + nblocks;
    dbg!(res);
}

use std::cmp::Ordering;
// use regex;
// #[derive(Debug, Copy, Clone)]
// struct Materials {
//     ore: usize,
//     clay: usize,
//     obsidian: usize,
//     geode: usize,
// }
// impl Materials {
//     fn new() -> Self {
//         Self {
//             ore: 0,
//             clay: 0,
//             obsidian: 0,
//             geode: 0,
//         }
//     }
// }
// #[derive(Debug)]
// struct Robots {
//     ore_robot: usize,
//     clay_robot: usize,
//     obsidian_robot: usize,
//     geode_robot: usize,
// }
// impl Robots {
//     fn new(n_ore: usize) -> Self {
//         Self {
//             ore_robot: n_ore,
//             clay_robot: 0,
//             obsidian_robot: 0,
//             geode_robot: 0,
//         }
//     }
//     fn reset(&mut self) {
//         self.ore_robot = 0;
//         self.clay_robot = 0;
//         self.obsidian_robot = 0;
//         self.geode_robot = 0;
//     }
//     fn collect(&self, old_materials: Materials) -> Materials {
//         Materials {
//             ore: old_materials.ore + self.ore_robot,
//             clay: old_materials.clay + self.clay_robot,
//             obsidian: old_materials.obsidian + self.obsidian_robot,
//             geode: old_materials.geode + self.geode_robot,
//         }
//     }
// }
// #[derive(Debug, Copy, Clone)]
// struct BluePrint {
//     ore_robot: usize,
//     clay_robot: usize,
//     obsidian_robot: (usize, usize),
//     geode_robot: (usize, usize),
// }
// struct Factory {
//     robots: Robots,
//     materials: Materials,
//     blueprints: BluePrint,
//     robot_being_built: Robots,
// }
// impl Factory {
//     fn new(blueprints: BluePrint) -> Self {
//         Self {
//             robots: Robots::new(1),
//             materials: Materials::new(),
//             blueprints,
//             robot_being_built: Robots::new(0),
//         }
//     }
//     fn print(&self) {
//         println!("Materials: {:?}", self.materials);
//         println!("Robots: {:?}", self.robots);
//     }
//     fn run(&mut self) {
//         // We start with no robots being made
//         self.robot_being_built.reset();
//         // Decide what to do with the materials
//         // Can we make geode robot?
//         if self.materials.ore >= self.blueprints.geode_robot.0
//             && self.materials.obsidian >= self.blueprints.geode_robot.1
//         {
//             self.materials.ore -= self.blueprints.geode_robot.0;
//             self.materials.obsidian -= self.blueprints.geode_robot.1;
//             self.robots.geode_robot += 1;
//             self.robot_being_built.geode_robot = 1;
//         }
//         // Can we make obsidian robot?
//         if self.materials.ore >= self.blueprints.obsidian_robot.0
//             && self.materials.clay >= self.blueprints.obsidian_robot.1
//             && self.robots.obsidian_robot < 2
//         {
//             self.materials.ore -= self.blueprints.obsidian_robot.0;
//             self.materials.clay -= self.blueprints.obsidian_robot.1;
//             self.robots.obsidian_robot += 1;
//             self.robot_being_built.obsidian_robot = 1;
//         }
//         // Can we make a clay robot
//         if self.materials.ore >= self.blueprints.clay_robot && self.robots.clay_robot < 4 {
//             self.materials.ore -= self.blueprints.clay_robot;
//             self.robots.clay_robot += 1;
//             self.robot_being_built.clay_robot = 1;
//         }
//         // Can we make an ore robot?
//         if self.materials.ore >= self.blueprints.ore_robot && self.robots.ore_robot < 1 {
//             self.materials.ore -= self.blueprints.ore_robot;
//             self.robots.ore_robot += 1;
//             self.robot_being_built.ore_robot = 1;
//         } // First, robots collect materials
//         self.materials = self.robots.collect(self.materials);
//         // Substract materials collected due to robots being built (a hack)
//         self.materials.ore -= self.robot_being_built.ore_robot;
//         self.materials.clay -= self.robot_being_built.clay_robot;
//         self.materials.obsidian -= self.robot_being_built.obsidian_robot;
//         self.materials.geode -= self.robot_being_built.geode_robot;
//     }
// }
// fn main() {
//     let reg = regex::Regex::new(r"(\d+)").unwrap();
//     let input: Vec<BluePrint> = include_str!("../../data/day19.txt")
//         .split("\n")
//         .map(|c| {
//             reg.captures_iter(c)
//                 .map(|c| c[1].parse::<usize>().unwrap())
//                 .collect::<Vec<usize>>()
//         })
//         .map(|c| BluePrint {
//             ore_robot: c[1],
//             clay_robot: c[2],
//             obsidian_robot: (c[3], c[4]),
//             geode_robot: (c[5], c[6]),
//         })
//         .collect();
//     let mut factory = Factory::new(input[0]);
//     for i in 1..=24 {
//         println!("Minute: {}", i);
//         factory.run();
//         factory.print();
//         println!();
//     }
// }
// Tried to be too clever https://github.com/orlp/aoc2022/blob/master/src/bin/day19.rs
// Couldnt find analytical solution
use std::collections::BinaryHeap;
use std::str::FromStr;

use hashbrown::HashSet;
use itertools::Itertools;

const ORE: usize = 0;
const CLAY: usize = 1;
const OBS: usize = 2;
const GEODE: usize = 3;

#[derive(Debug)]
struct Blueprint {
    ore_costs: [u32; 4],
    max_ore_cost: u32,
    obsidian_clay_cost: u32,
    geode_obsidian_cost: u32,
}

impl Blueprint {
    fn from_str(s: &str) -> Self {
        let nums = s.split_ascii_whitespace().filter_map(|s| s.parse().ok());
        let (ore, clay, obs_ore, obs, geo_ore, geo) = nums.collect_tuple().unwrap();
        Blueprint {
            ore_costs: [ore, clay, obs_ore, geo_ore],
            max_ore_cost: ore.max(clay).max(obs_ore).max(geo_ore),
            obsidian_clay_cost: obs,
            geode_obsidian_cost: geo,
        }
    }
}
#[derive(Copy, Clone, Debug)]
pub struct Priority<P, T>(pub P, pub T);

impl<P: Ord + Eq, T> Ord for Priority<P, T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<P: Ord + Eq, T> PartialOrd for Priority<P, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<P: Eq, T> PartialEq for Priority<P, T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<P: Eq, T> Eq for Priority<P, T> {}
impl Blueprint {
    pub fn best_num_geodes(&self, minutes: u32) -> u32 {
        let mut best = 0;
        let mut executions = BinaryHeap::new();
        executions.push(Priority(1, Execution::new()));
        let mut seen = HashSet::new();
        while let Some(Priority(upper, ex)) = executions.pop() {
            if upper <= best {
                break;
            }
            let choices = (0..4).map(|r| ex.build_robot(r, self, minutes)).flatten();
            for next in choices {
                let next_upper = next.geode_upper_bound(self, minutes);
                if next_upper > best {
                    best = best.max(next.geode_lower_bound(self, minutes));
                    if next.minutes < minutes && !seen.contains(&next) {
                        seen.insert(next.clone());
                        executions.push(Priority(next_upper, next));
                    }
                }
            }
        }

        best
    }
}

#[derive(Clone, Hash, Eq, PartialEq, Default)]
struct Execution {
    robots: [u32; 4],
    resources: [u32; 4],
    minutes: u32,
}

impl Execution {
    fn new() -> Self {
        let mut slf = Self::default();
        slf.robots[0] = 1;
        slf
    }

    pub fn geode_lower_bound(&self, bp: &Blueprint, max_mins: u32) -> u32 {
        // We only construct geode bots.
        let mut robots = self.robots;
        let mut res = self.resources;
        for _m in self.minutes..max_mins {
            let new_bot = res[ORE] >= bp.ore_costs[GEODE] && res[OBS] >= bp.geode_obsidian_cost;
            for r in 0..4 {
                res[r] += robots[r];
            }
            res[ORE] -= bp.ore_costs[GEODE] * new_bot as u32;
            res[OBS] -= new_bot as u32 * bp.geode_obsidian_cost;
            robots[GEODE] += new_bot as u32;
        }

        res[GEODE]
    }

    pub fn geode_upper_bound(&self, bp: &Blueprint, max_mins: u32) -> u32 {
        // We greedily build robots, but the costs for one type of robot are
        // not subtracted from the pool of resources of the other robots, and we
        // can build multiple robot types at once.
        let mut robots = self.robots;
        let mut ores_for = [self.resources[0]; 4];
        let [_, mut clay, mut obs, mut geodes] = self.resources;
        for _m in self.minutes..max_mins {
            let new_bot = [
                ores_for[ORE] >= bp.ore_costs[ORE],
                ores_for[CLAY] >= bp.ore_costs[CLAY],
                ores_for[OBS] >= bp.ore_costs[OBS] && clay >= bp.obsidian_clay_cost,
                ores_for[GEODE] >= bp.ore_costs[GEODE] && obs >= bp.geode_obsidian_cost,
            ];

            for r in 0..4 {
                ores_for[r] += robots[ORE] - new_bot[r] as u32 * bp.ore_costs[r];
            }
            clay += robots[CLAY] - new_bot[OBS] as u32 * bp.obsidian_clay_cost;
            obs += robots[OBS] - new_bot[GEODE] as u32 * bp.geode_obsidian_cost;
            geodes += robots[GEODE];

            for r in 0..4 {
                robots[r] += new_bot[r] as u32;
            }
        }

        geodes
    }

    pub fn build_robot(&self, resource: usize, bp: &Blueprint, max_mins: u32) -> Option<Execution> {
        let have_enough_already = match resource {
            ORE => self.robots[ORE] >= bp.max_ore_cost,
            CLAY => self.robots[CLAY] >= bp.obsidian_clay_cost,
            OBS => self.robots[OBS] >= bp.geode_obsidian_cost,
            _ => false,
        };
        let costs = [
            bp.ore_costs[resource],
            bp.obsidian_clay_cost * (resource == OBS) as u32,
            bp.geode_obsidian_cost * (resource == GEODE) as u32,
        ];
        let [ore_t, clay_t, obs_t] = [ORE, CLAY, OBS].map(|r| {
            if costs[r] <= self.resources[r] {
                return Some(0);
            }
            (costs[r] - self.resources[r] + self.robots[r] - 1).checked_div(self.robots[r])
        });
        let delay = 1 + ore_t?.max(clay_t?).max(obs_t?);

        let mut ret = self.clone();
        for r in 0..4 {
            ret.resources[r] += delay * ret.robots[r] - costs.get(r).unwrap_or(&0)
        }
        ret.minutes += delay;
        ret.robots[resource] += 1;
        (!have_enough_already && ret.minutes <= max_mins).then_some(ret)
    }
}

fn main() {
    let input = include_str!("../../data/day19.txt");
    let start = std::time::Instant::now();

    let bps: Vec<Blueprint> = input.lines().map(|l| Blueprint::from_str(l)).collect_vec();
    let p1_best = bps.iter().map(|bp| bp.best_num_geodes(24));
    let part1: u32 = p1_best.enumerate().map(|(i, b)| b * (i as u32 + 1)).sum();
    let part2: u32 = bps[..3].iter().map(|bp| bp.best_num_geodes(32)).product();

    println!("part1: {part1}");
    println!("part2: {part2}");
    println!("time: {:?}", start.elapsed());
}

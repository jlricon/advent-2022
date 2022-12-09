use itertools::Itertools;

struct Rope {
    knots: Vec<Knot>,
}
const N_KNOTS: usize = 10;
impl Rope {
    fn new() -> Rope {
        Rope {
            knots: vec![Knot::new(); N_KNOTS],
        }
    }
    fn move_head(&mut self, movement: &Movement) -> Vec<(isize, isize)> {
        let mut tail_movements = vec![];
        for _ in 0..movement.times {
            // Apply the movement to the head
            self.knots[0].move_head_dir(&movement.dir);
            // Correct all the tails
            for knot in 0..(N_KNOTS) {
                if let Some(tail_position) = self.knots[knot].set_tail_position() {
                    // self.knots[knot + 1].head = tail_position;
                    if let Some(v) = self.knots.get_mut(knot + 1) {
                        v.head = tail_position;
                    };
                }
            }
            tail_movements.push(self.knots[9].head);
        }
        tail_movements
    }
}
#[derive(Clone, Debug)]
struct Knot {
    head: (isize, isize),
    tail: (isize, isize),
}
impl Knot {
    fn new() -> Knot {
        Knot {
            head: (0, 0),
            tail: (0, 0),
        }
    }
    fn move_head_dir(&mut self, dir: &Direction) {
        match dir {
            Direction::Up => self.head.1 += 1,
            Direction::Down => self.head.1 -= 1,
            Direction::Left => self.head.0 -= 1,
            Direction::Right => self.head.0 += 1,
        }
    }
    fn set_tail_position(&mut self) -> Option<(isize, isize)> {
        let tail_position = self.get_vector_from_head();
        // This doesn't really need an euclidean norm
        let euclidean_norm =
            ((tail_position.0.pow(2) + tail_position.1.pow(2)) as f64).sqrt() as usize;
        // If manhattan norm >1 move tail towards head
        if euclidean_norm > 1 {
            let direction = (tail_position.0.signum(), tail_position.1.signum());
            self.tail.0 += direction.0 as isize;
            self.tail.1 += direction.1 as isize;
            Some(self.tail)
        } else {
            None
        }
    }
    fn move_head(&mut self, movement: &Movement) -> Vec<(isize, isize)> {
        let mut tail_movements = vec![self.tail];
        for _ in 0..movement.times {
            self.move_head_dir(&movement.dir);
            if let Some(tail_position) = self.set_tail_position() {
                tail_movements.push(tail_position);
            }
        }
        // dbg!(&movement, &tail_movements);
        tail_movements
    }

    fn get_vector_from_head(&self) -> (isize, isize) {
        (
            self.head.0 as isize - self.tail.0 as isize,
            self.head.1 as isize - self.tail.1 as isize,
        )
    }
}
#[derive(Debug)]
struct Movement {
    dir: Direction,
    times: usize,
}
#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let input = include_str!("../../data/day9.txt")
        .split('\n')
        .map(|x| {
            let mut splitted = x.split(' ');
            let direction = splitted.next().unwrap();
            let distance = splitted.next().unwrap().parse::<isize>().unwrap();
            match direction {
                "U" => Movement {
                    dir: Direction::Up,
                    times: distance as usize,
                },
                "D" => Movement {
                    dir: Direction::Down,
                    times: distance as usize,
                },
                "L" => Movement {
                    dir: Direction::Left,
                    times: distance as usize,
                },
                "R" => Movement {
                    dir: Direction::Right,
                    times: distance as usize,
                },
                _ => panic!("Unknown direction"),
            }
        })
        .collect::<Vec<_>>();
    let mut rope = Knot {
        head: (0, 0),
        tail: (0, 0),
    };
    let positions: Vec<(isize, isize)> = input.iter().flat_map(|v| rope.move_head(v)).collect();

    dbg!(positions.iter().unique().count());

    // Part 2
    let mut rope = Rope::new();
    let positions: Vec<(isize, isize)> = input.iter().flat_map(|v| rope.move_head(v)).collect();

    dbg!(positions.iter().unique().count());
}

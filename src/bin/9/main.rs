use std::cmp;
use std::collections::HashSet;

#[derive(Debug)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl std::str::FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "L" => Self::Left,
            "U" => Self::Up,
            "R" => Self::Right,
            "D" => Self::Down,
            _ => return Err(()),
        })
    }
}

struct Input {
    direction: Direction,
    count: i32,
}

#[derive(Hash, Clone, Copy, PartialEq, Eq, Default)]
struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    fn distance_from(&self, other: &Position) -> u32 {
        cmp::max(self.x.abs_diff(other.x), self.y.abs_diff(other.y))
    }

    fn directions_to(&self, other: &Position) -> Vec<Direction> {
        match ((other.x - self.x).signum(), (other.y - self.y).signum()) {
            (0, 1) => vec![Direction::Up],
            (0, -1) => vec![Direction::Down],
            (-1, 0) => vec![Direction::Left],
            (1, 0) => vec![Direction::Right],
            (1, 1) => vec![Direction::Up, Direction::Right],
            (1, -1) => vec![Direction::Down, Direction::Right],
            (-1, -1) => vec![Direction::Down, Direction::Left],
            (-1, 1) => vec![Direction::Up, Direction::Left],
            _ => unreachable!(),
        }
    }

    fn apply_direction(&self, direction: &Direction) -> Self {
        let mut ret = self.clone();

        match direction {
            Direction::Left => ret.x -= 1,
            Direction::Right => ret.x += 1,
            Direction::Up => ret.y += 1,
            Direction::Down => ret.y -= 1,
        };

        ret
    }
}

struct State<const KNOTS: usize> {
    pub knots: [Position; KNOTS],
    pub visited: HashSet<Position>,
}

impl<const KNOTS: usize> State<KNOTS> {
    fn new() -> Self {
        let mut visited = HashSet::new();

        visited.insert(Default::default());

        Self {
            knots: [Default::default(); KNOTS],
            visited,
        }
    }

    fn update(&mut self, input: &Input) {
        (0..input.count).for_each(|_| {
            self.knots[0] = self.knots[0].apply_direction(&input.direction);

            (1..KNOTS).for_each(|index| {
                let current = self.knots[index];
                let prev = self.knots[index - 1];

                (prev.distance_from(&current) > 1).then(|| {
                    current.directions_to(&prev).iter().for_each(|dir| {
                        self.knots[index] = self.knots[index].apply_direction(&dir);
                    })
                });
            });

            if let Some(last) = self.knots.last() {
                self.visited.insert(last.clone());
            }
        })
    }
}

fn do_the_thing<const KNOTS: usize>(input: &str) {
    let mut state = State::<KNOTS>::new();

    input
        .lines()
        .filter_map(|l| l.split_once(" "))
        .map(|(dir, num)| {
            Ok::<Input, ()>(Input {
                direction: dir.parse()?,
                count: num.parse().map_err(|_| ())?,
            })
        })
        .flatten()
        .for_each(|input| state.update(&input));

    println!("tail positions: {}", state.visited.len());
}

// part 1
fn main() {
    do_the_thing::<2>(include_str!("./input.txt"));
    do_the_thing::<10>(include_str!("./input.txt"));
}

// // test
// fn main() {
//     do_the_thing::<10>(include_str!("./test.txt"));
// }

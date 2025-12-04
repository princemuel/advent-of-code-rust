//! https://adventofcode.com/2025/day/1

use aoc2025::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left  = 0,
    Right = 1,
}

impl Direction {}

const DIAL_SIZE: isize = 100;

fn main() {
    use std::time::Instant;

    let data = read_input();

    let start = Instant::now();
    println!("Part 1: {}", part_one(&data));
    println!("Elapsed time: {:.4} seconds", start.elapsed().as_secs_f64());

    let start = Instant::now();
    println!("Part 2: {}", part_two(&data));
    println!("Elapsed time: {:.4} seconds", start.elapsed().as_secs_f64());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rotation(Direction, isize);
impl Rotation {
    pub const fn direction(&self) -> Direction { self.0 }

    pub const fn steps(&self) -> isize { self.1 }

    pub fn list(input: &str) -> Vec<Self> {
        input.lines().filter_map(|line| line.parse().ok()).collect()
    }
}

impl FromStr for Rotation {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let buffer = s.trim().as_bytes();
        if buffer.is_empty() {
            return Err(ParseError::EmptyInput);
        }

        let direction = match buffer[0] {
            b'L' | b'l' => Direction::Left,
            b'R' | b'r' => Direction::Right,
            c => return Err(ParseError::InvalidDirection(c)),
        };

        let steps = str::from_utf8(&buffer[1..])
            .map_err(|_| ParseError::InvalidSteps)?
            .parse()
            .map_err(|_| ParseError::InvalidSteps)?;

        Ok(Self(direction, steps))
    }
}

fn part_one(input: &str) -> isize {
    let rotations = Rotation::list(input);

    let (_, count) = rotations.iter().fold((50, 0), |(pos, count), item| {
        let position = match item.direction() {
            Direction::Left => (pos - item.steps()).rem_euclid(DIAL_SIZE),
            Direction::Right => (pos + item.steps()).rem_euclid(DIAL_SIZE),
        };

        let count = if position == 0 { count + 1 } else { count };

        (position, count)
    });

    count
}

fn part_two(input: &str) -> isize {
    let rotations = Rotation::list(input);

    let (_, count) = rotations.iter().fold((50, 0), |(pos, count), item| {
        // Count how many times we cross 0 during this rotation
        let crossings = rotate(pos, item.direction(), item.steps());
        let position = match item.direction() {
            Direction::Left => (pos - item.steps()).rem_euclid(DIAL_SIZE),
            Direction::Right => (pos + item.steps()).rem_euclid(DIAL_SIZE),
        };

        (position, count + crossings)
    });

    count
}

fn rotate(start: isize, direction: Direction, steps: isize) -> isize {
    // Steps required to reach zero from the current pos.
    let steps_to_zero = match direction {
        Direction::Right => (DIAL_SIZE - start).rem_euclid(DIAL_SIZE),
        Direction::Left => start,
    };

    // Steps remaining after the first time we land on zero.
    let remaining = steps - steps_to_zero;

    match steps_to_zero {
        0 => steps / DIAL_SIZE,
        _ if remaining >= 0 => 1 + (remaining / DIAL_SIZE),
        _ => 0,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ParseError {
    EmptyInput,
    InvalidDirection(u8),
    InvalidSteps,
}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyInput => write!(f, "Empty string"),
            Self::InvalidDirection(c) => write!(f, "Invalid direction: {}", *c as char),
            Self::InvalidSteps => write!(f, "Invalid steps"),
        }
    }
}
impl Error for ParseError {}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    prop_compose! {
        fn arb_direction()(b in any::<bool>()) -> Direction {
            if b { Direction::Left } else { Direction::Right }
        }
    }

    prop_compose! {
        fn arb_rotation()(
            dir in arb_direction(),
            dist in 0isize..5000
        ) -> Rotation {
            Rotation(dir, dist)
        }
    }

    prop_compose! {
        fn arb_rotation_list()(list in prop::collection::vec(arb_rotation(), 0..200)) -> Vec<Rotation> {
            list
        }
    }

    fn render(rot: &Rotation) -> String {
        let d = match rot.direction() {
            Direction::Left => "L",
            Direction::Right => "R",
        };
        format!("{d}{}", rot.steps())
    }

    fn render_list(xs: &[Rotation]) -> String {
        xs.iter().map(render).collect::<Vec<_>>().join("\n")
    }

    #[test]
    fn scenario_rotation_parses_valid_inputs() {
        assert_eq!(
            "L10".parse::<Rotation>().unwrap(),
            Rotation(Direction::Left, 10)
        );
        assert_eq!(
            "R7".parse::<Rotation>().unwrap(),
            Rotation(Direction::Right, 7)
        );
        assert_eq!(
            " l3 ".parse::<Rotation>().unwrap(),
            Rotation(Direction::Left, 3)
        );
        assert_eq!(
            "r25".parse::<Rotation>().unwrap(),
            Rotation(Direction::Right, 25)
        );
    }

    #[test]
    fn scenario_rotation_rejects_invalid_direction() {
        let err = "X5".parse::<Rotation>().unwrap_err();
        matches!(err, ParseError::InvalidDirection(_));
    }

    #[test]
    fn scenario_part1_hits_zero_exactly_once() {
        assert_eq!(part_one("R50"), 1);
    }

    #[test]
    fn scenario_part1_wraps_correctly_without_false_hits() {
        assert_eq!(part_one("R60"), 0);
    }

    #[test]
    fn scenario_part2_counts_multiple_zero_crossings() {
        assert_eq!(part_two("R250"), 3);
    }

    #[test]
    fn scenario_part2_handles_mixed_directions() {
        assert_eq!(part_two("L50\nR150"), 2);
    }

    #[test]
    fn scenario_part2_with_large_value() {
        assert_eq!(part_two("R10000"), 100);
    }

    proptest! {
        #[test]
        fn prop_part1_never_exceeds_part_two(rotations in arb_rotation_list()) {
            let input = render_list(&rotations);
            let p1 = part_one(&input);
            let p2 = part_two(&input);
            prop_assert!(p1 <= p2);
        }
    }

    proptest! {
        #[test]
        fn prop_zero_crossings_are_non_negative(
            start in 0..DIAL_SIZE,
            steps in 0..5000isize,
            dir in arb_direction()
        ) {
            let result = rotate(start, dir, steps);
            prop_assert!(result >= 0);
        }
    }

    proptest! {
        #[test]
        fn prop_cycles_count_exactly(
            cycles in 0..2000isize,
            dir in arb_direction()
        ) {
            let steps = cycles * DIAL_SIZE;
            let crosses = rotate(0, dir, steps);
            prop_assert_eq!(crosses, cycles);
        }
    }

    proptest! {
        #[test]
        fn prop_part2_matches_manual_model(rotations in arb_rotation_list()) {
            let mut pos = 50;
            let mut expected = 0;

            for rot in &rotations {
                expected += rotate(pos, rot.direction(), rot.steps());
                pos = match rot.direction() {
                    Direction::Left => (pos - rot.steps()).rem_euclid(DIAL_SIZE),
                    Direction::Right => (pos + rot.steps()).rem_euclid(DIAL_SIZE),
                };
            }

            let input = render_list(&rotations);
            prop_assert_eq!(part_two(&input), expected);
        }
    }

    prop_compose! {
        fn arb_safe_rotation()(
            dist in 1..10isize,  // Much smaller range
            is_left in prop::bool::ANY
        ) -> Rotation {
            let direction = if is_left {
                Direction::Left
            } else {
                Direction::Right
            };
            Rotation(direction, dist)
        }
    }

    proptest! {
        #[test]
        fn prop_safe_rotations_never_count(
            rotations in prop::collection::vec(arb_safe_rotation(), 0..10)
        ) {
            let input = render_list(&rotations);
            prop_assert_eq!(part_one(&input), 0);
            prop_assert_eq!(part_two(&input), 0);
        }
    }
}

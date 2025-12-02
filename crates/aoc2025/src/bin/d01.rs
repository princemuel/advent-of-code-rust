use aoc2025::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Right,
    Left,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rotation(Direction, i64);
impl Rotation {
    pub const fn direction(&self) -> Direction { self.0 }

    pub const fn distance(&self) -> i64 { self.1 }
}

impl core::str::FromStr for Rotation {
    type Err = ParseRotationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let buffer = s.trim().as_bytes();
        if buffer.is_empty() {
            return Err(ParseRotationError::Empty);
        }

        let direction = match buffer[0] {
            b'R' | b'r' => Direction::Right,
            b'L' | b'l' => Direction::Left,
            c => return Err(ParseRotationError::InvalidDirection(c)),
        };

        let distance = str::from_utf8(&buffer[1..])
            .map_err(|_| ParseRotationError::InvalidDistance)?
            .parse()
            .map_err(|_| ParseRotationError::InvalidDistance)?;

        Ok(Self(direction, distance))
    }
}

fn part1(input: impl AsRef<str>) -> i64 {
    let input = input.as_ref();

    let mut count = 0;
    let mut position = 50;

    let rotations: Vec<Rotation> = input.lines().filter_map(|line| line.parse().ok()).collect();

    for rot in &rotations {
        match rot.direction() {
            Direction::Right => {
                position = (position + rot.distance()) % 100;
            }
            Direction::Left => {
                position = (position - rot.distance()).rem_euclid(100);
            }
        };

        if position == 0 {
            count += 1
        }
    }

    count
}

fn part2(input: impl AsRef<str>) -> i64 {
    let _input = input.as_ref();
    0
}

#[derive(Debug, Clone, Copy)]
enum ParseRotationError {
    Empty,
    InvalidDirection(u8),
    InvalidDistance,
}
impl core::fmt::Display for ParseRotationError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Empty => write!(f, "Empty string"),
            Self::InvalidDirection(c) => write!(f, "Invalid direction: {}", *c as char),
            Self::InvalidDistance => write!(f, "Invalid distance"),
        }
    }
}
impl core::error::Error for ParseRotationError {}

fn main() {
    use std::time::Instant;

    let input = input();
    let start = Instant::now();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    let elapsed = start.elapsed();
    println!("Elapsed time: {:.4} seconds", elapsed.as_secs_f64());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 0);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 0);
    }
}

use core::str::FromStr;

use aoc2025::read_input;

fn main() {
    use std::time::Instant;

    let data = read_input();

    let start = Instant::now();

    println!("Part 1: {}", part_one(&data));
    println!("Part 2: {}", part_two(&data));

    println!("Elapsed time: {:.4} s", start.elapsed().as_secs_f64());
}

/// A joltage value (1-9)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Joltage(u8);

impl Joltage {
    pub fn new(value: u8) -> Option<Self> { (1..=9).contains(&value).then_some(Self(value)) }

    fn value(self) -> u8 { self.0 }
}

impl TryFrom<char> for Joltage {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        c.to_digit(10).and_then(|d| Joltage::new(d as u8)).ok_or(())
    }
}

impl From<Joltage> for u32 {
    fn from(value: Joltage) -> Self { value.0 as u32 }
}

/// A two-digit joltage reading
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct TwoDigitJoltage {
    tens: Joltage,
    ones: Joltage,
}

impl TwoDigitJoltage {
    fn new(tens: Joltage, ones: Joltage) -> Self { Self { tens, ones } }

    fn total(self) -> u32 { self.tens.value() as u32 * 10 + self.ones.value() as u32 }
}

/// A bank of batteries
#[derive(Debug, Clone)]
struct BatteryBank(Vec<Joltage>);

impl BatteryBank {
    fn joltages(&self) -> &[Joltage] { self.0.as_slice() }

    // fn max_joltage(&self) -> Option<TwoDigitJoltage> {
    //     (0..self.joltages().len().saturating_sub(1))
    //         .filter_map(|i| {
    //             let tens = self.joltages()[i];
    //             let ones = self.joltages()[i + 1..].iter().copied().max()?;
    //             Some(TwoDigitJoltage::new(tens, ones))
    //         })
    //         .max_by_key(|j| j.total())
    // }

    fn max_joltage(&self) -> Option<TwoDigitJoltage> {
        if self.joltages().len() < 2 {
            return None;
        }

        // Build suffix maximums using scan (right-to-left fold)
        let suffix_maxes: Vec<Joltage> = self
            .joltages()
            .iter()
            .copied()
            .rev()
            .scan(None, |max_so_far, joltage| {
                *max_so_far = Some(max_so_far.map_or(joltage, |m: Joltage| m.max(joltage)));
                *max_so_far
            })
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect();

        // Find the maximum two-digit joltage
        self.joltages()
            .iter()
            .copied()
            .zip(suffix_maxes.iter().skip(1).copied())
            .map(|(tens, ones)| TwoDigitJoltage::new(tens, ones))
            .max_by_key(|j| j.total())
    }
}

impl FromStr for BatteryBank {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let joltages: Result<Vec<_>, _> = s.chars().map(Joltage::try_from).collect();

        Ok(BatteryBank(joltages?))
    }
}

/// Solve part 1.
fn part_one(input: impl AsRef<str>) -> u32 {
    input
        .as_ref()
        .lines()
        .filter_map(|line| line.parse::<BatteryBank>().ok())
        .filter_map(|bank| bank.max_joltage())
        .map(|j| j.total())
        .sum()
}

/// Solve part 2.
fn part_two(input: impl AsRef<str>) -> i64 {
    let _ = input.as_ref().lines();
    0
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ParseError {
    EmptyInput,
    InvalidJolt(u8),
}
impl core::fmt::Display for ParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::EmptyInput => write!(f, "Empty string"),
            Self::InvalidJolt(c) => write!(f, "Invalid direction: {}", *c as char),
        }
    }
}
impl core::error::Error for ParseError {}

#[cfg(test)]
mod tests {}

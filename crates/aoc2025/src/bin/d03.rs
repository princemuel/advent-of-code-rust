use core::marker::PhantomData;

use aoc2025::prelude::*;

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

/// A joltage value between 1-9 (inclusive)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Joltage(usize);

impl Joltage {
    pub fn new(value: usize) -> Option<Self> { (1..=9).contains(&value).then_some(Self(value)) }

    fn value(self) -> usize { self.0 }
}

impl TryFrom<char> for Joltage {
    type Error = ParseError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        c.to_digit(10)
            .and_then(|digit| Joltage::new(digit as usize))
            .ok_or(ParseError::InvalidJoltage(c))
    }
}

/// A two-digit joltage reading formed by selecting two batteries
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct TwoDigitJoltage {
    tens: Joltage,
    ones: Joltage,
}

impl TwoDigitJoltage {
    fn new(tens: Joltage, ones: Joltage) -> Self { Self { tens, ones } }

    fn total(self) -> usize { self.tens.value() * 10 + self.ones.value() }
}

/// Marker: Battery bank has been parsed but not validated
struct Parsed;

/// Marker: Battery bank has been validated and is ready for computation
struct Validated;

/// A bank of batteries
#[derive(Debug, Clone)]
struct BatteryBank<State>(Vec<Joltage>, PhantomData<State>);

impl BatteryBank<Parsed> {
    fn joltages(&self) -> &[Joltage] { self.0.as_slice() }

    /// Validate that the bank has sufficient batteries for computation
    fn validate(self) -> Result<BatteryBank<Validated>, ParseError> {
        if self.joltages().is_empty() {
            return Err(ParseError::EmptyBank);
        }

        if self.joltages().len() < 2 {
            return Err(ParseError::InsufficientBatteries);
        }

        Ok(BatteryBank(self.joltages().to_vec(), PhantomData))
    }
}

impl BatteryBank<Validated> {
    fn joltages(&self) -> &[Joltage] { self.0.as_slice() }

    // Find the maximum two-digit joltage possible from this bank
    /// Algorithm: For each position as tens digit, find the max ones digit
    /// that can follow it, then return the overall maximum.
    #[allow(unused)]
    fn compute_maximum_joltage(&self) -> TwoDigitJoltage {
        (0..self.joltages().len() - 1)
            .filter_map(|position| {
                let tens = self.joltages()[position];
                let ones = self.joltages()[position + 1..].iter().copied().max()?;
                Some(TwoDigitJoltage::new(tens, ones))
            })
            .max_by_key(|joltage| joltage.total())
            .expect("Validated bank must have at least one valid pair")
    }

    /// Optimized O(n) version using precomputed suffix maximums
    fn max_joltage(&self) -> TwoDigitJoltage {
        let suffix_maximums = self.build_suffix_maximums();

        self.joltages()
            .iter()
            .copied()
            .zip(suffix_maximums.iter().skip(1).copied())
            .map(|(tens, ones)| TwoDigitJoltage::new(tens, ones))
            .max_by_key(|joltage| joltage.total())
            .expect("Validated bank must have at least one valid pair")
    }

    /// Build an array where suffix_maximums[i] = max joltage from i to end
    fn build_suffix_maximums(&self) -> Vec<Joltage> {
        self.joltages()
            .iter()
            .copied()
            .rev()
            .scan(None, |total, curr| {
                *total = Some(total.map_or(curr, |prev_max: Joltage| prev_max.max(curr)));
                *total
            })
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect()
    }

    fn max_twelve_digit_joltage(&self) -> usize {
        const TARGET: usize = 12;
        let joltages = self.joltages();

        if joltages.len() < TARGET {
            return 0;
        }

        let to_remove = joltages.len() - TARGET;
        let mut stack: Vec<Joltage> = Vec::with_capacity(TARGET);
        let mut removals_left = to_remove;

        for &joltage in joltages {
            // Pop smaller elements from stack while we can still afford removals
            // and the current element is larger
            while !stack.is_empty() && removals_left > 0 && stack.last().unwrap() < &joltage {
                stack.pop();
                removals_left -= 1;
            }

            stack.push(joltage);
        }

        // Convert the first TARGET elements to the result number
        stack
            .iter()
            .take(TARGET)
            .fold(0usize, |acc, &j| acc * 10 + j.value())
    }
}

impl FromStr for BatteryBank<Parsed> {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let joltages: Result<Vec<_>, _> = s.chars().map(Joltage::try_from).collect();

        Ok(BatteryBank(joltages?, PhantomData))
    }
}

/// Solve part 1.
fn part_one(input: &str) -> usize {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .filter_map(|line| line.parse::<BatteryBank<Parsed>>().ok()?.validate().ok())
        .map(|bank| bank.max_joltage().total())
        .sum()
}

/// Solve part 2.
fn part_two(input: &str) -> usize {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .filter_map(|line| line.parse::<BatteryBank<Parsed>>().ok()?.validate().ok())
        .map(|bank| bank.max_twelve_digit_joltage())
        .sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ParseError {
    InvalidJoltage(char),
    EmptyBank,
    InsufficientBatteries,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidJoltage(ch) => {
                write!(f, "Invalid joltage character: '{ch}'")
            }
            ParseError::EmptyBank => {
                write!(f, "Battery bank is empty")
            }
            ParseError::InsufficientBatteries => {
                write!(f, "Battery bank needs at least 2 batteries")
            }
        }
    }
}
impl Error for ParseError {}

#[cfg(test)]
mod tests {}

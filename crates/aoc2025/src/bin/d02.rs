use core::num::NonZeroU64;
use core::ops::RangeInclusive;
use core::str::FromStr;

use aoc2025::read_input;

fn main() {
    use std::time::Instant;

    let data = read_input();

    let start = Instant::now();
    match part_one(&data) {
        Ok(result) => println!("Part 1: {}", result),
        Err(e) => eprintln!("Part 1 error: {}", e),
    };
    println!("Elapsed time: {:.4} s", start.elapsed().as_secs_f64());

    let start = Instant::now();
    match part_two(&data) {
        Ok(result) => println!("Part 2: {}", result),
        Err(e) => eprintln!("Part 2 error: {}", e),
    };
    println!("Elapsed time: {:.4} s", start.elapsed().as_secs_f64());
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Range(RangeInclusive<u64>);

impl Range {
    pub fn new(start: u64, end: u64) -> Result<Self, ParseError> {
        if start > end {
            Err(ParseError::InRange { start, end })
        } else {
            Ok(Self(start..=end))
        }
    }

    /// Get the inner range
    pub fn inner(&self) -> &RangeInclusive<u64> { &self.0 }

    /// Convert to owned range
    pub fn into_inner(self) -> RangeInclusive<u64> { self.0 }

    pub fn start(&self) -> u64 { *self.0.start() }

    pub fn end(&self) -> u64 { *self.0.end() }

    pub fn contains(&self, value: u64) -> bool { self.0.contains(&value) }

    /// Iterate over all values in the range
    pub fn iter(&self) -> impl Iterator<Item = u64> + '_ { self.0.clone() }
}

impl IntoIterator for Range {
    type IntoIter = RangeInclusive<u64>;
    type Item = u64;

    fn into_iter(self) -> Self::IntoIter { self.0.clone() }
}

impl IntoIterator for &Range {
    type IntoIter = RangeInclusive<u64>;
    type Item = u64;

    fn into_iter(self) -> Self::IntoIter { self.0.clone() }
}

impl FromStr for Range {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('-').collect();

        if parts.len() != 2 {
            return Err(ParseError::InvalidFormat(format!(
                "Expected format 'start-end', got '{s}'",
            )));
        }

        let start = parts[0]
            .parse()
            .map_err(|_| ParseError::InvalidNumber(parts[0].to_string()))?;

        let end = parts[1]
            .parse()
            .map_err(|_| ParseError::InvalidNumber(parts[1].to_string()))?;

        Range::new(start, end)
    }
}

fn parse_ranges(input: &str) -> Result<Vec<Range>, ParseError> {
    if input.trim().is_empty() {
        return Err(ParseError::EmptyInput);
    }

    input.trim().split(',').map(|s| s.trim().parse()).collect()
}

/// Creates a range of valid pattern values for a given half-length
fn pattern_range(digits: usize) -> RangeInclusive<u64> {
    let min = 10u64.pow(digits as u32 - 1);
    let max = 10u64.pow(digits as u32) - 1;
    min..=max
}

/// Iterator that generates invalid ID candidates for a specific digit length
struct PatternIterator {
    pattern:     RangeInclusive<u64>,
    len:         usize,
    repetitions: usize,
    range:       Range,
}

impl PatternIterator {
    fn new(len: usize, repetitions: usize, range: Range) -> Self {
        Self {
            pattern: pattern_range(len),
            len,
            repetitions,
            range,
        }
    }
}

impl Iterator for PatternIterator {
    type Item = InvalidId;

    fn next(&mut self) -> Option<Self::Item> {
        // Iterate through the pattern range
        self.pattern.find_map(|value| {
            Pattern::new(value, self.len)
                .and_then(|pattern| pattern.repeat(self.repetitions))
                .filter(|id| self.range.contains(id.value()))
        })
    }
}

/// A pattern that can be repeated to form an invalid ID
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pattern {
    value:  u64,
    digits: usize,
}

impl Pattern {
    pub fn new(value: u64, digits: usize) -> Option<Self> {
        let s = value.to_string();

        if s.len() != digits || s.starts_with('0') {
            None
        } else {
            Some(Self { value, digits })
        }
    }

    pub fn to_invalid_id(&self) -> InvalidId {
        let s = self.value.to_string();
        let repeated = format!("{}{}", s, s);

        // Parse to NonZeroU64 - we know this won't be zero
        let value = repeated
            .parse()
            .expect("Pattern concatenation should produce valid NonZeroU64");

        // We've validated the pattern doesn't have leading zeros, so the repeated
        // version is guaranteed valid
        InvalidId::new_unchecked(value)
    }

    // Repeat this pattern a given number of times
    pub fn repeat(&self, times: usize) -> Option<InvalidId> {
        let pattern_str = self.value.to_string();
        let repeated = pattern_str.repeat(times);

        repeated
            .parse::<NonZeroU64>()
            .ok()
            .map(InvalidId::new_unchecked)
    }
}

/// An ID that is guaranteed to be invalid (repeated pattern)
///
/// # Invariants
/// - Must have even digit length
/// - When split in half, both halves are identical
/// - The pattern (half) does not start with '0'
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InvalidId(NonZeroU64);
impl InvalidId {
    pub const fn value(&self) -> u64 { self.0.get() }

    /// Validate and construct from raw parts (used by Pattern)
    ///
    /// # Safety
    /// This bypasses validation - only use when you've already validated
    /// the pattern externally (e.g., in Pattern::to_invalid_id)
    pub(crate) fn new_unchecked(value: NonZeroU64) -> Self { Self(value) }

    /// Check if a number has a repeating pattern with given minimum repetitions
    /// Returns Some(pattern_length) if valid, None otherwise
    fn has_repeating_pattern(s: &str, min_repetitions: usize) -> Option<usize> {
        let len = s.len();

        // Try all possible pattern lengths from 1 to len/min_repetitions
        for pattern_len in 1..=len / min_repetitions {
            // Check if total length is divisible by pattern length
            if !len.is_multiple_of(pattern_len) {
                continue;
            }

            let repetitions = len / pattern_len;

            // Must have at least min_repetitions
            if repetitions < min_repetitions {
                continue;
            }

            let pattern = &s[0..pattern_len];

            // Skip patterns with leading zeros
            if pattern.starts_with('0') {
                continue;
            }

            // Check if this pattern repeats throughout the entire string
            let is_repeating = (0..len)
                .step_by(pattern_len)
                .all(|i| &s[i..i + pattern_len] == pattern);

            if is_repeating {
                return Some(pattern_len);
            }
        }

        None
    }

    /// Check if exactly 2 repetitions (Part 1)
    pub fn is_double_pattern(s: &str) -> bool {
        let len = s.len();

        // Must be even length for exactly 2 repetitions
        if !len.is_multiple_of(2) {
            return false;
        }

        Self::has_repeating_pattern(s, 2)
            .map(|pattern_len| pattern_len * 2 == len)
            .unwrap_or(false)
    }

    /// Check if 2+ repetitions (Part 2)
    pub fn is_repeating_pattern(s: &str) -> bool { Self::has_repeating_pattern(s, 2).is_some() }
}

impl TryFrom<u64> for InvalidId {
    type Error = ParseError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        // Convert to NonZeroU64 first
        let non_zero =
            NonZeroU64::new(value).ok_or_else(|| ParseError::InvalidNumber("0".to_string()))?;

        // Reuse string we need anyway for validation
        let s = value.to_string();

        // Default to Part 2 behavior (2+ repetitions)
        if Self::is_repeating_pattern(&s) {
            Ok(Self(non_zero))
        } else {
            Err(ParseError::NotRepeatedPattern(
                NotRepeatedPatternReason::NoRepeatingPattern { value },
            ))
        }
    }
}

impl From<InvalidId> for u64 {
    fn from(id: InvalidId) -> Self { id.value() }
}

impl From<InvalidId> for NonZeroU64 {
    fn from(id: InvalidId) -> Self { id.0 }
}

/// Generate invalid IDs with a constraint on repetitions
/// - min_reps = 2, max_reps = 2: Part 1 (exactly 2 repetitions)
/// - min_reps = 2, max_reps = usize::MAX: Part 2 (2+ repetitions)
fn generate_invalid_candidates(
    range: &Range,
    min_reps: usize,
    max_reps: usize,
) -> impl Iterator<Item = InvalidId> + '_ {
    let start_len = range.start().to_string().len();
    let end_len = range.end().to_string().len();

    (start_len..=end_len)
        .flat_map(move |total_len| {
            // For each total length, find all valid (pattern_len, repetitions) pairs
            (1..=total_len).filter_map(move |pattern_len| {
                if total_len % pattern_len == 0 {
                    let repetitions = total_len / pattern_len;
                    if repetitions >= min_reps && repetitions <= max_reps {
                        Some((pattern_len, repetitions))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
        })
        .flat_map(move |(pattern_len, repetitions)| {
            PatternIterator::new(pattern_len, repetitions, range.clone())
        })
}

/// Solve part 1 (exactly 2 repetitions)
fn part_one(input: impl AsRef<str>) -> Result<u64, ParseError> {
    let ranges = parse_ranges(input.as_ref())?;

    let result = ranges
        .iter()
        .flat_map(|range| generate_invalid_candidates(range, 2, 2))
        .map(|id| id.value())
        .sum();

    Ok(result)
}

/// Solve part 2 (2+ repetitions)
fn part_two(input: impl AsRef<str>) -> Result<u64, ParseError> {
    use std::collections::HashSet;

    let ranges = parse_ranges(input.as_ref())?;

    // Collect into HashSet to remove duplicates
    let result: HashSet<u64> = ranges
        .iter()
        .flat_map(|range| generate_invalid_candidates(range, 2, usize::MAX))
        .map(|id| id.value())
        .collect();

    Ok(result.into_iter().sum())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotRepeatedPatternReason {
    OddLength {
        value:  u64,
        length: usize,
    },
    HalvesNotEqual {
        value: u64,
        left:  String,
        right: String,
    },
    LeadingZero {
        value:   u64,
        pattern: String,
    },
    NoRepeatingPattern {
        value: u64,
    },
}

impl core::fmt::Display for NotRepeatedPatternReason {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            NotRepeatedPatternReason::OddLength { value, length } => {
                write!(
                    f,
                    "{value} has odd length {length} (cannot be split evenly)",
                )
            }
            NotRepeatedPatternReason::HalvesNotEqual { value, left, right } => {
                write!(
                    f,
                    "{value} splits into '{left}' and '{right}' which are not equal",
                )
            }
            NotRepeatedPatternReason::LeadingZero { value, pattern } => {
                write!(f, "{value} has pattern '{pattern}' with leading zero")
            }
            NotRepeatedPatternReason::NoRepeatingPattern { value } => {
                write!(f, "{value} does not have a repeating pattern")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    EmptyInput,
    InvalidFormat(String),
    InvalidNumber(String),
    InRange { start: u64, end: u64 },
    NotRepeatedPattern(NotRepeatedPatternReason),
}

impl core::fmt::Display for ParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::EmptyInput => write!(f, "Empty input"),
            Self::InvalidFormat(s) => write!(f, "Invalid format: {s}"),
            Self::InvalidNumber(s) => write!(f, "Invalid number: {s}"),
            Self::InRange { start, end } => {
                write!(f, "Invalid range: {start} > {end}")
            }
            Self::NotRepeatedPattern(reason) => write!(f, "{}", reason),
        }
    }
}
impl core::error::Error for ParseError {}

#[cfg(test)]
mod tests {
    use super::*;
    const EX: &str = "";

    #[test]
    fn t1() {
        assert!(part_one(EX).is_err());
    }

    #[test]
    fn t2() {
        assert!(part_one(EX).is_err());
    }

    const EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
                           1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
                           824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_valid_range_wraps_std_range() {
        let range = Range::new(10, 20).unwrap();

        // Can use standard range methods
        assert_eq!(range.start(), 10);
        assert_eq!(range.end(), 20);
        assert!(range.contains(15));
        assert!(!range.contains(5));

        // Can iterate
        let values: Vec<u64> = range.iter().collect();
        assert_eq!(values.len(), 11);
        assert_eq!(values[0], 10);
        assert_eq!(values[10], 20);
    }

    #[test]
    fn test_valid_range_into_iterator() {
        let range = Range::new(5, 8).unwrap();

        // Can use in for loop directly
        let mut sum = 0;
        for n in &range {
            sum += n;
        }
        assert_eq!(sum, 5 + 6 + 7 + 8);

        // Can consume
        let values: Vec<u64> = range.into_iter().collect();
        assert_eq!(values, vec![5, 6, 7, 8]);
    }

    #[test]
    fn test_pattern_range() {
        let range = pattern_range(1);
        assert_eq!(range, 1..=9);

        let range = pattern_range(2);
        assert_eq!(range, 10..=99);

        let range = pattern_range(3);
        assert_eq!(range, 100..=999);
    }

    // #[test]
    // fn test_example() {
    //     assert_eq!(part_one(EXAMPLE).unwrap(), 1227775554);
    // }

    #[test]
    fn test_range_iteration_equivalence() {
        let range = Range::new(11, 22).unwrap();

        // All these should work the same
        let v1: Vec<u64> = range.iter().collect();
        let v2: Vec<u64> = range.clone().into_iter().collect();
        let v3: Vec<u64> = (&range).into_iter().collect();

        assert_eq!(v1, v2);
        assert_eq!(v2, v3);
    }

    #[test]
    fn test_invalid_id_try_from_valid() {
        assert_eq!(InvalidId::try_from(11).unwrap().value(), 11);
        assert_eq!(InvalidId::try_from(1010).unwrap().value(), 1010);
        assert_eq!(InvalidId::try_from(123123).unwrap().value(), 123123);
    }

    #[test]
    fn test_invalid_id_try_from_invalid() {
        // Odd length
        let result = InvalidId::try_from(123);
        assert!(matches!(
            result,
            Err(ParseError::NotRepeatedPattern(
                NotRepeatedPatternReason::OddLength {
                    value:  123,
                    length: 3,
                }
            ))
        ));

        // Different halves
        let result = InvalidId::try_from(1001);
        assert!(matches!(
            result,
            Err(ParseError::NotRepeatedPattern(
                NotRepeatedPatternReason::HalvesNotEqual { value: 1001, .. }
            ))
        ));

        // Zero
        let result = InvalidId::try_from(0);
        assert!(matches!(result, Err(ParseError::InvalidNumber(_))));
    }

    // #[test]
    // fn test_invalid_id_from_str() {
    //     assert_eq!("11".parse::<InvalidId>().unwrap().value(), 11);
    //     assert_eq!("1010".parse::<InvalidId>().unwrap().value(), 1010);

    //     assert!("123".parse::<InvalidId>().is_err());
    //     assert!("abc".parse::<InvalidId>().is_err());
    // }

    #[test]
    fn test_error_messages() {
        let err = InvalidId::try_from(123).unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("odd length"));
        assert!(msg.contains("123"));

        let err = InvalidId::try_from(1001).unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("not equal"));
        assert!(msg.contains("10"));
        assert!(msg.contains("01"));
    }

    #[test]
    fn test_non_zero_guarantee() {
        // InvalidId can never be zero
        assert!(InvalidId::try_from(0).is_err());

        // Can safely use NonZeroU64 APIs
        let id = InvalidId::try_from(11).unwrap();
        let non_zero: NonZeroU64 = id.into();
        assert_eq!(non_zero.get(), 11);
    }

    #[test]
    fn test_conversions() {
        let id = InvalidId::try_from(1010).unwrap();

        // To u64
        let val: u64 = id.into();
        assert_eq!(val, 1010);

        // To NonZeroU64
        let non_zero: NonZeroU64 = id.into();
        assert_eq!(non_zero.get(), 1010);
    }

    #[test]
    fn test_ord_and_hash() {
        let id1 = InvalidId::try_from(11).unwrap();
        let id2 = InvalidId::try_from(22).unwrap();
        let id3 = InvalidId::try_from(11).unwrap();

        // Ordering
        assert!(id1 < id2);
        assert_eq!(id1, id3);

        // Can use in hash-based collections
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(id1);
        set.insert(id2);
        set.insert(id3);
        assert_eq!(set.len(), 2); // id1 and id3 are equal
    }

    #[test]
    fn test_part1_pattern_detection() {
        assert!(InvalidId::is_double_pattern("11"));
        assert!(InvalidId::is_double_pattern("1010"));
        assert!(InvalidId::is_double_pattern("123123"));

        assert!(!InvalidId::is_double_pattern("111")); // 3 repetitions
        assert!(!InvalidId::is_double_pattern("1001")); // different halves
    }

    #[test]
    fn test_part2_pattern_detection() {
        assert!(InvalidId::is_repeating_pattern("11")); // 2 reps
        assert!(InvalidId::is_repeating_pattern("111")); // 3 reps
        assert!(InvalidId::is_repeating_pattern("1010")); // 2 reps
        assert!(InvalidId::is_repeating_pattern("565656")); // 3 reps
        assert!(InvalidId::is_repeating_pattern("2121212121")); // 5 reps

        assert!(!InvalidId::is_repeating_pattern("1001"));
        assert!(!InvalidId::is_repeating_pattern("123"));
    }

    #[test]
    fn test_pattern_repeat() {
        let pattern = Pattern::new(12, 2).unwrap();
        assert_eq!(pattern.repeat(2).unwrap().value(), 1212);
        assert_eq!(pattern.repeat(3).unwrap().value(), 121212);
    }
}

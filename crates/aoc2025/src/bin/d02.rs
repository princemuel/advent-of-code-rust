use core::num::NonZeroU64;
use core::ops::RangeInclusive;
use core::str::FromStr;

use aoc2025::read_input;

fn main() {
    use std::time::Instant;

    let input = read_input();

    let start = Instant::now();
    match part_one(&input) {
        Ok(result) => println!("Part 1: {}", result),
        Err(e) => eprintln!("Part 1 error: {}", e),
    };
    println!("Elapsed time: {:.4} s", start.elapsed().as_secs_f64());

    let start = Instant::now();
    match part_two(&input) {
        Ok(result) => println!("Part 2: {}", result),
        Err(e) => eprintln!("Part 2 error: {}", e),
    };
    println!("Elapsed time: {:.4} s", start.elapsed().as_secs_f64());
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdRange(RangeInclusive<u64>);

impl IdRange {
    pub fn new(start: u64, end: u64) -> Result<Self, ParseError> {
        if start > end {
            Err(ParseError::InvalidRangeBounds { start, end })
        } else {
            Ok(Self(start..=end))
        }
    }

    pub fn inner(&self) -> &RangeInclusive<u64> { &self.0 }

    pub fn into_inner(self) -> RangeInclusive<u64> { self.0 }

    pub fn start(&self) -> u64 { *self.0.start() }

    pub fn end(&self) -> u64 { *self.0.end() }

    pub fn contains(&self, value: u64) -> bool { self.0.contains(&value) }

    pub fn iter(&self) -> impl Iterator<Item = u64> + '_ { self.0.clone() }
}

impl IntoIterator for IdRange {
    type IntoIter = RangeInclusive<u64>;
    type Item = u64;

    fn into_iter(self) -> Self::IntoIter { self.0.clone() }
}

impl IntoIterator for &IdRange {
    type IntoIter = RangeInclusive<u64>;
    type Item = u64;

    fn into_iter(self) -> Self::IntoIter { self.0.clone() }
}

impl FromStr for IdRange {
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

        IdRange::new(start, end)
    }
}

fn parse_id_ranges(input: &str) -> Result<Vec<IdRange>, ParseError> {
    if input.trim().is_empty() {
        return Err(ParseError::EmptyInput);
    }

    input.trim().split(',').map(|s| s.trim().parse()).collect()
}

/// Range of possible pattern seeds for a given digit length.
///
/// For example:
/// - digits = 1 -> 1..=9
/// - digits = 2 -> 10..=99
fn pattern_seed_span(digits: usize) -> RangeInclusive<u64> {
    let min = 10u64.pow(digits as u32 - 1);
    let max = 10u64.pow(digits as u32) - 1;
    min..=max
}

/// A numeric pattern (seed) that can be repeated to form an invalid ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DigitPattern {
    value:  u64,
    digits: usize,
}

impl DigitPattern {
    pub fn new(value: u64, digits: usize) -> Option<Self> {
        let s = value.to_string();

        if s.len() != digits || s.starts_with('0') {
            None
        } else {
            Some(Self { value, digits })
        }
    }

    pub fn repeat(&self, times: usize) -> Option<InvalidId> {
        let pattern_str = self.value.to_string();
        let repeated = pattern_str.repeat(times);

        repeated
            .parse::<NonZeroU64>()
            .ok()
            .map(InvalidId::new_unchecked)
    }

    pub fn to_invalid_id(&self) -> InvalidId {
        let repeated = format!("{0}{0}", self.value);
        let nz = repeated
            .parse::<NonZeroU64>()
            .expect("Pattern concatenation should produce valid NonZeroU64");
        InvalidId::new_unchecked(nz)
    }
}

/// An ID that is guaranteed to be invalid (repeated pattern).
///
/// Invariants:
/// - At least 2 repetitions of some non-empty pattern.
/// - Pattern does not start with '0'.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InvalidId(NonZeroU64);

impl InvalidId {
    pub const fn value(&self) -> u64 { self.0.get() }

    /// Safety: caller must ensure `value` satisfies the invariants.
    pub(crate) fn new_unchecked(value: NonZeroU64) -> Self { Self(value) }

    /// Core helper: find a repeating pattern inside `s`.
    ///
    /// Returns Some(pattern_len) if there is a pattern of length `pattern_len`
    /// repeated at least `min_repetitions` times.
    fn has_repeating_pattern(s: &str, min_repetitions: usize) -> Option<usize> {
        let len = s.len();

        for pattern_len in 1..=len / min_repetitions {
            if !len.is_multiple_of(pattern_len) {
                continue;
            }

            let repetitions = len / pattern_len;
            if repetitions < min_repetitions {
                continue;
            }

            let pattern = &s[0..pattern_len];

            if pattern.starts_with('0') {
                continue;
            }

            let is_repeating = (0..len)
                .step_by(pattern_len)
                .all(|i| &s[i..i + pattern_len] == pattern);

            if is_repeating {
                return Some(pattern_len);
            }
        }

        None
    }

    /// Exactly 2 repetitions (used for Part 1).
    pub fn is_double_pattern(s: &str) -> bool {
        let len = s.len();

        if !len.is_multiple_of(2) {
            return false;
        }

        Self::has_repeating_pattern(s, 2)
            .map(|pattern_len| pattern_len * 2 == len)
            .unwrap_or(false)
    }

    /// At least 2 repetitions (used for Part 2).
    pub fn is_repeating_pattern(s: &str) -> bool { Self::has_repeating_pattern(s, 2).is_some() }
}

impl TryFrom<u64> for InvalidId {
    type Error = ParseError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        // First ensure non zero.
        let non_zero =
            NonZeroU64::new(value).ok_or_else(|| ParseError::InvalidNumber("0".to_string()))?;

        let s = value.to_string();

        if Self::is_repeating_pattern(&s) {
            Ok(Self(non_zero))
        } else {
            let reason = NotRepeatedPatternReason::explain(value);
            Err(ParseError::NotRepeatedPattern(reason))
        }
    }
}

impl From<InvalidId> for u64 {
    fn from(id: InvalidId) -> Self { id.value() }
}

impl From<InvalidId> for NonZeroU64 {
    fn from(id: InvalidId) -> Self { id.0 }
}

// ============================================================================
// Candidate generation
// ============================================================================

fn digit_count(n: u64) -> usize { if n == 0 { 1 } else { n.to_string().len() } }

/// Generate invalid IDs within `range` with repetition constraints.
///
/// - `min_reps = 2, max_reps = 2`     => exactly two repetitions (Part 1).
/// - `min_reps = 2, max_reps = usize::MAX` => at least two repetitions (Part
///   2).
fn generate_invalid_ids_for_range(
    range: &IdRange,
    min_reps: usize,
    max_reps: usize,
) -> impl Iterator<Item = InvalidId> + '_ {
    let min_len = digit_count(range.start());
    let max_len = digit_count(range.end());

    (min_len..=max_len).flat_map(move |total_len| {
        // For this total length, consider all (pattern_len, repetitions) pairs.
        (1..=total_len)
            .filter_map(move |pattern_len| {
                if total_len % pattern_len != 0 {
                    return None;
                }

                let repetitions = total_len / pattern_len;
                if repetitions < min_reps || repetitions > max_reps {
                    return None;
                }

                Some((pattern_len, repetitions))
            })
            .flat_map(move |(pattern_len, repetitions)| {
                // For each pattern length and repetition count, iterate all
                // possible seeds and keep those whose repeated form falls
                // inside the range.
                pattern_seed_span(pattern_len).filter_map(move |seed_value| {
                    let pattern = DigitPattern::new(seed_value, pattern_len)?;
                    let repeated = pattern.repeat(repetitions)?;
                    let id_value = repeated.value();

                    if range.contains(id_value) { Some(repeated) } else { None }
                })
            })
    })
}

fn sum_invalid_ids_in_ranges(
    input: impl AsRef<str>,
    min_reps: usize,
    max_reps: usize,
    deduplicate: bool,
) -> Result<u64, ParseError> {
    use std::collections::HashSet;

    let ranges = parse_id_ranges(input.as_ref())?;

    let result = if !deduplicate {
        ranges
            .iter()
            .flat_map(|range| generate_invalid_ids_for_range(range, min_reps, max_reps))
            .map(|id| id.value())
            .sum()
    } else {
        let result: HashSet<u64> = ranges
            .iter()
            .flat_map(|range| generate_invalid_ids_for_range(range, min_reps, max_reps))
            .map(|id| id.value())
            .collect();

        result.into_iter().sum()
    };

    Ok(result)
}

/// Part 1: exactly 2 repetitions.
fn part_one(input: impl AsRef<str>) -> Result<u64, ParseError> {
    sum_invalid_ids_in_ranges(input, 2, 2, false)
}

/// Part 2: 2 or more repetitions, unique IDs only.
fn part_two(input: impl AsRef<str>) -> Result<u64, ParseError> {
    sum_invalid_ids_in_ranges(input, 2, usize::MAX, true)
}

// ============================================================================
// Errors
// ============================================================================

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

impl NotRepeatedPatternReason {
    /// Best-effort explanation of why a value is not considered a repeated
    /// pattern.
    pub fn explain(value: u64) -> Self {
        let s = value.to_string();
        let length = s.len();

        if !length.is_multiple_of(2) {
            return NotRepeatedPatternReason::OddLength { value, length };
        }

        let mid = length / 2;
        let left = &s[..mid];
        let right = &s[mid..];

        if left != right {
            return NotRepeatedPatternReason::HalvesNotEqual {
                value,
                left: left.to_string(),
                right: right.to_string(),
            };
        }

        if left.starts_with('0') {
            return NotRepeatedPatternReason::LeadingZero {
                value,
                pattern: left.to_string(),
            };
        }

        NotRepeatedPatternReason::NoRepeatingPattern { value }
    }
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
    InvalidRangeBounds { start: u64, end: u64 },
    NotRepeatedPattern(NotRepeatedPatternReason),
}

impl core::fmt::Display for ParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::EmptyInput => write!(f, "Empty input"),
            Self::InvalidFormat(s) => write!(f, "Invalid format: {s}"),
            Self::InvalidNumber(s) => write!(f, "Invalid number: {s}"),
            Self::InvalidRangeBounds { start, end } => {
                write!(f, "Invalid range: {start} > {end}")
            }
            Self::NotRepeatedPattern(reason) => write!(f, "{reason}"),
        }
    }
}

impl core::error::Error for ParseError {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    const EMPTY_INPUT: &str = "";

    #[test]
    fn scenario_empty_input_fails_for_part_one() {
        assert!(part_one(EMPTY_INPUT).is_err());
    }

    #[test]
    fn scenario_empty_input_fails_for_part_two() {
        assert!(part_two(EMPTY_INPUT).is_err());
    }

    #[test]
    fn scenario_id_range_wraps_std_range_behavior() {
        let range = IdRange::new(10, 20).unwrap();

        assert_eq!(range.start(), 10);
        assert_eq!(range.end(), 20);
        assert!(range.contains(15));
        assert!(!range.contains(5));

        let values: Vec<u64> = range.iter().collect();
        assert_eq!(values.len(), 11);
        assert_eq!(values[0], 10);
        assert_eq!(values[10], 20);
    }

    #[test]
    fn scenario_id_range_supports_into_iterator_for_borrowed_and_owned() {
        let range = IdRange::new(5, 8).unwrap();

        let mut sum = 0;
        for n in &range {
            sum += n;
        }
        assert_eq!(sum, 5 + 6 + 7 + 8);

        let values: Vec<u64> = range.into_iter().collect();
        assert_eq!(values, vec![5, 6, 7, 8]);
    }

    #[test]
    fn scenario_pattern_seed_span_matches_digit_lengths() {
        let range = pattern_seed_span(1);
        assert_eq!(range, 1..=9);

        let range = pattern_seed_span(2);
        assert_eq!(range, 10..=99);

        let range = pattern_seed_span(3);
        assert_eq!(range, 100..=999);
    }

    #[test]
    fn scenario_id_range_iteration_yields_same_values_for_all_paths() {
        let range = IdRange::new(11, 22).unwrap();

        let v1: Vec<u64> = range.iter().collect();
        let v2: Vec<u64> = range.clone().into_iter().collect();
        let v3: Vec<u64> = (&range).into_iter().collect();

        assert_eq!(v1, v2);
        assert_eq!(v2, v3);
    }

    #[test]
    fn scenario_invalid_id_try_from_accepts_repeated_values() {
        assert_eq!(InvalidId::try_from(11).unwrap().value(), 11);
        assert_eq!(InvalidId::try_from(1010).unwrap().value(), 1010);
        assert_eq!(InvalidId::try_from(123123).unwrap().value(), 123123);
    }

    #[test]
    fn scenario_invalid_id_try_from_rejects_non_repeated_values_with_reasons() {
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

        // Different halves (no repeating pattern of any length either).
        let result = InvalidId::try_from(1001);
        assert!(matches!(
            result,
            Err(ParseError::NotRepeatedPattern(
                NotRepeatedPatternReason::HalvesNotEqual { value: 1001, .. }
            ))
        ));

        // Zero is rejected as invalid number before repeated pattern analysis.
        let result = InvalidId::try_from(0);
        assert!(matches!(result, Err(ParseError::InvalidNumber(_))));
    }

    #[test]
    fn scenario_error_messages_are_human_readable() {
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
    fn scenario_invalid_id_can_never_be_zero_and_exposes_non_zero_api() {
        assert!(InvalidId::try_from(0).is_err());

        let id = InvalidId::try_from(11).unwrap();
        let non_zero: NonZeroU64 = id.into();
        assert_eq!(non_zero.get(), 11);
    }

    #[test]
    fn scenario_invalid_id_converts_to_u64_and_non_zero_u64() {
        let id = InvalidId::try_from(1010).unwrap();

        let val: u64 = id.into();
        assert_eq!(val, 1010);

        let non_zero: NonZeroU64 = id.into();
        assert_eq!(non_zero.get(), 1010);
    }

    #[test]
    fn scenario_invalid_id_is_orderable_and_hashable() {
        let id1 = InvalidId::try_from(11).unwrap();
        let id2 = InvalidId::try_from(22).unwrap();
        let id3 = InvalidId::try_from(11).unwrap();

        assert!(id1 < id2);
        assert_eq!(id1, id3);

        let mut set = HashSet::new();
        set.insert(id1);
        set.insert(id2);
        set.insert(id3);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn scenario_pattern_detection_for_double_repetition_behaves_as_expected() {
        assert!(InvalidId::is_double_pattern("11"));
        assert!(InvalidId::is_double_pattern("1010"));
        assert!(InvalidId::is_double_pattern("123123"));

        assert!(!InvalidId::is_double_pattern("111"));
        assert!(!InvalidId::is_double_pattern("1001"));
    }

    #[test]
    fn scenario_pattern_detection_for_two_or_more_repetitions_behaves_as_expected() {
        assert!(InvalidId::is_repeating_pattern("11"));
        assert!(InvalidId::is_repeating_pattern("111"));
        assert!(InvalidId::is_repeating_pattern("1010"));
        assert!(InvalidId::is_repeating_pattern("565656"));
        assert!(InvalidId::is_repeating_pattern("2121212121"));

        assert!(!InvalidId::is_repeating_pattern("1001"));
        assert!(!InvalidId::is_repeating_pattern("123"));
    }

    #[test]
    fn scenario_digit_pattern_repeat_builds_longer_invalid_ids() {
        let pattern = DigitPattern::new(12, 2).unwrap();
        assert_eq!(pattern.repeat(2).unwrap().value(), 1212);
        assert_eq!(pattern.repeat(3).unwrap().value(), 121212);
    }

    // If you want to assert the official example once you know it:
    // #[test]
    // fn scenario_example_input_matches_known_answer_for_part_one() {
    //     assert_eq!(part_one(EXAMPLE).unwrap(), 1227775554);
    // }
}

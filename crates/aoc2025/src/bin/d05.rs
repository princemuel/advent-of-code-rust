use core::cmp::Ordering;

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Range(RangeInclusive<isize>);
impl Range {
    /// Create a new Range from start and end (inclusive)
    pub fn new(start: isize, end: isize) -> Self { Self(start..=end) }

    /// Get the start of the range
    pub const fn start(&self) -> isize { *self.0.start() }

    /// Get the end of the range
    pub const fn end(&self) -> isize { *self.0.end() }

    /// Check if this range contains a value
    pub fn contains(&self, value: isize) -> bool { self.0.contains(&value) }

    /// Check if this range overlaps with another range
    pub const fn overlaps(&self, other: &Self) -> bool {
        self.start() <= other.end() && other.start() <= self.end()
    }

    /// Check if this range is adjacent to another (no gap between them)
    pub const fn is_adjacent(&self, other: &Self) -> bool {
        self.end() + 1 == other.start() || other.end() + 1 == self.start()
    }

    /// Merge this range with another (assumes they overlap or are adjacent)
    pub fn merge(&self, other: &Self) -> Range {
        Self::new(self.start().min(other.start()), self.end().max(other.end()))
    }

    /// Check if the range is empty (should never happen with inclusive ranges
    /// where start <= end)
    pub fn is_empty(&self) -> bool { self.0.is_empty() }

    /// Get the count of values in this range
    pub const fn len(&self) -> usize { (self.end() - self.start() + 1) as usize }
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare by start first, then by end if starts are equal
        match self.start().cmp(&other.start()) {
            Ordering::Equal => self.end().cmp(&other.end()),
            ordering => ordering,
        }
    }
}

impl From<(isize, isize)> for Range {
    fn from((start, end): (isize, isize)) -> Self { Range::new(start, end) }
}

/// Solve part 1.
fn part_one(input: &str) -> usize {
    let (ranges, ingredients) = parse_input(input);
    count_fresh_ingredients(&ranges, &ingredients)
}

/// Solve part 2.
fn part_two(input: &str) -> usize {
    let (ranges, ..) = parse_input(input);
    count_all_fresh_ids(&ranges)
}

/// Parse the input into ranges and ingredient IDs
fn parse_input(input: &str) -> (Vec<Range>, Vec<isize>) {
    let mut lines = input.lines().peekable();

    // Parse ranges until we hit a blank line
    let ranges = lines
        .by_ref()
        .map(str::trim)
        .take_while(|line| !line.is_empty())
        .filter_map(|line| {
            let (start, end) = line.split_once("-")?;
            Some(Range::new(start.parse().ok()?, end.parse().ok()?))
        })
        .collect();

    // Parse remaining ingredient IDs
    let ids = lines
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .filter_map(|line| line.parse().ok())
        .collect();

    (ranges, ids)
}

/// Count how many ingredient IDs are fresh (i.e fall in any range)
fn count_fresh_ingredients(ranges: &[Range], ingredients: &[isize]) -> usize {
    ingredients
        .iter()
        .filter(|&&id| ranges.iter().any(|range| range.contains(id)))
        .count()
}

/// Count all distinct IDs covered by the ranges (using merge approach)
fn count_all_fresh_ids(ranges: &[Range]) -> usize {
    merge_ranges(ranges)
        .into_iter()
        .map(|range| range.len())
        .sum()
}

/// Merge overlapping and adjacent ranges
fn merge_ranges(ranges: &[Range]) -> Vec<Range> {
    let mut sorted = ranges.to_vec();
    sorted.sort_unstable();

    sorted
        .into_iter()
        .fold(Vec::with_capacity(ranges.len()), |mut ranges, range| {
            match ranges.last() {
                None => ranges.push(range),
                Some(last) if range.overlaps(last) || range.is_adjacent(last) => {
                    let merged = ranges.pop().unwrap().merge(&range);
                    ranges.push(merged);
                }
                Some(_) => ranges.push(range),
            }
            ranges
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_contains() {
        let range = Range::new(3, 5);
        assert!(range.contains(3));
        assert!(range.contains(4));
        assert!(range.contains(5));
        assert!(!range.contains(2));
        assert!(!range.contains(6));
    }

    #[test]
    fn test_range_overlaps() {
        let r1 = Range::new(3, 5);
        let r2 = Range::new(4, 7);
        let r3 = Range::new(10, 15);

        assert!(r1.overlaps(&r2));
        assert!(r2.overlaps(&r1));
        assert!(!r1.overlaps(&r3));
    }

    #[test]
    fn test_range_adjacent() {
        let r1 = Range::new(3, 5);
        let r2 = Range::new(6, 8);
        let r3 = Range::new(10, 15);

        assert!(r1.is_adjacent(&r2));
        assert!(r2.is_adjacent(&r1));
        assert!(!r1.is_adjacent(&r3));
    }

    #[test]
    fn test_range_merge() {
        let r1 = Range::new(3, 5);
        let r2 = Range::new(4, 8);
        let merged = r1.merge(&r2);

        assert_eq!(merged.start(), 3);
        assert_eq!(merged.end(), 8);
    }

    #[test]
    fn test_range_len() {
        assert_eq!(Range::new(3, 5).len(), 3);
        assert_eq!(Range::new(10, 14).len(), 5);
        assert_eq!(Range::new(1, 1).len(), 1);
    }

    #[test]
    fn test_part_one_example() {
        let input = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32";

        assert_eq!(part_one(input), 3);
    }

    #[test]
    fn test_part_two_example() {
        let input = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32";

        assert_eq!(part_two(input), 14);
    }

    #[test]
    fn test_merge_ranges() {
        // Non-overlapping ranges
        let ranges = vec![Range::new(1, 3), Range::new(5, 7), Range::new(9, 10)];
        assert_eq!(merge_ranges(&ranges), vec![
            Range::new(1, 3),
            Range::new(5, 7),
            Range::new(9, 10)
        ]);

        // Overlapping ranges
        let ranges = vec![Range::new(1, 5), Range::new(3, 8), Range::new(10, 12)];
        assert_eq!(merge_ranges(&ranges), vec![
            Range::new(1, 8),
            Range::new(10, 12)
        ]);

        // Adjacent ranges (should merge)
        let ranges = vec![Range::new(1, 3), Range::new(4, 6), Range::new(7, 9)];
        assert_eq!(merge_ranges(&ranges), vec![Range::new(1, 9)]);

        // Unsorted input
        let ranges = vec![
            Range::new(10, 14),
            Range::new(3, 5),
            Range::new(12, 18),
            Range::new(16, 20),
        ];
        assert_eq!(merge_ranges(&ranges), vec![
            Range::new(3, 5),
            Range::new(10, 20)
        ]);
    }
}

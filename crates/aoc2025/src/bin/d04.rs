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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Direction(i8, i8);
impl Direction {
    pub fn neighbor(&self, row: usize, col: usize) -> Option<(usize, usize)> {
        let nr = row.checked_add_signed(self.row())?;
        let nc = col.checked_add_signed(self.col())?;
        Some((nr, nc))
    }

    pub const fn row(&self) -> isize { self.0 as isize }

    pub const fn col(&self) -> isize { self.1 as isize }
}

const D8: [Direction; 8] = [
    Direction(-1, -1),
    Direction(-1, 0),
    Direction(-1, 1),
    Direction(0, -1),
    Direction(0, 1),
    Direction(1, -1),
    Direction(1, 0),
    Direction(1, 1),
];

/// Solve part 1.
fn part_one(input: &str) -> usize {
    let grid: Vec<_> = input.lines().map(|line| line.as_bytes().to_vec()).collect();

    let rows = grid.len();
    let cols = grid.first().map_or_else(|| 0, |v| v.len());

    let rolls = find_accessible_rolls(&grid, rows, cols);
    rolls.len()
}

/// Solve part 2.
fn part_two(input: &str) -> usize {
    let grid: Vec<_> = input.lines().map(|line| line.as_bytes().to_vec()).collect();

    let rows = grid.len();
    let cols = grid.first().map_or_else(|| 0, |v| v.len());

    // Keep removing until no more rolls are accessible
    iter::successors(Some((grid, 0)), move |(current_grid, _)| {
        let (next_grid, removed) = remove_once(current_grid, rows, cols);
        (removed > 0).then_some((next_grid, removed))
    })
    .map(|(.., total)| total)
    .sum()
}

fn remove_once(grid: &[Vec<u8>], rows: usize, cols: usize) -> (Vec<Vec<u8>>, usize) {
    let rolls = find_accessible_rolls(grid, rows, cols);
    if rolls.is_empty() {
        return (grid.to_vec(), 0);
    }

    let mut grid = grid.to_vec();
    for (row, col) in &rolls {
        grid[*row][*col] = b'.';
    }

    (grid, rolls.len())
}

fn find_accessible_rolls(grid: &[Vec<u8>], rows: usize, cols: usize) -> Vec<(usize, usize)> {
    let is_lonely_roll = |row: usize, col| {
        grid[row][col] == b'@' && count_adjacent_rolls(grid, row, col, rows, cols) < 4
    };

    (0..rows)
        .flat_map(|row| (0..cols).map(move |col| (row, col)))
        .filter(|&(row, col)| is_lonely_roll(row, col))
        .collect()
}

fn count_adjacent_rolls(
    grid: &[Vec<u8>],
    row: usize,
    col: usize,
    rows: usize,
    cols: usize,
) -> usize {
    D8.iter()
        .filter_map(|d| d.neighbor(row, col))
        .filter(|&(row, col)| row < rows && col < cols)
        .filter(|&(row, col)| grid[row][col] == b'@')
        .count()
}

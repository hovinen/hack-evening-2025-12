use std::{env::args, fs::read_to_string};

#[derive(Copy, Clone)]
enum Cell {
    Roll,
    NoRoll,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        if value == '@' {
            Cell::Roll
        } else {
            Cell::NoRoll
        }
    }
}

struct Grid(Vec<Vec<Cell>>);

impl Grid {
    fn from_str(value: &str) -> Self {
        let content = value
            .lines()
            .filter(|r| !r.trim().is_empty())
            .map(|r| r.trim().chars().map(Cell::from).collect::<Vec<_>>())
            .collect::<Vec<Vec<_>>>();
        Self(content)
    }

    fn rows(&self) -> usize {
        self.0.len()
    }

    fn cols(&self) -> usize {
        self.0[0].len()
    }

    fn cell_at(&self, row: usize, col: usize) -> Cell {
        self.0[row][col]
    }

    fn neighbours(&self, row: usize, col: usize) -> Vec<Cell> {
        let mut result = Vec::new();
        if row > 0 {
            if col > 0 {
                result.push(self.0[row - 1][col - 1]);
            }
            result.push(self.0[row - 1][col]);
            if col < self.cols() - 1 {
                result.push(self.0[row - 1][col + 1]);
            }
        }
        if col > 0 {
            result.push(self.0[row][col - 1]);
        }
        if col < self.cols() - 1 {
            result.push(self.0[row][col + 1]);
        }
        if row < self.rows() - 1 {
            if col > 0 {
                result.push(self.0[row + 1][col - 1]);
            }
            result.push(self.0[row + 1][col]);
            if col < self.cols() - 1 {
                result.push(self.0[row + 1][col + 1]);
            }
        }
        result
    }
}

fn count_free_rolls(input: &str) -> usize {
    let grid = Grid::from_str(input);
    let mut count = 0;
    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            if !matches!(grid.cell_at(row, col), Cell::Roll) {
                continue;
            }
            let occupied_sides = grid
                .neighbours(row, col)
                .into_iter()
                .filter(|n| matches!(n, Cell::Roll))
                .count();
            if occupied_sides < 4 {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let input = read_to_string(args().last().expect("Must specify an input file"))
        .expect("Could not read input file");
    let count = count_free_rolls(&input);
    println!("Count: {count}");
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::count_free_rolls;

    #[test]
    fn should_work_on_example() {
        let input = r#"
            ..@@.@@@@.
            @@@.@.@.@@
            @@@@@.@.@@
            @.@@@@..@.
            @@.@@@@.@@
            .@@@@@@@.@
            .@.@.@.@@@
            @.@@@.@@@@
            .@@@@@@@@.
            @.@.@@@.@.
        "#;

        assert_eq!(count_free_rolls(input), 13);
    }

    #[test]
    fn should_work_on_large_test_data() {
        let input = read_to_string("../../input.txt").expect("Cannot open file");

        assert_eq!(count_free_rolls(&input), 1491);
    }
}

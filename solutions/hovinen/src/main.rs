use async_std::{stream::StreamExt as _, task};
use async_stream::stream;
use futures_core::stream::Stream;
use futures_util::pin_mut;
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

    fn neighbours(&self, row: usize, col: usize) -> impl Stream<Item = Cell> {
        stream! {
            if row > 0 {
                if col > 0 {
                    yield self.0[row - 1][col - 1];
                }
                yield self.0[row - 1][col];
                if col < self.cols() - 1 {
                    yield self.0[row - 1][col + 1];
                }
            }
            if col > 0 {
                yield self.0[row][col - 1];
            }
            if col < self.cols() - 1 {
                yield self.0[row][col + 1];
            }
            if row < self.rows() - 1 {
                if col > 0 {
                    yield self.0[row + 1][col - 1];
                }
                yield self.0[row + 1][col];
                if col < self.cols() - 1 {
                    yield self.0[row + 1][col + 1];
                }
            }
        }
    }
}

async fn count_free_rolls(input: &str) -> usize {
    let grid = Grid::from_str(input);
    let mut count = 0;
    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            if !matches!(grid.cell_at(row, col), Cell::Roll) {
                continue;
            }
            let mut occupied_sides = 0;
            let neighbours = grid
                .neighbours(row, col)
                .filter(|neighbour| matches!(neighbour, Cell::Roll));
            pin_mut!(neighbours);
            while let Some(_) = neighbours.next().await {
                occupied_sides += 1;
            }
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
    let count = task::block_on(count_free_rolls(&input));
    println!("Count: {count}");
}

#[cfg(test)]
mod tests {
    use super::count_free_rolls;
    use async_std::task;
    use std::fs::read_to_string;

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

        assert_eq!(task::block_on(count_free_rolls(input)), 13);
    }

    #[test]
    fn should_work_on_large_test_data() {
        let input = read_to_string("../../input.txt").expect("Cannot open file");

        assert_eq!(task::block_on(count_free_rolls(&input)), 1491);
    }
}

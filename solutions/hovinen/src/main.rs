use std::{env::args, fs::read_to_string};

fn count_free_rolls(input: &str) -> usize {
    let lines = input
        .lines()
        .filter(|r| !r.trim().is_empty())
        .map(|r| r.trim().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut count = 0;
    for (row, line) in lines.iter().enumerate() {
        for (col, char) in line.iter().enumerate() {
            if *char != '@' {
                continue;
            }
            let mut occupied_sides = 0;
            if row > 0 {
                if col > 0 {
                    if lines[row - 1][col - 1] == '@' {
                        occupied_sides += 1;
                    }
                }
                if lines[row - 1][col] == '@' {
                    occupied_sides += 1;
                }
                if col < line.len() - 1 {
                    if lines[row - 1][col + 1] == '@' {
                        occupied_sides += 1;
                    }
                }
            }
            if col > 0 {
                if lines[row][col - 1] == '@' {
                    occupied_sides += 1;
                }
            }
            if col < line.len() - 1 {
                if lines[row][col + 1] == '@' {
                    occupied_sides += 1;
                }
            }
            if row < lines.len() - 1 {
                if col > 0 {
                    if lines[row + 1][col - 1] == '@' {
                        occupied_sides += 1;
                    }
                }
                if lines[row + 1][col] == '@' {
                    occupied_sides += 1;
                }
                if col < line.len() - 1 {
                    if lines[row + 1][col + 1] == '@' {
                        occupied_sides += 1;
                    }
                }
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

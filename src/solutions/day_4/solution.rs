use std::env::join_paths;

use crate::utils::runner::Runner;

pub struct D4 {}

impl D4 {
    fn search_row(&self, row: String) -> usize {
        let chars = row.chars();
        let s_chars = row.chars().collect::<Vec<char>>(); // create a static index to check patterns
        let row_len = row.len() - 1;
        let mut count: usize = 0;

        for (i, char) in chars.enumerate() {
            if row_len - i < 3 {
                break;
            }

            if char == 'X' || char == 'S' {
                let combo = format!(
                    "{}{}{}{}",
                    char,
                    s_chars[i + 1],
                    s_chars[i + 2],
                    s_chars[i + 3]
                );

                if combo == "XMAS" || combo == "SAMX" {
                    count += 1;
                }
            }
        }

        count
    }

    // essentially we're going to take each column of the input
    // and turn it into a "row" (just a combined string) which lets us reuse our row logic
    fn split_columns_into_rows(&self, input: &String) -> Vec<String> {
        let lines = input.split_terminator("\n").collect::<Vec<&str>>();
        let columns = input.split_terminator("\n").next().unwrap().len();

        // if we have 4 columns, we need to do 4 loops
        // then in each of those loops, we need to grab the nth element of each line
        // loop 1 we grab lines[0][0], lines[0][1], etc..

        let mut cols = Vec::<String>::new();

        for i in 0..columns {
            let mut column = "".to_string();
            for g in 0..lines.len() {
                // so for example if we're grabbing the 3rd column, we will get the third element in each row
                column = format!("{}{}", column, lines[g].chars().nth(i).unwrap());
            }

            cols.push(column)
        }

        cols
    }

    fn search_diagonally(&self, input: &String) -> usize {
        let rows = input.split_terminator("\n").map(|s| s.to_string());
        let s_rows = rows.clone().collect::<Vec<String>>();

        let num_rows = s_rows.len() - 1;
        let mut count: usize = 0;

        for (row_index, row) in rows.enumerate() {
            let chars = row.chars();

            if row_index + 3 > num_rows {
                return count;
            }

            for (char_index, char) in chars.enumerate() {
                if char == 'X' || char == 'S' {
                    let down_right = format!(
                        "{}{}{}{}",
                        char,
                        s_rows[row_index + 1]
                            .chars()
                            .nth(char_index + 1)
                            .unwrap_or('?'),
                        s_rows[row_index + 2]
                            .chars()
                            .nth(char_index + 2)
                            .unwrap_or('?'),
                        s_rows[row_index + 3]
                            .chars()
                            .nth(char_index + 3)
                            .unwrap_or('?')
                    );

                    // make sure we have enough room to go left
                    if char_index >= 3 {
                        let down_left = format!(
                            "{}{}{}{}",
                            char,
                            s_rows[row_index + 1]
                                .chars()
                                .nth(char_index - 1)
                                .unwrap_or('?'),
                            s_rows[row_index + 2]
                                .chars()
                                .nth(char_index - 2)
                                .unwrap_or('?'),
                            s_rows[row_index + 3]
                                .chars()
                                .nth(char_index - 3)
                                .unwrap_or('?')
                        );
                        if down_left == "XMAS" || down_left == "SAMX" {
                            count += 1;
                        }
                    }

                    if down_right == "XMAS" || down_right == "SAMX" {
                        count += 1;
                    }
                }
            }
        }
        count
    }
}

impl Runner for D4 {
    fn name(&self) -> (usize, usize) {
        (2024, 4)
    }

    // Problem:
    // In this problem we have a crossword puzzle where we need to search for "XMAS"
    // However, it can be horizontal, vertical, or diagonal. Forwards or backwards.

    // Solution:
    // We're going to do 3 full searches over the input
    // For the first one, we'll search rows for XMAS or MASX
    // For the second one, we'll search columns for the same
    // Diagonal is the sort of complicated one.
    // We have to get the length of the rows
    // Then we should go row by row, and when we encounter an M or an S in a row, we have to check the following 3 rows
    // If we an encounter an X in row 1, we check row 2 at i+1 for M, row 3 at i+2 for A, and row 4 at i+3 for S
    // We also have to check i-1, i-2, and i-3 for a opposite diagonal
    // This holds true for finding an S, just the opposite
    // There are some bounds we have to check though:
    // If the M or S is encountered within the last 3 positions of the row, a down->right diagonal isnt possible
    // If the M/S is encountered in the first 3 positions of the row, a down->left diagonal isnt possible
    // If the M/S is encountered in the last 3 columns of the input, neither is possible

    fn part_one(&self) -> Option<usize> {
        let input = self.load_input(false);

        let mut count: usize = 0;

        let rows = input.split_terminator("\n");

        for row in rows {
            let res = self.search_row(row.to_string());
            count += res;
        }

        let columns = self.split_columns_into_rows(&input);

        for col in columns {
            let res = self.search_row(col.clone());
            count += res;
        }

        let diag = self.search_diagonally(&input);
        count += diag;
        Some(count)
    }

    // ok so we now have to search for occurances where there are two overlapping MAS/SAM
    // the first thing that comes to my brain is search for occurances of A, and then check the 4 characters surrounding it
    // i.e. if A is on line 3 position 5
    // line 2 position 4 should be M or S
    // line 4 position 6 M or S
    // line 2 position 6 should be M or S
    // line 4 position 4 should be M or S

    // so we'll find the A (on say l3p5)
    // and we'll make two strings:
    // l2p4+l3p5+l4p6
    // l2p6+l3p5+l4p4
    // if they both are MAS or SAM, we increment the counter
    fn part_two(&self) -> Option<usize> {
        let input = self.load_input(false);

        let mut count: usize = 0;

        let rows = input.split_terminator("\n").map(|s| s.to_string());
        let s_rows = rows.clone().collect::<Vec<String>>();

        for (row_index, row) in rows.enumerate() {
            let chars = row.chars();

            for (char_index, char) in chars.enumerate() {
                if char == 'A' {
                    // out of bounds checks
                    // we need the top row to exist
                    // we need the bottom row to exist
                    // and char_index needs to be more than 0 and less than len?

                    if row_index == 0 || row_index == s_rows.len() - 1 {
                        continue;
                    }

                    if char_index == 0 || char_index == row.len() - 1 {
                        continue;
                    }

                    let top_left = s_rows[row_index - 1]
                        .chars()
                        .nth(char_index - 1)
                        .unwrap_or('?');
                    let bottom_right = s_rows[row_index + 1]
                        .chars()
                        .nth(char_index + 1)
                        .unwrap_or('?');
                    let left_to_right = format!("{}{}{}", top_left, char, bottom_right);

                    let top_right = s_rows[row_index - 1]
                        .chars()
                        .nth(char_index + 1)
                        .unwrap_or('?');
                    let bottom_left = s_rows[row_index + 1]
                        .chars()
                        .nth(char_index - 1)
                        .unwrap_or('?');
                    let right_to_left = format!("{}{}{}", top_right, char, bottom_left);

                    if (left_to_right == "MAS" || left_to_right == "SAM")
                        && (right_to_left == "MAS" || right_to_left == "SAM")
                    {
                        count += 1;
                    }
                }
            }
        }
        Some(count)
    }
}

use crate::utils::runner::Runner;

pub struct D3;

impl Runner for D3 {
    fn name(&self) -> (usize, usize) {
        (2024, 3)
    }

    fn part_one(&self) -> Option<usize> {
        let input = self.load_input(false);

        let chars = input.chars().collect::<Vec<char>>();
        let mut total: i32 = 0;

        for i in 0..chars.len() {
            if i + 3 >= chars.len() {
                break;
            }

            let potential_mul = format!(
                "{}{}{}{}",
                chars[i],
                chars[i + 1],
                chars[i + 2],
                chars[i + 3]
            );
            if potential_mul == "mul(" {
                let mut track = i + 4;
                let mut first_number: String = "".to_string();
                let mut second_number: String = "".to_string();

                while track < chars.len() && chars[track].is_digit(10) {
                    first_number = format!("{}{}", first_number, chars[track]);
                    track += 1;
                }

                if track >= chars.len() || chars[track] != ',' {
                    continue;
                }

                track += 1;

                while track < chars.len() && chars[track].is_digit(10) {
                    second_number = format!("{}{}", second_number, chars[track]);
                    track += 1;
                }

                if track >= chars.len() || chars[track] != ')' {
                    continue;
                }

                let first = first_number.parse::<i32>().unwrap();
                let second = second_number.parse::<i32>().unwrap();
                let sum = first * second;
                total += sum;
            }
        }

        Some(total as usize)
    }

    fn part_two(&self) -> Option<usize> {
        let input = self.load_input(false);

        let chars = input.chars().collect::<Vec<char>>();
        let mut do_multiply = true;
        let mut total: i32 = 0;

        for i in 0..chars.len() {
            if i + 3 >= chars.len() {
                break;
            }

            let potential_mul_or_do = format!(
                "{}{}{}{}",
                chars[i],
                chars[i + 1],
                chars[i + 2],
                chars[i + 3]
            );

            if do_multiply {
                if potential_mul_or_do == "mul(" {
                    let mut track = i + 4;
                    let mut first_number: String = "".to_string();
                    let mut second_number: String = "".to_string();

                    while track < chars.len() && chars[track].is_digit(10) {
                        first_number = format!("{}{}", first_number, chars[track]);
                        track += 1;
                    }

                    if track >= chars.len() || chars[track] != ',' {
                        continue;
                    }

                    track += 1;

                    while track < chars.len() && chars[track].is_digit(10) {
                        second_number = format!("{}{}", second_number, chars[track]);
                        track += 1;
                    }

                    if track >= chars.len() || chars[track] != ')' {
                        continue;
                    }

                    let first = first_number.parse::<i32>().unwrap();
                    let second = second_number.parse::<i32>().unwrap();
                    let sum = first * second;
                    total += sum;
                } else {
                    if i + 6 <= chars.len() - 1 {
                        let potential_dont = format!(
                            "{}{}{}{}{}{}{}",
                            chars[i],
                            chars[i + 1],
                            chars[i + 2],
                            chars[i + 3],
                            chars[i + 4],
                            chars[i + 5],
                            chars[i + 6]
                        );
                        if potential_dont == "don't()" {
                            do_multiply = false;
                        }
                    }
                }
            } else {
                if potential_mul_or_do == "do()" {
                    do_multiply = true;
                }
            }
        }

        Some(total as usize)
    }
}

use crate::utils::runner::Runner;

pub struct D2 {}

enum Operation {
    Unknown,
    Increasing,
    Decreasing,
}

impl D2 {
    fn eval_report(r: &Vec<i32>) -> (bool, Option<usize>) {
        let len = r.len();

        let mut operation = Operation::Unknown;

        for i in 0..len - 1 {
            let left = r[i];
            let right = r[i + 1];

            if left == right {
                return (false, Some(i));
            }

            let res = left - right;

            match operation {
                Operation::Unknown => {
                    if res > 0 {
                        operation = Operation::Decreasing;
                        let min = left - 3;
                        let max = left - 1;

                        if right < min || right > max {
                            return (false, Some(i));
                        }
                    } else {
                        operation = Operation::Increasing;
                        let min = left + 1;
                        let max = left + 3;

                        if right < min || right > max {
                            return (false, Some(i));
                        }
                    }
                }
                Operation::Increasing => {
                    if res > 0 {
                        return (false, Some(i));
                    }

                    let min = left + 1;
                    let max = left + 3;

                    if right < min || right > max {
                        return (false, Some(i));
                    }
                }
                Operation::Decreasing => {
                    if res < 0 {
                        return (false, Some(i));
                    }

                    let min = left - 3;
                    let max = left - 1;

                    if right < min || right > max {
                        return (false, Some(i));
                    }
                }
            }
        }

        (true, None)
    }
}

impl Runner for D2 {
    fn name(&self) -> (usize, usize) {
        (2024, 2)
    }

    fn part_one(&self) -> Option<usize> {
        let input = self.load_input(false);
        let reports = input.split_terminator("\n");

        let mut safe_ctr: i32 = 0;

        'outer: for report in reports {
            let levels = report.split(" ").collect::<Vec<&str>>();

            let mut operation = Operation::Unknown;

            for window in levels.windows(2) {
                let left = window[0].parse::<i32>().unwrap();
                let right = window[1].parse::<i32>().unwrap();

                if left == right {
                    continue 'outer;
                }

                let res = left - right;

                match operation {
                    Operation::Unknown => {
                        if res > 0 {
                            operation = Operation::Decreasing;
                            let min = left - 3;
                            let max = left - 1;
                            if right < min || right > max {
                                continue 'outer;
                            }
                        } else {
                            operation = Operation::Increasing;
                            let min = left + 1;
                            let max = left + 3;
                            if right < min || right > max {
                                continue 'outer;
                            }
                        }
                    }
                    Operation::Decreasing => {
                        if res < 0 {
                            continue 'outer;
                        }

                        let min = left - 3;
                        let max = left - 1;
                        if right < min || right > max {
                            continue 'outer;
                        }
                    }
                    Operation::Increasing => {
                        if res > 0 {
                            continue 'outer;
                        }

                        let min = left + 1;
                        let max = left + 3;

                        if right < min || right > max {
                            continue 'outer;
                        }
                    }
                }
            }

            // if we are still in this loop, we can assume its safe
            safe_ctr += 1;
        }
        Some(safe_ctr as usize)
    }

    fn part_two(&self) -> Option<usize> {
        let input = self.load_input(false);

        let reports = input.split_terminator("\n");
        let mut safe_ctr: i32 = 0;

        'outer: for report in reports {
            let original_report = report
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            if Self::eval_report(&original_report).0 {
                safe_ctr += 1;
                continue;
            }

            for i in 0..original_report.len() {
                let mut modified_report = original_report.clone();
                modified_report.remove(i);

                if Self::eval_report(&modified_report).0 {
                    safe_ctr += 1;
                    continue 'outer;
                }
            }
        }
        Some(safe_ctr as usize)
    }
}

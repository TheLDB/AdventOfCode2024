// warning:
// this solution is incredibly messy and i am not proud of it lmao

pub struct DayTwo {}

enum InputType {
    Test,
    Real,
}

enum Operation {
    Unknown,
    Increasing,
    Decreasing,
}

impl DayTwo {
    fn get_input(which: InputType) -> String {
        match which {
            InputType::Test => std::fs::read_to_string("src/day_two/test_input.txt").unwrap(),
            InputType::Real => std::fs::read_to_string("src/day_two/input.txt").unwrap(),
        }
    }

    // so we need to identify the # of reports that are safe
    // per usual, we'll split the list into reports by \n
    // then we'll start going through the report and checking to make sure its safe
    // for our first check, we'll ensure the current number isnt the same as the next number, because it has to increase or decrease by min 1
    // then, we'll do first-second to figure out the operation.
    // if the number is positive, the number is decreasing
    // if the number is negative, the number is increasing
    // if this is the first loop, this operation is the status quo that other checks will be held to
    // if its not, we'll ensure the operations are consistent and mark unsafe if not
    // then, we need to check bounds
    // if we're decreasing, and our first number is 6, the next number has to be max 5 and minimum 3
    // if we're increasing and our first number is 6, the next number has to be max 9 and min 7
    // if all these pass, we can move to the next set
    //
    // so take 7 6 4 2 1
    // 7 and 6 are not the same
    // 7-6 is 1, so we're decreasing
    // first loop, so decreasing is now the status quo
    // second number needs to be max 6 and min 4, and it is, so we move to the next pairing
    //
    // 6 and 4 are not the same
    // 6-4 is 2, so we're still decreasing, checks out
    // second number needs to be max 5 and min 3, and it is
    //
    // 4 and 2 are not the same
    // 4-2 is 2, so we're still decreasing
    // second number needs to be max 3 and min 1, and it is
    //
    // 2 and 1 are not the same
    // 2-1 is 1, so we're still decreasig
    // second number needs to be max 1 and min 0 (idt we can have negatives), and it is
    //
    // theres no next number, so our report is marked safe by incrementing a counter
    pub fn part_one() {
        let input = Self::get_input(InputType::Real);

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

        println!("Num Safe Reports: {}", safe_ctr);
    }

    // ok so now we have this dampener thing where we can remove a max of 1 bad level from a report to turn it safe
    // so i need a way to like go through it, and then when theres a bad value, remove that value and retest the pairing?
    // i could maybe do it in like a traditional iterator not a window iter where its like
    // if !dampener_used eval_pairing(l, r, operation)

    // ok nvm im lowkey lost idk how im gonna do this i need to sleep on it icl
    
    fn eval_report(r: &Vec<i32>) -> (bool, Option<usize>) {
        let len = r.len();

        let mut operation = Operation::Unknown;

        for i in 0..len - 1 {
            let left = r[i];
            let right = r[i+1];

            if left == right {
                return (false, Some(i));
            }

            let res = left-right;

            match operation {
                Operation::Unknown => {
                    if res > 0 {
                        operation = Operation::Decreasing;
                        let min = left-3;
                        let max = left-1;

                        if right < min || right > max {
                            return (false, Some(i));
                        }
                    } else {
                        operation = Operation::Increasing;
                        let min = left+1;
                        let max = left+3;

                        if right < min || right > max {
                            return (false, Some(i));
                        }
                    }
                    
                }
                Operation::Increasing => {
                    if res > 0 {
                        return (false, Some(i));
                    }

                    let min = left+1;
                    let max = left+3;

                    if right < min || right > max {
                        return (false, Some(i));
                    }
                }
                Operation::Decreasing => {
                    if res < 0 {
                        return (false, Some(i));
                    }

                    let min = left-3;
                    let max = left-1;

                    if right < min || right > max {
                        return (false, Some(i));
                    }
                }
            }
        }

        (true, None)
    }

    pub fn part_two() {
        let input = Self::get_input(InputType::Real);
        let reports = input.split_terminator("\n");
        let mut safe_ctr: i32 = 0;
    
        'outer: for report in reports {
            let original_report = report.split_whitespace()
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
    
        println!("Num Safe Reports: {}", safe_ctr);
    }
}

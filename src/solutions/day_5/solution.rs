use crate::utils::runner::Runner;

pub struct D5 {}

impl D5 {
    // ((rule, rule), array of array of chars)
    fn break_input(&self, input: String) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
        let parts = input.split("\n\n").collect::<Vec<&str>>();

        let rules = parts[0]
            .to_string()
            .split_terminator("\n")
            .map(|r| {
                let rule = r
                    .split("|")
                    .map(|t| t.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                (rule[0], rule[1])
            })
            .collect::<Vec<(usize, usize)>>();

        let problems = parts[1]
            .split_terminator("\n")
            .map(|p| {
                p.split(",")
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>();

        (rules, problems)
    }

    fn compare_values(
        &self,
        values: &(usize, usize),
        rules: &Vec<(usize, usize)>,
    ) -> (usize, usize) {
        // Find a matching rule for the compared values
        // If found, rule.0 must be before rule.1
        // So for example if values is (10, 5) and the rule found is (5, 10)
        // We're going to realize that found.0 (5) does not equal values.0 (10), so we have to switch them around
        let cmp1 = values.0;
        let cmp2 = values.1;

        let find = rules
            .into_iter()
            .find(|r| (r.0 == cmp1 && r.1 == cmp2) || (r.0 == cmp2 && r.1 == cmp1));

        if let Some(found) = find {
            if found.0 == cmp1 && found.1 == cmp2 {
                (cmp1, cmp2)
            } else {
                (cmp2, cmp1)
            }
        } else {
            (cmp1, cmp2)
        }
    }
}

impl Runner for D5 {
    fn name(&self) -> (usize, usize) {
        (2024, 5)
    }

    // Ok so we have a list of numbers that need to abide by a custom sorting rule
    // i think a very simple and bruteforcy way of doing it is like a modified bubblesort
    // where we go through the array in an O(n^2) time complexity and compare each value
    // but instead of sorting, if a value is wrong we just continue on to the next loop
    //  and we store all the correctly sorted ones in an array
    fn part_one(&self) -> Option<usize> {
        let input = self.load_input(false);

        let (rules, updates) = self.break_input(input);

        let mut correct: Vec<Vec<usize>> = vec![];

        let mut sum: usize = 0;

        'update_loop: for update in updates {
            // For each update, we need to take the first value, and start comparing it to the other values
            //

            for (index, value) in update.clone().into_iter().enumerate() {
                for i in (index + 1)..update.len() {
                    let cmp = self.compare_values(&(value, update[i]), &rules);

                    if value != cmp.0 || update[i] != cmp.1 {
                        continue 'update_loop;
                    }
                }
            }

            let mid = update[update.len() / 2];
            sum += mid;
            correct.push(update);
        }

        Some(sum)
    }

    // ok so now we have to do the same, except we need to fix the not in order arrays, and only take the middle of those
    fn part_two(&self) -> Option<usize> {
        let input = self.load_input(false);

        let (rules, updates) = self.break_input(input);

        let mut fixed: Vec<Vec<usize>> = vec![];

        let mut sum: usize = 0;

        for update in &updates {
            let mut fixed_update = update.clone();
            let mut made_swap = true;

            while made_swap {
                made_swap = false;
                for index in 0..fixed_update.len() {
                    for i in (index + 1)..fixed_update.len() {
                        let cmp =
                            self.compare_values(&(fixed_update[index], fixed_update[i]), &rules);

                        // if the values sent are not the same ones received, we swap them and need to recheck the array
                        if fixed_update[index] != cmp.0 || fixed_update[i] != cmp.1 {
                            fixed_update.swap(index, i);
                            made_swap = true;
                        }
                    }
                }
            }

            if fixed_update != *update {
                let mid = fixed_update[fixed_update.len() / 2];
                sum += mid;
            }

            fixed.push(fixed_update);
        }

        Some(sum)
    }
}

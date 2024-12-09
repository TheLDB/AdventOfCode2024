use crate::utils::runner::Runner;

pub struct D7 {}

#[derive(Clone)]
struct InputPart {
    answer: i64,
    parts: Vec<i64>,
}

#[derive(Debug)]
enum Operation {
    Addition,
    Multiplication,
    Concat,
}

impl D7 {
    fn get_input_parts(&self, test: bool) -> Vec<InputPart> {
        let input = self.load_input(test);

        let lines = input.split_terminator("\n");

        let mut hl_parts: Vec<InputPart> = Vec::new();

        for line in lines {
            let parts = line.split(":").collect::<Vec<&str>>();

            let answer = parts[0].parse::<i64>().unwrap();
            let items = parts[1]
                .trim()
                .split(" ")
                .map(|v| v.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            hl_parts.push(InputPart {
                answer,
                parts: items,
            });
        }

        hl_parts
    }

    // so if we're on index 0 and total 0, it'll grab the first element, and add it to our 0 total
    // then, we break into different branches
    // we do item[0] + item[1] and item[0] * item[1]
    // then those results will break into their own branches
    // only terminating when we've ran through all the numbers
    fn search(
        problem: InputPart,
        index: i64,
        total: i64,
        operation: Operation,
        part_two: bool,
    ) -> bool {
        let (index, mut total) = (index, total);

        match operation {
            Operation::Addition => {
                total += problem.parts[index as usize];
            }
            Operation::Multiplication => {
                total *= problem.parts[index as usize];
            }
            Operation::Concat => {
                let mut digits = problem.parts[index as usize];
                let mut multiplier = 1;
                while digits > 0 {
                    multiplier *= 10;
                    digits /= 10;
                }

                total = total * multiplier + problem.parts[index as usize];
            }
        }

        if total > problem.answer {
            return false;
        }

        if (index + 1) as usize >= problem.parts.len() {
            return total == problem.answer;
        }

        let left = D7::search(
            problem.clone(),
            index + 1,
            total,
            Operation::Addition,
            part_two,
        );
        let right = D7::search(
            problem.clone(),
            index + 1,
            total,
            Operation::Multiplication,
            part_two,
        );

        if part_two {
            let middle = D7::search(
                problem.clone(),
                index + 1,
                total,
                Operation::Concat,
                part_two,
            );
            left || middle || right
        } else {
            left || right
        }
    }
}

impl Runner for D7 {
    fn name(&self) -> (usize, usize) {
        (2024, 7)
    }

    // Ok so we have a list of like
    // product: item item item
    // and we need to see if its possible to get the product by adding/multiplying the items
    // this does not follow pemdas, just l->r
    //
    // i think we could solve this with like a simple tree search where for each pairing we branch out and try each different combo until we've exhausted our options
    fn part_one(&self) -> Option<usize> {
        let input = self.get_input_parts(false);

        let mut total: i64 = 0;

        for problem in input {
            let solution = D7::search(problem.clone(), 0, 0, Operation::Addition, false);

            if solution {
                total += problem.answer
            }
        }

        Some(total as usize)
    }

    fn part_two(&self) -> Option<usize> {
        let input = self.get_input_parts(false);

        let mut total: i64 = 0;

        for problem in input {
            let solution = D7::search(problem.clone(), 0, 0, Operation::Addition, true);

            if solution {
                total += problem.answer
            }
        }

        Some(total as usize)
    }
}

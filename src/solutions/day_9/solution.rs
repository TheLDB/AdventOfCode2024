use crate::utils::runner::Runner;

pub struct D9 {}

impl D9 {}

impl Runner for D9 {
    fn name(&self) -> (usize, usize) {
        (2024, 9)
    }

    fn part_one(&self) -> Option<usize> {
        let input = self.load_input(false);

        None
    }

    fn part_two(&self) -> Option<usize> {
        None
    }
}

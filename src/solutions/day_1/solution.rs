use crate::utils::runner::Runner;
use std::{
    cmp::{max, min},
    collections::HashMap,
};

pub struct D1 {}

impl Runner for D1 {
    fn name(&self) -> (usize, usize) {
        (2024, 1)
    }

    fn part_one(&self) -> Option<usize> {
        let input = self.load_input(true);
        let mut left: Vec<i64> = vec![];
        let mut right: Vec<i64> = vec![];

        let mut answer: i64 = 0;

        let pairs = input.split_terminator("\n");

        for pair in pairs {
            let nums = pair.split("   ").collect::<Vec<&str>>();

            left.push(nums[0].parse::<i64>().unwrap());
            right.push(nums[1].parse::<i64>().unwrap());
        }

        left.sort();
        right.sort();

        for i in 0..left.len() {
            let left_val = left[i];
            let right_val = right[i];

            let bigger = max(left_val, right_val);
            let smaller = min(left_val, right_val);

            let diff = bigger - smaller;

            answer += diff;
        }

        Some(answer as usize)
    }

    fn part_two(&self) -> Option<usize> {
        let input = self.load_input(true);

        let mut left: Vec<i64> = Vec::new();
        let mut right: Vec<i64> = Vec::new();

        let mut similarity: i64 = 0;

        let pairs = input.split_terminator("\n");

        for pair in pairs {
            let nums = pair.split("   ").collect::<Vec<&str>>();

            left.push(nums[0].parse::<i64>().unwrap());
            right.push(nums[1].parse::<i64>().unwrap());
        }

        // mapping of value to # of occurances
        let mut occurance_map = HashMap::<i64, i64>::new();

        for item in right {
            *occurance_map.entry(item).or_insert(0) += 1;
        }

        for item in left {
            let occurances = occurance_map.get(&item).unwrap_or(&0);

            let score = item * occurances;

            similarity += score;
        }

        Some(similarity as usize)
    }
}

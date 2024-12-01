use std::{cmp::{max, min}, collections::HashMap};

pub struct DayOne {}

// We need to take the combined lists and split them into two diff i64 vectors
// then we need to sort the two vectors lowest -> highest
// then we need to start going the lists, comparing each number, finding the difference (based on high-low num), and adding it to a number we track

// i.e.
// 3   4
// 4   3
// 2   5
// 1   3
// 3   9
// 3   3

// gets sorted to

// 1 3 = 2
// 2 3 = 1
// 3 3 = 0
// 3 4 = 1
// 3 5 = 2
// 4 9 = 5
//     = 11 (which is correct)

impl DayOne {
    pub fn solution() {
        let input = std::fs::read_to_string("src/day_one/input.txt").unwrap();

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

        println!("Answer: {}", answer);
    }

    // ok so now we need to calculate the similarity score
    // for this we'll do the same splitting into l/r like before
    // but this time we'll go through the left list by value, rather than iterating by a range
    // when we encounter a value, we'll do a hashmap lookup for it
    // if it doesn't exist, we'll get the # of occurances in the right vec and then add it to our hashmap, and do the value * # of occurances and add it to our similarity scorr
    // if it does exist, we'll just multiply the value by # of occurances and up the similarity score

    // ok slight change i just kind of figured rust had like an easy occurances thing, alas they do not?
    // so im going to loop through the right array, add each item to a hashmap, and then use that
    // this will also help with the ones where the value doesnt appear in the right

    pub fn solution_p2() {
        let input = std::fs::read_to_string("src/day_one/input.txt").unwrap();

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

        println!("Similarity Score: {}", similarity);
    }
}

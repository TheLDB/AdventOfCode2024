use crate::{utils::read_input::InputType, utils::read_input::ReadInput, Days};

pub struct DayThree {}

impl DayThree {
    // Test Input: xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
    // We need to extract from our input only valid commands
    // Currently, the only valid command we have is mul(x,y)
    // So we need to parse the string and extract only the sequences where it is exactly
    // mul(i32,i32)
    // I think the ideal solution is prob regex, but I feel like thats sort of a cop out, so I want to find another way

    // Bruteforce Solution:
    // Create a loop for the length of the input
    // For each loop, check if str[i] + str[i+1] + str[i+2] + str[i+3] = "mul("
    // If it does:
    // Start a new loop that goes through until it hits a non number
    // If the non number is , -- start a new loop thhat goes through until it hits a non number
    // If the second non number is ), we have a full valid command
    // If the non number isnt , -- we just have to set i to the current value + 1 and move on
    // If it doesn't set the loop to i+4, and move on
    pub fn part_one() {
        let input = ReadInput::init(Days::Three, InputType::Real);

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

        println!("Final Sum: {}", total);
    }

    // Ok so now we have the same thing, except instructions are enabled/disabled at certain times
    // I believe we can actually reuse our existing logic for this
    // Such that we'll have potential_mul, potential_do, and potential_dont
    // potential do/dont will effect our variable we store to tell whether we should even check for mul()
    // if we're in dont mode, then we just wont even do the mul logic i think
    pub fn part_two() {
        let input = ReadInput::init(Days::Three, InputType::Real);

        // let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string();
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
                        let potential_dont = format!("{}{}{}{}{}{}{}", chars[i], chars[i+1], chars[i+2], chars[i+3], chars[i+4], chars[i+5], chars[i+6]);
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
        
        println!("Part Two: {}", total);
    }
}

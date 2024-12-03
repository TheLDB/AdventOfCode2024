mod utils;
use day_one::solution::DayOne;
use day_three::solution::DayThree;
use day_two::solution::DayTwo;
mod day_one;
mod day_two;
mod day_three;

pub enum Days {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
    Sixteen,
    Seventeen,
    Eighteen,
    Nineteen,
    Twenty,
    TwentyOne,
    TwentyTwo,
    TwentyThree,
    TwentyFour,
    TwentyFive,
    TwentySix,
    TwentySeven,
    TwentyEight,
    TwentyNine,
    Thirty,
    ThirtyOne,
}

impl std::fmt::Display for Days {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let day_str = match self {
            Days::One => "day_one",
            Days::Two => "day_two",
            Days::Three => "day_three",
            Days::Four => "day_four",
            Days::Five => "day_five",
            Days::Six => "day_six",
            Days::Seven => "day_seven",
            Days::Eight => "day_eight",
            Days::Nine => "day_nine",
            Days::Ten => "day_10",
            Days::Eleven => "day_11",
            Days::Twelve => "day_12",
            Days::Thirteen => "day_13",
            Days::Fourteen => "day_14",
            Days::Fifteen => "day_15",
            Days::Sixteen => "day_16",
            Days::Seventeen => "day_17",
            Days::Eighteen => "day_18",
            Days::Nineteen => "day_19",
            Days::Twenty => "day_20",
            Days::TwentyOne => "day_21",
            Days::TwentyTwo => "day_22",
            Days::TwentyThree => "day_23",
            Days::TwentyFour => "day_24",
            Days::TwentyFive => "day_25",
            Days::TwentySix => "day_26",
            Days::TwentySeven => "day_27",
            Days::TwentyEight => "day_28",
            Days::TwentyNine => "day_29",
            Days::Thirty => "day_30",
            Days::ThirtyOne => "day_31",
        };
        write!(f, "{}", day_str)
    }
}

fn main() {
    let day = Days::Three;

    match day {
        Days::One => {
            let _day_one_solution = DayOne::solution();
            let _day_one_solution_p2 = DayOne::solution_p2();
        }
        Days::Two => {
            DayTwo::part_one();
            DayTwo::part_two();
        }
        Days::Three => {
            DayThree::part_one();
            DayThree::part_two();
        }
        Days::Four => {}
        Days::Five => {}
        Days::Six => {}
        Days::Seven => {}
        Days::Eight => {}
        Days::Nine => {}
        Days::Ten => {}
        Days::Eleven => {}
        Days::Twelve => {}
        Days::Thirteen => {}
        Days::Fourteen => {}
        Days::Fifteen => {}
        Days::Sixteen => {}
        Days::Seventeen => {}
        Days::Eighteen => {}
        Days::Nineteen => {}
        Days::Twenty => {}
        Days::TwentyOne => {}
        Days::TwentyTwo => {}
        Days::TwentyThree => {}
        Days::TwentyFour => {}
        Days::TwentyFive => {}
        Days::TwentySix => {}
        Days::TwentySeven => {}
        Days::TwentyEight => {}
        Days::TwentyNine => {}
        Days::Thirty => {}
        Days::ThirtyOne => {}
    }
}

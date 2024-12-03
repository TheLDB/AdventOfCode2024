use day_one::solution::DayOne;
use day_two::solution::DayTwo;
mod day_one;
mod day_two;
enum Days {
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

fn main() {
    let day = Days::Two;

    match day {
        Days::One => {
            let _day_one_solution = DayOne::solution();
            let _day_one_solution_p2 = DayOne::solution_p2();
        }
        Days::Two => {
            DayTwo::part_one();
            DayTwo::part_two();
        }
        Days::Three => {}
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

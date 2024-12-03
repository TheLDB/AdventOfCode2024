use crate::Days;

pub enum InputType {
    Test,
    Real
}

pub struct ReadInput {}

impl ReadInput {
    pub fn init(day: Days, it: InputType) -> String {
        match it {
            InputType::Test => std::fs::read_to_string(format!("src/{}/test_input.txt", day)).unwrap(),
            InputType::Real => std::fs::read_to_string(format!("src/{}/input.txt", day)).unwrap()
        }
    }
}
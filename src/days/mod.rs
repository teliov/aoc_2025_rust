mod day1;

pub enum Day {
    One
}

impl Day {
    pub fn from(day: i32) -> Day {
        match day {
            1 => Day::One,
            _ => panic!("Day {} is not implemented", day),
        }
    }
    
    pub fn solve_part_one(&self, file_contents: &String) -> String {
        match self {
            Day::One => day1::solve_part_one(file_contents),
        }
    }

    pub fn solve_part_two(&self, file_contents: &String) -> String {
        match self {
            Day::One => day1::solve_part_two(file_contents),
        }
    }
}

impl std::fmt::Display for Day  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Day::One => write!(f, "Day One"),
        }
    }
}
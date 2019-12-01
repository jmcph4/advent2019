use std::fs;
use std::str::FromStr;

pub fn calculate_fuel_requirement(module_mass: u128) -> u128 {
    (module_mass / 3) - 2
}

pub fn solve_day1(input: String) -> u128 {
    let mut total_fuel_requirement: u128 = 0;

    for line in input.lines()  {
        let module_mass: u128 = u128::from_str(line).unwrap();
        total_fuel_requirement += calculate_fuel_requirement(module_mass);
    }

    total_fuel_requirement
}

fn main() {
    println!("{:?}", solve_day1(fs::read_to_string("input.txt").expect("Failed to read input file.")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_solve_day1_case1() {
        let input: String = "12".to_string();
        let actual_solution: u128 = solve_day1(input);
        let expected_solution: u128 = 2;

        assert_eq!(actual_solution, expected_solution);
    }

    #[test]
    pub fn test_solve_day1_case2() {
        let input: String = "14".to_string();
        let actual_solution: u128 = solve_day1(input);
        let expected_solution: u128 = 2;

        assert_eq!(actual_solution, expected_solution);
    }

    #[test]
    pub fn test_solve_day1_case3() {
        let input: String = "1969".to_string();
        let actual_solution: u128 = solve_day1(input);
        let expected_solution: u128 = 654;

        assert_eq!(actual_solution, expected_solution);
    }

    #[test]
    pub fn test_solve_day1_case4() {
        let input: String = "100756".to_string();
        let actual_solution: u128 = solve_day1(input);
        let expected_solution: u128 = 33583;

        assert_eq!(actual_solution, expected_solution);
    }
}


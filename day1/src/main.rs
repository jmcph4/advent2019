use std::fs;
use std::str::FromStr;

pub fn calculate_fuel_requirement(module_mass: u128) -> u128 {
    if module_mass <= 6 { 
        return 0;
    }

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

pub fn solve_day1_part2(input: String) -> u128 {
    let mut total_fuel_requirement: u128 = 0;

    for line in input.lines() {
        let module_mass: u128 = u128::from_str(line).unwrap();
        let mut module_fuel: u128 = calculate_fuel_requirement(module_mass);
        total_fuel_requirement += module_fuel;

        loop {
            println!("{:?}", module_fuel); // DEBUG
            let new_module_fuel = calculate_fuel_requirement(module_fuel);
            
            /* terminating condition */
            if new_module_fuel >= module_fuel || new_module_fuel == 0 {
                break
            }

            module_fuel = new_module_fuel;
            total_fuel_requirement += module_fuel;
        }
    }

    total_fuel_requirement
}

fn main() {
    println!("{:?}", solve_day1(fs::read_to_string("input.txt").expect("Failed to read input file.")));
    println!("{:?}", solve_day1_part2(fs::read_to_string("input.txt").expect("Failed to read input file.")));
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

    #[test]
    pub fn test_solve_day1_part2_case1() {
        let input: String = "14".to_string();
        let actual_solution: u128 = solve_day1_part2(input);
        let expected_solution: u128 = 2;

        assert_eq!(actual_solution, expected_solution);
    }

    #[test]
    pub fn test_solve_day1_part2_case2() {
        let input: String = "1969".to_string();
        let actual_solution: u128 = solve_day1_part2(input);
        let expected_solution: u128 = 966;

        assert_eq!(actual_solution, expected_solution);
    }

    #[test]
    pub fn test_solve_day1_part2_case3() {
        let input: String = "100756".to_string();
        let actual_solution: u128 = solve_day1_part2(input);
        let expected_solution: u128 = 50346;

        assert_eq!(actual_solution, expected_solution);
    }
}


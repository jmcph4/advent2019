use std::fs;
use std::str::FromStr;

pub type Opcode = u128;
pub type Program = Vec<Opcode>;

pub fn run_intcode_program(program: Program) -> Program {
    let mut opcodes: Program = program.clone();

    for i in (0..opcodes.len()).step_by(4) {
        let opcode: Opcode = opcodes[i]; 
        
        match opcode {
            1 => {
                let a: usize = opcodes[i+1] as usize;
                let b: usize = opcodes[i+2] as usize;
                let write_addr: usize = opcodes[i+3] as usize;

                opcodes[write_addr] = opcodes[a] + opcodes[b];
            },
            2 => {
                let a: usize = opcodes[i+1] as usize;
                let b: usize = opcodes[i+2] as usize;
                let write_addr: usize = opcodes[i+3] as usize;

                opcodes[write_addr] = opcodes[a] * opcodes[b];
            },
            _ => {}
        }
    }

    opcodes.clone()
}

pub fn get_noun_verb_pair(n: usize) -> (u8, u8) {
    ((n / 99) as u8, (n % 100) as u8)
}

pub fn solve_day2(input: String) -> Opcode {
    let mut opcodes: Program = Vec::new();

    for elem in input.split(",") {
        opcodes.push(Opcode::from_str(elem.trim()).unwrap());
    }

    // modify input as per problem specficiation
    opcodes[1] = 12;
    opcodes[2] = 2;

    let program_result: Program = run_intcode_program(opcodes);

    program_result[0]
}

pub fn solve_day2_part2(input: String) -> u128 {
    let mut i: usize = 0;

    loop {
        let mut opcodes: Program = Vec::new();

        for elem in input.split(",") {
            opcodes.push(Opcode::from_str(elem.trim()).unwrap());
        }

        let (noun, verb): (u8, u8) = get_noun_verb_pair(i);

        // modify input as per problem specification
        opcodes[1] = noun.into();
        opcodes[2] = verb.into();

        let program_result: Program = run_intcode_program(opcodes);

        if program_result[0] == 19690720 {
            return (100 * noun as u128 + verb as u128) as u128;
        }

        i += 1;
    }
}

fn main() {
    println!("{:?}", solve_day2(fs::read_to_string("input.txt").expect("Failed to read input file.")));
    println!("{:?}", solve_day2_part2(fs::read_to_string("input.txt").expect("Failed to read input file.")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_run_intcode_program_case1() {
        let input: Program = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let actual_solution: Program = run_intcode_program(input);
        let expected_solution: Program = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];

        assert_eq!(actual_solution, expected_solution);
    }
    
    #[test]
    pub fn test_run_intcode_program_case2() {
        let input: Program = vec![1, 0, 0, 0, 99];
        let actual_solution: Program = run_intcode_program(input);
        let expected_solution: Program = vec![2, 0, 0, 0, 99];

        assert_eq!(actual_solution, expected_solution);
    }

    #[test]
    pub fn test_run_intcode_program_case3() {
        let input: Program = vec![1, 0, 0, 0, 99];
        let actual_solution: Program = run_intcode_program(input);
        let expected_solution: Program = vec![2, 0, 0, 0, 99];

        assert_eq!(actual_solution, expected_solution);
    }

    #[test]
    pub fn test_run_intcode_program_case4() {
        let input: Program = vec![2, 4, 4, 5, 99, 0];
        let actual_solution: Program = run_intcode_program(input);
        let expected_solution: Program = vec![2, 4, 4, 5, 99, 9801];

        assert_eq!(actual_solution, expected_solution);
    }

    #[test]
    pub fn test_run_intcode_program_case5() {
        let input: Program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let actual_solution: Program = run_intcode_program(input);
        let expected_solution: Program = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];

        assert_eq!(actual_solution, expected_solution);
    }
}


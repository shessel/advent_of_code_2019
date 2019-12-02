fn intcode(mut program: Vec<i32>) -> Vec<i32> {
    let mut pc: usize = 0;
    while pc < program.len() {
        match program[pc] {
            1 => {
                let src0 = program[pc + 1] as usize;
                let src1 = program[pc + 2] as usize;
                let dst = program[pc + 3] as usize;
                program[dst] = program[src0] + program[src1]
            }
            2 => {
                let src0 = program[pc + 1] as usize;
                let src1 = program[pc + 2] as usize;
                let dst = program[pc + 3] as usize;
                program[dst] = program[src0] * program[src1]
            }
            99 => break,
            _ => break, // error case
        };
        pc += 4;
    }
    program
}

fn intcode_find_input(memory: Vec<i32>) -> (i32, i32) {
    let mut noun = 12;
    let mut verb = 2;
    loop {
        let mut memory = memory.clone();
        memory[1] = noun;
        memory[2] = verb;
        let result = intcode(memory)[0];
        if result == 19690720 {
            break (noun, verb);
        } else if result < 19690720 {
            noun += 1;
        } else {
            verb += 1;
            noun = 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::*;

    #[test]
    fn test_intcode_examples() {
        assert_eq!(intcode(vec![1, 0, 0, 0, 99]), vec![2, 0, 0, 0, 99]);
        assert_eq!(intcode(vec![2, 3, 0, 3, 99]), vec![2, 3, 0, 6, 99]);
        assert_eq!(intcode(vec![2, 4, 4, 5, 99, 0]), vec![2, 4, 4, 5, 99, 9801]);
        assert_eq!(
            intcode(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]),
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
    }

    #[test]
    fn test_intcode_aoc_input() {
        let mut input = parse_input_file_separator("data/input02", ",");
        input[1] = 12;
        input[2] = 2;
        assert_eq!(intcode(input)[0], 3716293);
    }

    #[test]
    fn test_intcode_find_input() {
        let input = parse_input_file_separator("data/input02", ",");
        assert_eq!(intcode_find_input(input), (64, 29));
    }
}

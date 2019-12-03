type INTCODE = usize;

enum INSTRUCTION {
    ADD(INTCODE, INTCODE, INTCODE),
    MULTIPLY(INTCODE, INTCODE, INTCODE),
    HALT,
}

fn intcode_program_evaluator(mut input: Vec<INTCODE>) -> INTCODE {
    let mut current_location = 0;
    loop {
        let opcode = *input.get(current_location).unwrap();
        let instruction = match opcode {
            1 => INSTRUCTION::ADD(
                *input.get(current_location + 1).unwrap(),
                *input.get(current_location + 2).unwrap(),
                *input.get(current_location + 3).unwrap(),
            ),
            2 => INSTRUCTION::MULTIPLY(
                *input.get(current_location + 1).unwrap(),
                *input.get(current_location + 2).unwrap(),
                *input.get(current_location + 3).unwrap(),
            ),
            99 => INSTRUCTION::HALT,
            _ => INSTRUCTION::HALT
        };

        let first_element: Option<INTCODE> = match instruction {
            INSTRUCTION::ADD(first_idx, second_idx, third_idx) => {
                input[third_idx] = input[first_idx] + input[second_idx];
                None
            },
            INSTRUCTION::MULTIPLY(first_idx, second_idx, third_idx) => {
                input[third_idx] = input[first_idx] * input[second_idx];
                None
            },
            INSTRUCTION::HALT => { Some(*input.get(0).unwrap()) },
        };
        if first_element.is_some() {
            return first_element.unwrap();
        } else {
            current_location += 4;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_for_simple_inputs() {
        assert_eq!(intcode_program_evaluator(simple_data()), 3500)
    }

    fn simple_data() -> Vec<INTCODE> {
        vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]
    }

    #[test]
    fn it_works_for_second_inputs() {
        assert_eq!(intcode_program_evaluator(second_simple_data()), 2)
    }

    fn second_simple_data() -> Vec<INTCODE> {
        vec![1,0,0,0,99]
    }

    #[test]
    fn it_works_for_third_inputs() {
        assert_eq!(intcode_program_evaluator(third_simple_data()), 2)
    }

    fn third_simple_data() -> Vec<INTCODE> {
        vec![2,3,0,3,99]
    }

    #[test]
    fn it_works_for_fourth_inputs() {
        assert_eq!(intcode_program_evaluator(fourth_simple_data()), 2)
    }

    fn fourth_simple_data() -> Vec<INTCODE> {
        vec![2,4,4,5,99,0]
    }

    #[test]
    fn it_works_for_fifth_inputs() {
        assert_eq!(intcode_program_evaluator(fifth_simple_data()), 30)
    }

    fn fifth_simple_data() -> Vec<INTCODE> {
        vec![1,1,1,4,99,5,6,0,99]
    }

    #[test]
    fn it_works() {
        assert_eq!(intcode_program_evaluator(data()), 7210630)
    }

    fn data() -> Vec<INTCODE> {
        vec![1,12,2,3,1,1,2,3,1,3,4,3,1,5,0,3,2,13,1,19,1,19,10,23,1,23,13,27,1,6,27,31,1,9,31,35,2,10,35,39,1,39,6,43,1,6,43,47,2,13,47,51,1,51,6,55,2,6,55,59,2,59,6,63,2,63,13,67,1,5,67,71,2,9,71,75,1,5,75,79,1,5,79,83,1,83,6,87,1,87,6,91,1,91,5,95,2,10,95,99,1,5,99,103,1,10,103,107,1,107,9,111,2,111,10,115,1,115,9,119,1,13,119,123,1,123,9,127,1,5,127,131,2,13,131,135,1,9,135,139,1,2,139,143,1,13,143,0,99,2,0,14,0]
    }
}
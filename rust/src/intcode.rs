enum Mode {
    Position,
    Immediate,
}

fn next_arg_mode<'a, T>(modes: &mut T) -> Mode
where
    T: Iterator<Item = &'a u8>,
{
    match modes.next() {
        None | Some(b'0') => Mode::Position,
        Some(_) => Mode::Immediate,
    }
}

fn get(program: &Vec<i32>, addr: usize, mode: Mode) -> &i32 {
    let val = program.get(addr).expect("invalid pc");
    match mode {
        Mode::Immediate => val,
        Mode::Position => program.get(*val as usize).expect("invalid pc"),
    }
}

fn op_3<'a, F, M>(program: &mut Vec<i32>, pc: usize, modes: &mut M, f: F) -> usize
where
    F: Fn(&i32, &i32) -> i32,
    M: Iterator<Item = &'a u8>,
{
    let arg_1_mode = next_arg_mode(modes);
    let arg_2_mode = next_arg_mode(modes);
    let arg_1 = get(program, pc + 1, arg_1_mode);
    let arg_2 = get(program, pc + 2, arg_2_mode);
    let &result_addr = get(program, pc + 3, Mode::Immediate);
    program[result_addr as usize] = f(arg_1, arg_2);
    pc + 4
}

fn jump_if<'a, F, M>(program: &mut Vec<i32>, pc: usize, modes: &mut M, f: F) -> usize
where
    F: Fn(&i32) -> bool,
    M: Iterator<Item = &'a u8>,
{
    let arg1_mode = next_arg_mode(modes);
    let arg2_mode = next_arg_mode(modes);
    let arg1 = get(program, pc + 1, arg1_mode);
    let pc = if f(arg1) {
        *get(program, pc + 2, arg2_mode) as usize
    } else {
        pc + 3
    };
    pc
}

fn step(program: &mut Vec<i32>, pc: usize, input: i32) -> (Option<usize>, Option<i32>) {
    let instruction = program.get(pc).expect("invalid pc");
    let s: String = instruction.to_string();
    let (modes, opcode) = if s.len() < 2 {
        ("", &s[..])
    } else {
        s.split_at(s.len() - 2)
    };
    let mut modes = modes.as_bytes().iter().rev();
    match opcode.parse::<u8>() {
        Ok(99) =>
        // terminate
        {
            (None, None)
        }
        Ok(1) =>
        // add
        {
            let pc = op_3(program, pc, &mut modes, |x, y| x + y);
            (Some(pc), None)
        }
        Ok(2) =>
        // mul
        {
            let pc = op_3(program, pc, &mut modes, |x, y| x * y);
            (Some(pc), None)
        }
        Ok(3) =>
        // store input
        {
            let &addr = get(program, pc + 1, Mode::Immediate);
            program[addr as usize] = input;
            (Some(pc + 2), None)
        }
        Ok(4) =>
        // output
        {
            let mode = next_arg_mode(&mut modes);
            let &addr = get(program, pc + 1, mode);
            (Some(pc + 2), Some(addr))
        }
        Ok(5) =>
        // jump-if-true
        {
            let pc = jump_if(program, pc, &mut modes, |&x| x != 0);
            (Some(pc), None)
        }
        Ok(6) =>
        // jump-if-false
        {
            let pc = jump_if(program, pc, &mut modes, |&x| x == 0);
            (Some(pc), None)
        }
        Ok(7) =>
        // less-than
        {
            let pc = op_3(program, pc, &mut modes, |x, y| if x < y { 1 } else { 0 });
            (Some(pc), None)
        }
        Ok(8) =>
        // equals
        {
            let pc = op_3(program, pc, &mut modes, |x, y| if x == y { 1 } else { 0 });
            (Some(pc), None)
        }
        _ => panic!("invalid instruction"),
    }
}

pub fn run(program: &mut Vec<i32>, input: i32) -> Option<i32> {
    let mut pc = Some(0);
    let mut output = None;
    loop {
        match pc {
            None => return output,
            Some(pc_) => {
                let (pc_, step_output) = step(program, pc_, input);
                pc = pc_;
                match step_output {
                    Some(_) => output = step_output,
                    None => (),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod day_02 {
        use super::*;

        #[test]
        fn test_step() {
            let mut program = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
            let input = 1;

            let (pc, _) = step(&mut program, 0, input);
            assert_eq!(program, vec![1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
            assert_eq!(pc, Some(4));

            let (pc, _) = step(&mut program, 4, input);
            assert_eq!(program, vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
            assert_eq!(pc, Some(8));

            let (pc, _) = step(&mut program, 8, input);
            assert_eq!(program, vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
            assert_eq!(pc, None);
        }

        #[test]
        fn test_run() {
            let input = 1;

            let mut program = vec![1, 0, 0, 0, 99];
            let _output = run(&mut program, input);
            assert_eq!(program, vec![2, 0, 0, 0, 99]);

            let mut program = vec![2, 3, 0, 3, 99];
            let _output = run(&mut program, input);
            assert_eq!(program, vec![2, 3, 0, 6, 99]);

            let mut program = vec![2, 4, 4, 5, 99, 0];
            let _output = run(&mut program, input);
            assert_eq!(program, vec![2, 4, 4, 5, 99, 9801]);

            let mut program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
            let _output = run(&mut program, input);
            assert_eq!(program, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
        }
    }

    mod day_05 {
        use super::*;

        #[test]
        fn test_run1() {
            let mut program = vec![3, 0, 4, 0, 99];

            let input = 1;
            let output = run(&mut program, input);
            assert_eq!(output, Some(input));
        }

        #[test]
        fn test_run2() {
            let mut program = vec![1002, 4, 3, 4, 33];

            let input = 1;
            let _ = run(&mut program, input);
            assert_eq!(program[4], 99);
        }

        #[test]
        fn test_run_neg() {
            let mut program = vec![1101, 100, -1, 4, 0];

            let input = 1;
            let _ = run(&mut program, input);
            assert_eq!(program[4], 99);
        }

        // Day 5 part 2 tests

        #[test]
        fn test_equal_position_mode() {
            // outputs 1 if input == 8
            let program = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];

            let output = run(&mut program.clone(), 8);
            assert_eq!(output, Some(1));

            let output = run(&mut program.clone(), 9);
            assert_eq!(output, Some(0));
        }

        #[test]
        fn test_less_than_position_mode() {
            // outputs 1 if input < 8
            let program = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];

            let output = run(&mut program.clone(), 7);
            assert_eq!(output, Some(1));

            let output = run(&mut program.clone(), 8);
            assert_eq!(output, Some(0));
        }

        #[test]
        fn test_equal_immediate_mode() {
            // outputs 1 if input == 8
            let program = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];

            let output = run(&mut program.clone(), 8);
            assert_eq!(output, Some(1));

            let output = run(&mut program.clone(), 9);
            assert_eq!(output, Some(0));
        }

        #[test]
        fn test_less_than_immediate_mode() {
            // outputs 1 if input < 8
            let program = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];

            let output = run(&mut program.clone(), 7);
            assert_eq!(output, Some(1));

            let output = run(&mut program.clone(), 8);
            assert_eq!(output, Some(0));
        }

        #[test]
        fn test_jump_position_mode() {
            // ouputs 1 if input != 0
            let program = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];

            let output = run(&mut program.clone(), 10);
            assert_eq!(output, Some(1));

            let output = run(&mut program.clone(), 0);
            assert_eq!(output, Some(0));
        }

        #[test]
        fn test_jump_immediate_mode() {
            // ouputs 1 if input != 0
            let program = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];

            let output = run(&mut program.clone(), 10);
            assert_eq!(output, Some(1));

            let output = run(&mut program.clone(), 0);
            assert_eq!(output, Some(0));
        }

        #[test]
        fn test_large() {
            // case input:
            //   < 8 ->  999
            //  == 8 -> 1000
            //   > 8 -> 1001
            let program = vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ];

            let output = run(&mut program.clone(), 7);
            assert_eq!(output, Some(999));

            let output = run(&mut program.clone(), 8);
            assert_eq!(output, Some(1000));
            let output = run(&mut program.clone(), 9);
            assert_eq!(output, Some(1001));
        }
    }
}

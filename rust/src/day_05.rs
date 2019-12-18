pub fn solve(part: u8, data: Result<String, std::io::Error>) {
    let mut program: Vec<i32> = data
        .expect("couldn't read data file")
        .split(",")
        .map(|s| s.parse::<i32>().expect("bad data"))
        .collect();

    match part {
        1 => {
            let output = part_1::run(&mut program, 1);
            println!("{:?}", output);
        }
        _ => (),
    }
}

pub mod part_1 {
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

    fn op_3<'a, F, M>(program: &mut Vec<i32>, pc: usize, modes: &mut M, f: F) -> Option<usize>
    where
        F: Fn(&i32, &i32) -> i32,
        M: Iterator<Item = &'a u8>,
    {
        let arg_1_mode = next_arg_mode(modes);
        let arg_2_mode = next_arg_mode(modes);
        let arg_1 = get(program, pc + 1, arg_1_mode);
        let arg_2 = get(program, pc + 2, arg_2_mode);
        let &result_addr = program.get(pc + 3).expect("invalid pc");
        program[result_addr as usize] = f(arg_1, arg_2);
        Some(pc + 4)
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
            Ok(99) => (None, None),
            Ok(1) => (op_3(program, pc, &mut modes, |x, y| x + y), None),
            Ok(2) => (op_3(program, pc, &mut modes, |x, y| x * y), None),
            Ok(3) => {
                let &addr = get(program, pc + 1, Mode::Immediate);
                program[addr as usize] = input;
                (Some(pc + 2), None)
            }
            Ok(4) => {
                let mode = next_arg_mode(&mut modes);
                let &addr = get(program, pc + 1, mode);
                (Some(pc + 2), Some(addr))
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
        #[test]
        fn test_run() {
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
    }
}

pub mod part_2 {}

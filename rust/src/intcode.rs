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

#[derive(Debug, PartialEq, Clone)]
pub enum State {
    Running,
    WaitForInput(usize),
    Output(i32),
    Terminated,
}

#[derive(Clone)]
pub struct Program {
    pc: usize,
    pub memory: Vec<i32>,
    pub state: State,
}

impl Program {
    pub fn new(memory: Vec<i32>) -> Self {
        Self {
            pc: 0,
            memory,
            state: State::Running,
        }
    }

    fn get(&self, addr: usize, mode: Mode) -> &i32 {
        let val = self.memory.get(addr).expect("invalid pc");
        match mode {
            Mode::Immediate => val,
            Mode::Position => self.memory.get(*val as usize).expect("invalid pc"),
        }
    }

    fn op_3<'a, F, M>(&mut self, modes: &mut M, f: F)
    where
        F: Fn(&i32, &i32) -> i32,
        M: Iterator<Item = &'a u8>,
    {
        let arg_1_mode = next_arg_mode(modes);
        let arg_2_mode = next_arg_mode(modes);
        let arg_1 = self.get(self.pc + 1, arg_1_mode);
        let arg_2 = self.get(self.pc + 2, arg_2_mode);
        let &result_addr = self.get(self.pc + 3, Mode::Immediate);
        self.memory[result_addr as usize] = f(arg_1, arg_2);
        self.pc += 4;
    }

    fn jump_if<'a, F, M>(&mut self, modes: &mut M, f: F)
    where
        F: Fn(&i32) -> bool,
        M: Iterator<Item = &'a u8>,
    {
        let arg1_mode = next_arg_mode(modes);
        let arg2_mode = next_arg_mode(modes);
        let arg1 = self.get(self.pc + 1, arg1_mode);
        self.pc = if f(arg1) {
            *self.get(self.pc + 2, arg2_mode) as usize
        } else {
            self.pc + 3
        }
    }

    pub fn step(&mut self) {
        let instruction = self.get(self.pc, Mode::Immediate);
        let s: String = instruction.to_string();
        let (modes, opcode) = if s.len() < 2 {
            ("", &s[..])
        } else {
            s.split_at(s.len() - 2)
        };
        let mut modes = modes.as_bytes().iter().rev();
        match opcode.parse::<u8>() {
            Ok(99) => self.state = State::Terminated,
            Ok(1) =>
            // add
            {
                self.op_3(&mut modes, |x, y| x + y);
            }
            Ok(2) =>
            // mul
            {
                self.op_3(&mut modes, |x, y| x * y);
            }
            Ok(3) =>
            // store input
            {
                let &addr = self.get(self.pc + 1, Mode::Immediate);
                self.state = State::WaitForInput(addr as usize);
                self.pc += 2;
                //     program[addr as usize] = input;
                //     State::Running(pc + 2)
            }
            Ok(4) =>
            // output
            {
                let mode = next_arg_mode(&mut modes);
                let &output = self.get(self.pc + 1, mode);
                self.state = State::Output(output);
                self.pc += 2;
            }
            Ok(5) =>
            // jump-if-true
            {
                self.jump_if(&mut modes, |&x| x != 0);
            }
            Ok(6) =>
            // jump-if-false
            {
                self.jump_if(&mut modes, |&x| x == 0);
            }
            Ok(7) =>
            // less-than
            {
                self.op_3(&mut modes, |x, y| if x < y { 1 } else { 0 });
            }
            Ok(8) =>
            // equals
            {
                self.op_3(&mut modes, |x, y| if x == y { 1 } else { 0 });
            }
            _ => panic!("invalid instruction"),
        }
    }

    pub fn run(&mut self, input: &mut impl Iterator<Item = i32>) -> Option<i32> {
        let mut output = None;
        loop {
            match self.state {
                State::Terminated => return output,
                State::Running => self.step(),
                State::Output(o) => {
                    output = Some(o);
                    self.state = State::Running;
                }
                State::WaitForInput(addr) => {
                    self.memory[addr] = input.next().expect("not enough input");
                    self.state = State::Running;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::repeat;

    mod day_02 {
        use super::*;

        #[test]
        fn test_step() {
            let mut program = Program::new(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);

            program.step();
            assert_eq!(program.memory, vec![1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
            assert_eq!(program.pc, 4);

            program.step();
            assert_eq!(program.memory, vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
            assert_eq!(program.pc, 8);

            program.step();
            assert_eq!(program.memory, vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
            assert_eq!(program.state, State::Terminated);
        }

        #[test]
        fn test_run() {
            let mut input = repeat(1);

            let mut program = Program::new(vec![1, 0, 0, 0, 99]);
            let _output = program.run(&mut input);
            assert_eq!(program.memory, vec![2, 0, 0, 0, 99]);

            let mut program = Program::new(vec![2, 3, 0, 3, 99]);
            let _output = program.run(&mut input);
            assert_eq!(program.memory, vec![2, 3, 0, 6, 99]);

            let mut program = Program::new(vec![2, 4, 4, 5, 99, 0]);
            let _output = program.run(&mut input);
            assert_eq!(program.memory, vec![2, 4, 4, 5, 99, 9801]);

            let mut program = Program::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
            let _output = program.run(&mut input);
            assert_eq!(program.memory, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
        }
    }

    mod day_05 {
        use super::*;

        #[test]
        fn test_run1() {
            let mut program = Program::new(vec![3, 0, 4, 0, 99]);

            let mut input = repeat(1);
            let output = program.run(&mut input);
            assert_eq!(output, Some(1));
        }

        #[test]
        fn test_run2() {
            let mut program = Program::new(vec![1002, 4, 3, 4, 33]);

            let mut input = repeat(1);
            let _ = program.run(&mut input);
            assert_eq!(program.memory[4], 99);
        }

        #[test]
        fn test_run_neg() {
            let mut program = Program::new(vec![1101, 100, -1, 4, 0]);

            let mut input = repeat(1);
            let _ = program.run(&mut input);
            assert_eq!(program.memory[4], 99);
        }

        // Day 5 part 2 tests

        #[test]
        fn test_equal_position_mode() {
            // outputs 1 if input == 8
            let program = Program::new(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);

            let output = program.clone().run(&mut repeat(8));
            assert_eq!(output, Some(1));

            let output = program.clone().run(&mut repeat(9));
            assert_eq!(output, Some(0));
        }

        #[test]
        fn test_less_than_position_mode() {
            // outputs 1 if input < 8
            let program = Program::new(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);

            let output = program.clone().run(&mut repeat(7));
            assert_eq!(output, Some(1));

            let output = program.clone().run(&mut repeat(8));
            assert_eq!(output, Some(0));
        }

        #[test]
        fn test_equal_immediate_mode() {
            // outputs 1 if input == 8
            let program = Program::new(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]);

            let output = program.clone().run(&mut repeat(8));
            assert_eq!(output, Some(1));

            let output = program.clone().run(&mut repeat(9));
            assert_eq!(output, Some(0));
        }

        #[test]
        fn test_less_than_immediate_mode() {
            // outputs 1 if input < 8
            let program = Program::new(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);

            let output = program.clone().run(&mut repeat(7));
            assert_eq!(output, Some(1));

            let output = program.clone().run(&mut repeat(8));
            assert_eq!(output, Some(0));
        }

        #[test]
        fn test_jump_position_mode() {
            // ouputs 1 if input != 0
            let program = Program::new(vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9]);

            let output = program.clone().run(&mut repeat(10));
            assert_eq!(output, Some(1));

            let output = program.clone().run(&mut repeat(0));
            assert_eq!(output, Some(0));
        }

        #[test]
        fn test_jump_immediate_mode() {
            // ouputs 1 if input != 0
            let program = Program::new(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);

            let output = program.clone().run(&mut repeat(10));
            assert_eq!(output, Some(1));

            let output = program.clone().run(&mut repeat(0));
            assert_eq!(output, Some(0));
        }

        #[test]
        fn test_large() {
            // case input:
            //   < 8 ->  999
            //  == 8 -> 1000
            //   > 8 -> 1001
            let program = Program::new(vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ]);

            let output = program.clone().run(&mut repeat(7));
            assert_eq!(output, Some(999));
            let output = program.clone().run(&mut repeat(8));
            assert_eq!(output, Some(1000));
            let output = program.clone().run(&mut repeat(9));
            assert_eq!(output, Some(1001));
        }
    }
}

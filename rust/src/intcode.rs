enum Mode {
    Position,
    Immediate,
    Relative,
}

fn next_arg_mode<'a, T>(modes: &mut T) -> Mode
where
    T: Iterator<Item = &'a u8>,
{
    match modes.next() {
        None | Some(b'0') => Mode::Position,
        Some(b'1') => Mode::Immediate,
        Some(_) => Mode::Relative,
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum State {
    Running,
    WaitForInput(usize),
    Output(i64),
    Terminated,
}

#[derive(Clone)]
pub struct Program {
    pc: usize,
    pub memory: Vec<i64>,
    pub state: State,
    relative_base: usize,
}

impl Program {
    pub fn new(memory: Vec<i64>) -> Self {
        Self {
            pc: 0,
            memory,
            state: State::Running,
            relative_base: 0,
        }
    }

    fn get(&mut self, addr: usize) -> i64 {
        if addr >= self.memory.len() {
            self.memory.resize(addr + 1, 0)
        }

        *self.memory.get(addr).expect("invalid pc")
    }

    fn get_addr(&mut self, addr: usize, mode: Mode) -> usize {
        match mode {
            Mode::Immediate => {
                addr
            }
            Mode::Position => {
                 self.get(addr) as usize
            }
            Mode::Relative => {
                let offset = self.get(addr);
                let addr1 = self.relative_base as i64 + offset;
                addr1 as usize
            }
        }
    }

    fn get_param(&mut self, addr: usize, mode: Mode) -> i64 {
        let addr = self.get_addr(addr, mode);
         self.get(addr)
    }

    fn set(&mut self, addr: usize, val: i64) {
        if addr >= self.memory.len() {
            self.memory.resize(addr + 1, 0)
        }

        self.memory[addr] = val
    }

    fn op_3<'a, F, M>(&mut self, modes: &mut M, f: F)
    where
        F: Fn(i64, i64) -> i64,
        M: Iterator<Item = &'a u8>,
    {
        let arg_1_mode = next_arg_mode(modes);
        let arg_2_mode = next_arg_mode(modes);
        let arg_3_mode = next_arg_mode(modes);
        let arg_1 = self.get_param(self.pc + 1, arg_1_mode);
        let arg_2 = self.get_param(self.pc + 2, arg_2_mode);
        let result_addr = self.get_addr(self.pc + 3, arg_3_mode);
        self.set(result_addr as usize, f(arg_1, arg_2));
        self.pc += 4;
    }

    fn jump_if<'a, F, M>(&mut self, modes: &mut M, f: F)
    where
        F: Fn(i64) -> bool,
        M: Iterator<Item = &'a u8>,
    {
        let arg1_mode = next_arg_mode(modes);
        let arg2_mode = next_arg_mode(modes);
        let arg1 = self.get_param(self.pc + 1, arg1_mode);
        self.pc = if f(arg1) {
            self.get_param(self.pc + 2, arg2_mode) as usize
        } else {
            self.pc + 3
        }
    }

    pub fn step(&mut self) {
        let instruction = self.get(self.pc);
        let s: String = instruction.to_string();
        let (modes, opcode) = if s.len() < 2 {
            ("", &s[..])
        } else {
            s.split_at(s.len() - 2)
        };
        let mut modes = modes.as_bytes().iter().rev();
        match opcode.parse::<u8>() {
            Ok(99) => {
                self.state = State::Terminated
            }
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
                let mode = next_arg_mode(&mut modes);
                let addr = self.get_addr(self.pc + 1, mode);
                self.state = State::WaitForInput(addr as usize);
                self.pc += 2;
            }
            Ok(4) =>
            // output
            {
                let mode = next_arg_mode(&mut modes);
                let output = self.get_param(self.pc + 1, mode);
                self.state = State::Output(output);
                self.pc += 2;
            }
            Ok(5) =>
            // jump-if-true
            {
                self.jump_if(&mut modes, |x| x != 0);
            }
            Ok(6) =>
            // jump-if-false
            {
                self.jump_if(&mut modes, |x| x == 0);
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
            Ok(9) =>
            // change relative base
            {
                let mode = next_arg_mode(&mut modes);
                let param = self.get_param(self.pc + 1, mode);
                self.relative_base = (self.relative_base as i64 + param) as usize;
                self.pc += 2;
            }
            _ => panic!("invalid instruction"),
        }
    }

    pub fn run(&mut self, input: &mut impl Iterator<Item = i64>) -> Vec<i64> {
        let mut output = Vec::new();
        loop {
            match self.state {
                State::Terminated => return output,
                State::Running => self.step(),
                State::Output(o) => {
                    output.push(o);
                    self.state = State::Running;
                }
                State::WaitForInput(addr) => {
                    self.set(addr, input.next().expect("not enough input"));
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
            assert_eq!(
                program.memory,
                vec![1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
            );
            assert_eq!(program.pc, 4);

            program.step();
            assert_eq!(
                program.memory,
                vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
            );
            assert_eq!(program.pc, 8);

            program.step();
            assert_eq!(
                program.memory,
                vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
            );
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
            assert_eq!(output[0], 1);
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
            assert_eq!(output, vec![1]);

            let output = program.clone().run(&mut repeat(9));
            assert_eq!(output, vec![0]);
        }

        #[test]
        fn test_less_than_position_mode() {
            // outputs 1 if input < 8
            let program = Program::new(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);

            let output = program.clone().run(&mut repeat(7));
            assert_eq!(output, vec![1]);

            let output = program.clone().run(&mut repeat(8));
            assert_eq!(output, vec![0]);
        }

        #[test]
        fn test_equal_immediate_mode() {
            // outputs 1 if input == 8
            let program = Program::new(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]);

            let output = program.clone().run(&mut repeat(8));
            assert_eq!(output, vec![1]);

            let output = program.clone().run(&mut repeat(9));
            assert_eq!(output, vec![0]);
        }

        #[test]
        fn test_less_than_immediate_mode() {
            // outputs 1 if input < 8
            let program = Program::new(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);

            let output = program.clone().run(&mut repeat(7));
            assert_eq!(output, vec![1]);

            let output = program.clone().run(&mut repeat(8));
            assert_eq!(output, vec![0]);
        }

        #[test]
        fn test_jump_position_mode() {
            // ouputs 1 if input != 0
            let program = Program::new(vec![
                3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9,
            ]);

            let output = program.clone().run(&mut repeat(10));
            assert_eq!(output, vec![1]);

            let output = program.clone().run(&mut repeat(0));
            assert_eq!(output, vec![0]);
        }

        #[test]
        fn test_jump_immediate_mode() {
            // ouputs 1 if input != 0
            let program = Program::new(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);

            let output = program.clone().run(&mut repeat(10));
            assert_eq!(output, vec![1]);

            let output = program.clone().run(&mut repeat(0));
            assert_eq!(output, vec![0]);
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
            assert_eq!(output, vec![999]);
            let output = program.clone().run(&mut repeat(8));
            assert_eq!(output, vec![1000]);
            let output = program.clone().run(&mut repeat(9));
            assert_eq!(output, vec![1001]);
        }
    }

    mod day_09 {
        use super::*;

        #[test]
        fn test_run1() {
            let mem = vec![
                109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
            ];
            let mut program = Program::new(mem.clone());

            let output = program.run(&mut std::iter::empty());
            assert_eq!(output, mem);
        }

        #[test]
        fn test_output_big_number() {
            let mem = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
            let mut program = Program::new(mem);

            let output = program.run(&mut std::iter::empty());
            assert_eq!(output[0].to_string().len(), 16);
        }

        #[test]
        fn test_output_big_number_2() {
            let mem = vec![104, 1125899906842624, 99];
            let mut program = Program::new(mem.clone());

            let output = program.run(&mut std::iter::empty());
            assert_eq!(output[0], mem[1]);
        }
    }
}

use crate::intcode::{Program, State};

pub fn solve(part: u8, data: Result<String, std::io::Error>) {
    let memory: Vec<i32> = data
        .expect("couldn't read data file")
        .split(",")
        .map(|s| s.parse::<i32>().expect("bad data"))
        .collect();

    match part {
        1 => {
            let program = Program::new(memory.clone());
            let output = part_1::find_largest_thruster_signal(&program);
            println!("{:?}", output);
        }
        2 => {
            let output = part_2::find_largest_thruster_signal(&memory);
            println!("{:?}", output);
        }
        _ => (),
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PhaseSetting {
    a: i32,
    b: i32,
    c: i32,
    d: i32,
    e: i32,
}

impl PhaseSetting {
    fn new(a: i32, b: i32, c: i32, d: i32, e: i32) -> Self {
        Self { a, b, c, d, e }
    }

    fn iter() -> impl Iterator<Item = Self> {
        use itertools::Itertools;
        (0..5)
            .permutations(5)
            .map(|v| Self::new(v[0], v[1], v[2], v[3], v[4]))
    }

    fn iter_feedback() -> impl Iterator<Item = Self> {
        use itertools::Itertools;
        (5..10)
            .permutations(5)
            .map(|v| Self::new(v[0], v[1], v[2], v[3], v[4]))
    }
}

mod part_1 {
    use super::*;
    use crate::intcode::Program;
    use std::collections::VecDeque;

    fn run_amp(program: &Program, phase_setting: i32, input: i32) -> Option<i32> {
        let mut inputs = VecDeque::new();
        inputs.push_back(phase_setting);
        inputs.push_back(input);
        let mut program = program.clone();
        program.run(&mut inputs.iter().cloned())
    }

    fn thruster_signal(program: &Program, p: &PhaseSetting) -> i32 {
        let a_output = run_amp(program, p.a, 0).expect("amp A produced no output");
        let b_output = run_amp(program, p.b, a_output).expect("amp B produced no output");
        let c_output = run_amp(program, p.c, b_output).expect("amp C produced no output");
        let d_output = run_amp(program, p.d, c_output).expect("amp D produced no output");
        let e_output = run_amp(program, p.e, d_output).expect("amp E produced no output");
        e_output
    }

    pub fn find_largest_thruster_signal(program: &Program) -> Option<(PhaseSetting, i32)> {
        PhaseSetting::iter()
            .map(|ps| (ps, thruster_signal(program, &ps)))
            .max_by_key(|x| x.1)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_1() {
            let program = Program::new(vec![
                3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
            ]);
            let phase_setting = PhaseSetting::new(4, 3, 2, 1, 0);

            assert_eq!(thruster_signal(&program, &phase_setting), 43210);

            assert_eq!(
                find_largest_thruster_signal(&program),
                Some((phase_setting, 43210))
            );
        }

        #[test]
        fn test_2() {
            let program = Program::new(vec![
                3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4,
                23, 99, 0, 0,
            ]);
            let phase_setting = PhaseSetting::new(0, 1, 2, 3, 4);

            assert_eq!(thruster_signal(&program, &phase_setting), 54321);

            assert_eq!(
                find_largest_thruster_signal(&program),
                Some((phase_setting, 54321))
            );
        }

        #[test]
        fn test_3() {
            let program = Program::new(vec![
                3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33,
                1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
            ]);
            let phase_setting = PhaseSetting::new(1, 0, 4, 3, 2);

            assert_eq!(thruster_signal(&program, &phase_setting), 65210);

            assert_eq!(
                find_largest_thruster_signal(&program),
                Some((phase_setting, 65210))
            );
        }
    }
}

mod part_2 {
    use super::*;
    use std::collections::VecDeque;

    struct Amp {
        name: &'static str,
        program: Program,
    }

    impl Amp {
        fn new(name: &'static str, memory: &[i32]) -> Self {
            Self {
                name,
                program: Program::new(Vec::from(memory)),
            }
        }
    }

    fn thruster_signal(memory: &[i32], phase_setting: &PhaseSetting) -> i32 {
        let mut amps = VecDeque::from(vec![
            Amp::new("A", memory),
            Amp::new("B", memory),
            Amp::new("C", memory),
            Amp::new("D", memory),
            Amp::new("E", memory),
        ]);

        let mut inputs = VecDeque::from(vec![
            phase_setting.a,
            phase_setting.b,
            phase_setting.c,
            phase_setting.d,
            phase_setting.e,
            0,
        ]);

        loop {
            match amps.pop_front() {
                Some(mut amp) => {
                    let mut had_input = false;
                    loop {
                        match amp.program.state {
                            State::Terminated => {
                                break;
                            }
                            State::Running => amp.program.step(),
                            State::Output(o) => {
                                inputs.push_back(o);
                                amp.program.state = State::Running;
                                amps.push_back(amp);
                                break;
                            }
                            State::WaitForInput(addr) => {
                                if had_input {
                                    amps.push_back(amp);
                                    break;
                                } else {
                                    had_input = true;
                                    let input = inputs.pop_front().expect("not enough input");
                                    amp.program.memory[addr] = input;
                                    amp.program.state = State::Running;
                                }
                            }
                        }
                    }
                }
                None => break,
            }
        }

        inputs.pop_front().expect("not enought outputs")
    }

    pub fn find_largest_thruster_signal(memory: &[i32]) -> Option<(PhaseSetting, i32)> {
        PhaseSetting::iter_feedback()
            .map(|ps| (ps, thruster_signal(memory, &ps)))
            .max_by_key(|x| x.1)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_1() {
            let memory = vec![
                3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28,
                -1, 28, 1005, 28, 6, 99, 0, 0, 5,
            ];
            let phase_setting = PhaseSetting::new(9, 8, 7, 6, 5);
            assert_eq!(thruster_signal(&memory, &phase_setting), 139629729);
            assert_eq!(
                find_largest_thruster_signal(&memory),
                Some((phase_setting, 139629729))
            );
        }

        #[test]
        fn test_2() {
            let memory = vec![
                3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001,
                54, -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53,
                55, 53, 4, 53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
            ];
            let phase_setting = PhaseSetting::new(9, 7, 8, 5, 6);
            assert_eq!(thruster_signal(&memory, &phase_setting), 18216);
            assert_eq!(
                find_largest_thruster_signal(&memory),
                Some((phase_setting, 18216))
            );
        }
    }
}

pub fn solve(part: u8, data: Result<String, std::io::Error>) {
    let program: Vec<i32> = data
        .expect("couldn't read data file")
        .split(",")
        .map(|s| s.parse::<i32>().expect("bad data"))
        .collect();

    match part {
        1 => {
            let output = part_1::find_largest_thruster_sigmal(&program);
            println!("{:?}", output);
        }
        _ => (),
    }
}

mod part_1 {
    use std::collections::VecDeque;

    fn run_amp(program: &Vec<i32>, phase_setting: i32, input: i32) -> Option<i32> {
        let mut inputs = VecDeque::new();
        inputs.push_back(phase_setting);
        inputs.push_back(input);
        let mut program = program.clone();
        crate::intcode::run(&mut program, &mut inputs.iter().cloned())
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
    }

    fn thruster_signal(program: &Vec<i32>, p: &PhaseSetting) -> i32 {
        let a_output = run_amp(program, p.a, 0).expect("amp A produced no output");
        let b_output = run_amp(program, p.b, a_output).expect("amp B produced no output");
        let c_output = run_amp(program, p.c, b_output).expect("amp C produced no output");
        let d_output = run_amp(program, p.d, c_output).expect("amp D produced no output");
        let e_output = run_amp(program, p.e, d_output).expect("amp E produced no output");
        e_output
    }

    pub fn find_largest_thruster_sigmal(program: &Vec<i32>) -> Option<(PhaseSetting, i32)> {
        PhaseSetting::iter()
            .map(|ps| (ps, thruster_signal(program, &ps)))
            .max_by_key(|x| x.1)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_1() {
            let program = vec![
                3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
            ];
            let phase_setting = PhaseSetting::new(4, 3, 2, 1, 0);

            assert_eq!(thruster_signal(&program, &phase_setting), 43210);

            assert_eq!(
                find_largest_thruster_sigmal(&program),
                Some((phase_setting, 43210))
            );
        }

        #[test]
        fn test_2() {
            let program = vec![
                3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4,
                23, 99, 0, 0,
            ];
            let phase_setting = PhaseSetting::new(0, 1, 2, 3, 4);

            assert_eq!(thruster_signal(&program, &phase_setting), 54321);

            assert_eq!(
                find_largest_thruster_sigmal(&program),
                Some((phase_setting, 54321))
            );
        }

        #[test]
        fn test_3() {
            let program = vec![
                3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33,
                1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
            ];
            let phase_setting = PhaseSetting::new(1, 0, 4, 3, 2);

            assert_eq!(thruster_signal(&program, &phase_setting), 65210);

            assert_eq!(
                find_largest_thruster_sigmal(&program),
                Some((phase_setting, 65210))
            );
        }
    }
}

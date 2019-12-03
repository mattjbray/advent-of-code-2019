pub mod part_1 {
    fn op_3<F>(mut program: Vec<usize>, pc: usize, f: F) -> (Vec<usize>, Option<usize>)
    where
        F: Fn(&usize, &usize) -> usize,
    {
        let &arg_1_addr = program.get(pc + 1).expect("invalid pc");
        let arg_1 = program.get(arg_1_addr).expect("invalid pc");
        let &arg_2_addr = program.get(pc + 2).expect("invalid pc");
        let arg_2 = program.get(arg_2_addr).expect("invalid pc");
        let &result_addr = program.get(pc + 3).expect("invalid pc");
        program[result_addr] = f(arg_1, arg_2);
        (program, Some(pc + 4))
    }

    fn step(program: Vec<usize>, pc: usize) -> (Vec<usize>, Option<usize>) {
        let instruction = program.get(pc).expect("invalid pc");
        match instruction {
            99 => (program, None),
            1 => op_3(program, pc, |x, y| x + y),
            2 => op_3(program, pc, |x, y| x * y),
            _ => panic!("invalid instruction"),
        }
    }

    fn run_rec(program: Vec<usize>, pc: usize) -> Vec<usize> {
        let (program, pc) = step(program, pc);
        match pc {
            None => program,
            Some(pc) => run_rec(program, pc),
        }
    }

    pub fn run(program: Vec<usize>) -> Vec<usize> {
        run_rec(program, 0)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_step() {
            let program = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];

            let (program, pc) = step(program, 0);
            assert_eq!(program, vec![1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
            assert_eq!(pc, Some(4));

            let (program, pc) = step(program, 4);
            assert_eq!(program, vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
            assert_eq!(pc, Some(8));

            let (program, pc) = step(program, 8);
            assert_eq!(program, vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
            assert_eq!(pc, None);
        }

        #[test]
        fn test_run() {
            assert_eq!(run(vec![1, 0, 0, 0, 99]), vec![2, 0, 0, 0, 99]);

            assert_eq!(run(vec![2, 3, 0, 3, 99]), vec![2, 3, 0, 6, 99]);

            assert_eq!(run(vec![2, 4, 4, 5, 99, 0]), vec![2, 4, 4, 5, 99, 9801]);

            assert_eq!(
                run(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]),
                vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
            );
        }
    }
}

pub mod part_2 {
    pub fn force(program: &Vec<usize>) -> usize {
        let mut noun = 0;
        let mut verb = 0;
        loop {
            let mut p = program.clone();
            p[1] = noun;
            p[2] = verb;
            let p = super::part_1::run(p);
            if p[0] == 19690720 {
                break noun * 100 + verb;
            }
            if verb < 99 {
                verb = verb + 1
            } else {
                noun = noun + 1;
                verb = 0
            };
        }
    }
}

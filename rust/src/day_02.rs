use std::iter::repeat;
use crate::intcode::Program;

pub fn solve(part: u8, data: Result<String, std::io::Error>) {
    let memory: Vec<i32> = data
        .expect("couldn't read data file")
        .split(",")
        .map(|s| s.parse::<i32>().expect("bad data"))
        .collect();

    match part {
        1 => {
            let mut program = Program::new(memory);
            program.memory[1] = 12;
            program.memory[2] = 2;
            let _output = program.run(&mut repeat(0));
            println!("{}", program.memory[0]);
        }
        2 => {
            let result = part_2::force(&memory);
            println!("{}", result);
        }
        _ => (),
    }
}

pub mod part_2 {
    use std::iter::repeat;
    use crate::intcode::Program;
    pub fn force(memory: &Vec<i32>) -> i32 {
        let mut noun = 0;
        let mut verb = 0;
        loop {
            let mut p = Program::new(memory.clone());
            p.memory[1] = noun;
            p.memory[2] = verb;
            let _output = p.run(&mut repeat(0));
            if p.memory[0] == 19690720 {
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

use std::iter::repeat;
use crate::intcode::Program;

pub fn solve(part: u8, data: Result<String, std::io::Error>) {
    let memory: Vec<i64> = data
        .expect("couldn't read data file")
        .split(",")
        .map(|s| s.parse::<i64>().expect("bad data"))
        .collect();

    let mut program = Program::new(memory);

    match part {
        1 => {
            let output = program.run(&mut repeat(1));
            println!("{:?}", output);
        }
        2 => {
            let output = program.run(&mut repeat(5));
            println!("{:?}", output);
        }
        _ => (),
    }
}

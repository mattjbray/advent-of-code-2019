use std::iter::repeat;

pub fn solve(part: u8, data: Result<String, std::io::Error>) {
    let mut program: Vec<i32> = data
        .expect("couldn't read data file")
        .split(",")
        .map(|s| s.parse::<i32>().expect("bad data"))
        .collect();

    match part {
        1 => {
            program[1] = 12;
            program[2] = 2;
            let _output = crate::intcode::run(&mut program, &mut repeat(0));
            println!("{}", program[0]);
        }
        2 => {
            let result = part_2::force(&program);
            println!("{}", result);
        }
        _ => (),
    }
}

pub mod part_2 {
    use std::iter::repeat;
    pub fn force(program: &Vec<i32>) -> i32 {
        let mut noun = 0;
        let mut verb = 0;
        loop {
            let mut p = program.clone();
            p[1] = noun;
            p[2] = verb;
            let _output = crate::intcode::run(&mut p, &mut repeat(0));
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

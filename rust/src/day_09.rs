use crate::intcode::Program;

pub fn solve(part: u8, data: Result<String, std::io::Error>) {
    let memory: Vec<i64> = data
        .expect("couldn't read data file")
        .split(",")
        .map(|s| s.parse::<i64>().expect("bad data"))
        .collect();

    let mut p = Program::new(memory);

    match part {
        1 => {
            let r = run_with_input(&mut p, 1);
            println!("{:?}", r);
        }
        2 => {
            let r = run_with_input(&mut p, 2);
            println!("{:?}", r);
        }
        _ => (),
    }
}

fn run_with_input(p: &mut Program, input: i64) -> Result<i64, Vec<i64>> {
    let mut input = std::iter::once(input);
    let outputs = p.run(&mut input);
    if outputs.len() == 1 {
        Ok(outputs[0])
    } else {
        Err(outputs)
    }
}

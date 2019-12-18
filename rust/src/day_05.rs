pub fn solve(part: u8, data: Result<String, std::io::Error>) {
    let mut program: Vec<i32> = data
        .expect("couldn't read data file")
        .split(",")
        .map(|s| s.parse::<i32>().expect("bad data"))
        .collect();

    match part {
        1 => {
            let output = crate::intcode::run(&mut program, 1);
            println!("{:?}", output);
        }
        2 => {
            let output = crate::intcode::run(&mut program, 5);
            println!("{:?}", output);
        }
        _ => (),
    }
}

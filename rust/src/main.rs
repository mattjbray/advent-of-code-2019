extern crate clap;

fn main() {
    let matches = clap::App::new("AoC 2019")
        .author("Matt Bray <mattjbray@gmail.com>")
        .arg(
            clap::Arg::with_name("day")
                .short("d")
                .long("day")
                .value_name("DD")
                .help("Day to solve")
                .takes_value(true)
                .default_value("01"),
        )
        .arg(
            clap::Arg::with_name("part")
                .short("p")
                .long("part")
                .value_name("PART")
                .help("Part to solve")
                .takes_value(true)
                .default_value("1"),
        )
        .arg(
            clap::Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("Data file")
                .takes_value(true),
        )
        .get_matches();
    let day = matches.value_of("day").unwrap();
    let part = matches.value_of("part").unwrap();

    match day.parse::<i8>() {
        Ok(day) => {
            let def_file = format!("../ocaml/data/day_{:02}.txt", day);
            let file = matches.value_of("file").unwrap_or(&def_file);
            let data = std::fs::read_to_string(file).expect("couldn't read file");
            match day {
                1 => {
                    use day_01::*;
                    let masses: Vec<i32> =
                        data.lines().map(|s| s.parse::<i32>().unwrap()).collect();
                    match part.parse::<i8>() {
                        Ok(1) => {
                            let result1: i32 = masses.iter().map(|&m| part_1::fuel(m)).sum();
                            println!("{}", result1);
                        }
                        Ok(2) => {
                            let result2: i32 = masses.iter().map(|&m| part_2::fuel(m)).sum();
                            println!("{}", result2);
                        }
                        _ => (),
                    }
                }
                2 => {
                    use day_02::*;
                    let mut program: Vec<usize> = data
                        .split(",")
                        .map(|s| s.parse::<usize>().expect("bad data"))
                        .collect();

                    match part.parse::<i8>() {
                        Ok(1) => {
                            program[1] = 12;
                            program[2] = 2;
                            let program = part_1::run(program);
                            println!("{}", program[0]);
                        }
                        Ok(2) => {
                            let result = part_2::force(&program);
                            println!("{}", result);
                        }
                        _ => (),
                    }
                }
                _ => (),
            }
        }
        _ => (),
    }
}

mod day_01;
mod day_02;

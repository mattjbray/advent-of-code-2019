extern crate clap;
#[macro_use]
extern crate itertools;

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
    let file = matches.value_of("file");

    match (day.parse::<u8>(), part.parse::<u8>()) {
        (Ok(day), Ok(part)) => {
            let def_file = format!("../ocaml/data/day_{:02}.txt", day);
            let file = file.unwrap_or(&def_file);
            let data = std::fs::read_to_string(file);
            match day {
                1 => day_01::solve(part, data),
                2 => day_02::solve(part, data),
                3 => day_03::solve(part, data),
                4 => day_04::solve(part, data),
                5 => day_05::solve(part, data),
                6 => day_06::solve(part, data),
                7 => day_07::solve(part, data),
                8 => day_08::solve(part, data),
                9 => day_09::solve(part, data),
                _ => (),
            }
        }
        _ => (),
    }
}

mod intcode;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;

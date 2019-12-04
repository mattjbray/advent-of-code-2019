pub fn solve(part: u8, data: Result<String, std::io::Error>) {
    let masses: Vec<i32> = data
        .expect("couldn't read data file")
        .lines()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    match part {
        1 => {
            let result1: i32 = masses.iter().map(|&m| part_1::fuel(m)).sum();
            println!("{}", result1);
        }
        2 => {
            let result2: i32 = masses.iter().map(|&m| part_2::fuel(m)).sum();
            println!("{}", result2);
        }
        _ => (),
    }
}

pub mod part_1 {
    pub fn fuel(mass: i32) -> i32 {
        (mass / 3) - 2
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn it_works() {
            assert_eq!(fuel(12), 2);
            assert_eq!(fuel(14), 2);
            assert_eq!(fuel(1969), 654);
            assert_eq!(fuel(100756), 33583);
        }
    }
}

pub mod part_2 {
    pub fn fuel(mass: i32) -> i32 {
        let f = super::part_1::fuel(mass);
        if f < 0 {
            0
        } else {
            f + fuel(f)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn it_works() {
            assert_eq!(fuel(14), 2);
            assert_eq!(fuel(1969), 966);
            assert_eq!(fuel(100756), 50346);
        }
    }
}

pub fn solve(part: u8, _data: Result<String, std::io::Error>) {
    let range = 265275..781584;
    match part {
        1 => {
            let result = range.filter(|&i| part_1::meets_criteria(i)).count();
            println!("{}", result);
        }
        2 => {
            let result = range.filter(|&i| part_2::meets_criteria(i)).count();
            println!("{}", result);
        }
        _ => (),
    };
}

mod part_1 {
    use itertools::Itertools;
    fn is_six_digits(s: &str) -> bool {
        s.len() == 6
    }

    fn two_adjacent_digits_the_same(s: &str) -> bool {
        for (a, b) in s.bytes().tuple_windows() {
            if a == b {
                return true;
            }
        }
        false
    }

    fn digits_do_not_decrease(s: &str) -> bool {
        for (a, b) in s.bytes().tuple_windows() {
            if a > b {
                return false;
            }
        }
        true
    }

    pub fn meets_criteria(i: i32) -> bool {
        let s = i.to_string();
        is_six_digits(&s) && two_adjacent_digits_the_same(&s) && digits_do_not_decrease(&s)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_meets_criteria() {
            assert!(meets_criteria(111111));
            assert!(!(meets_criteria(223450)));
            assert!(!(meets_criteria(223450)));
        }
    }
}

mod part_2 {
    fn exactly_two_adjacent_digits_the_same(s: &str) -> bool {
        let mut last = None;
        let mut group = 0;
        for x in s.bytes() {
            if Some(x) == last {
                group += 1;
            } else if group == 2 {
                return true;
            } else {
                last = Some(x);
                group = 1;
            }
        }
        return group == 2;
    }

    pub fn meets_criteria(i: i32) -> bool {
        let s = i.to_string();
        super::part_1::meets_criteria(i) && exactly_two_adjacent_digits_the_same(&s)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_meets_criteria() {
            assert!(meets_criteria(112233));
            assert!(!(meets_criteria(123444)));
            assert!(meets_criteria(111122));
        }
    }
}

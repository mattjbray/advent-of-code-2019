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

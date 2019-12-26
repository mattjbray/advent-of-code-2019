pub fn solve(part: u8, data: Result<String, std::io::Error>) {
    use part_1::*;
    let grid: Grid<Roid> = data.expect("Couldn't read data").parse().unwrap();
    match part {
        1 => {
            println!("{:?}", grid.solve().map(|pos| grid.count_visible(pos)))
        }
        2 => {}
        _ => (),
    }
}

mod part_1 {
    use std::collections::HashMap;

    #[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
    pub struct Pos {
        x: u32,
        y: u32,
    }

    impl Pos {
        fn new(x: u32, y: u32) -> Self {
            Pos { x, y }
        }

        fn angle_to(&self, other: &Pos) -> i32 {
            let dx = other.x as f32 - self.x as f32;
            let dy = other.y as f32 - self.y as f32;
            (dy.atan2(dx) * 1000.) as i32
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct Grid<T>(HashMap<Pos, T>);

    impl<T> std::fmt::Display for Grid<T>
    where
        T: std::fmt::Display,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
            let max_x = self.0.keys().map(|p| p.x).max().unwrap_or(0);
            let max_y = self.0.keys().map(|p| p.y).max().unwrap_or(0);
            for y in 0..max_y + 1 {
                for x in 0..max_x + 1 {
                    let p = Pos::new(x, y);
                    match self.0.get(&p) {
                        Some(v) => write!(f, "{}", v),
                        None => write!(f, "."),
                    }?
                }
                writeln!(f, "")?;
            }
            Ok(())
        }
    }

    #[derive(Debug)]
    pub enum GridParseError<E> {
        Utf8Error(std::str::Utf8Error),
        ItemError(E),
    }

    impl<E> From<std::str::Utf8Error> for GridParseError<E> {
        fn from(err: std::str::Utf8Error) -> Self {
            GridParseError::Utf8Error(err)
        }
    }

    impl<T> std::str::FromStr for Grid<T>
    where
        T: std::str::FromStr,
    {
        type Err = GridParseError<T::Err>;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut roids = HashMap::new();
            for (y, line) in s.lines().enumerate() {
                for (x, byte) in line.as_bytes().iter().enumerate() {
                    match byte {
                        b'.' => (),
                        _ => {
                            let v = std::str::from_utf8(std::slice::from_ref(byte))?;
                            let v = v.parse().map_err(|e| GridParseError::ItemError(e))?;
                            let pos = Pos::new(x as u32, y as u32);
                            roids.insert(pos, v);
                        }
                    }
                }
            }
            Ok(Grid(roids))
        }
    }

    #[derive(Debug)]
    pub struct Roid;

    impl std::fmt::Display for Roid {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
            write!(f, "#")
        }
    }

    #[derive(Debug)]
    pub struct RoidParseError {
        msg: String,
    }

    impl std::fmt::Display for RoidParseError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Couldn't parse Roid: {}", self.msg)
        }
    }

    impl std::error::Error for RoidParseError {}

    impl std::str::FromStr for Roid {
        type Err = RoidParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "#" => Ok(Roid),
                _ => Err(RoidParseError {
                    msg: format!("Not a roid: {}", s),
                }),
            }
        }
    }

    impl<T> Grid<T> {
        pub fn count_visible<'a>(&self, from: &Pos) -> usize {
            let mut angles = std::collections::HashSet::new();
            for to in self.0.keys() {
                if from == to {
                    continue;
                }
                let a = from.angle_to(to);
                if !angles.contains(&a) {
                    angles.insert(a);
                }
            }
            angles.len()
        }

        pub fn to_counts(&self) -> Grid<usize> {
            Grid(self.0.iter().map(|(p, _)| (*p, self.count_visible(p))).collect())
        }

        pub fn solve(&self) -> Option<&Pos> {
            self.0.keys().max_by_key(|p| self.count_visible(p))
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const FACTOR: f64 = std::f64::consts::PI * 1000.;

        #[test]
        fn test_angle() {
            assert_eq!(
                Pos::new(0, 0).angle_to(&Pos::new(1, 1)),
                (FACTOR * 1. / 4.) as i32
            );
            assert_eq!(
                Pos::new(1, 1).angle_to(&Pos::new(0, 0)),
                (FACTOR * -3. / 4.) as i32
            );
        }

        #[test]
        fn test_visible() {
            let grid = "\
            .#..#\n\
            .....\n\
            #####\n\
            ....#\n\
            ...##";

            let grid: Grid<Roid> = grid.parse().unwrap();

            let from = Pos::new(1, 0);

            assert_eq!(grid.count_visible(&from), 7);
        }

        #[test]
        fn test_solve() {
            let grid = "\
.#..#
.....
#####
....#
...##
";

            let expected = "\
.7..7
.....
67775
....7
...87
";

            let grid: Grid<Roid> = grid.parse().unwrap();
            let grid: Grid<usize> = grid.to_counts();
            let expected_grid: Grid<usize> = expected.parse().unwrap();

            assert_eq!(grid, expected_grid);
        }

        #[test]
        fn test_solve_1() {
            let grid = "\
......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####
";
            let grid: Grid<Roid> = grid.parse().unwrap();
            assert_eq!(grid.solve(), Some(&Pos::new(5, 8)));
        }

        #[test]
        fn test_solve_2() {
            let grid = "\
#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.";
            let grid: Grid<Roid> = grid.parse().unwrap();
            assert_eq!(grid.solve(), Some(&Pos::new(1, 2)));
        }

        #[test]
        fn test_solve_3() {
            let grid = "\
.#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..";
            let grid: Grid<Roid> = grid.parse().unwrap();
            assert_eq!(grid.solve(), Some(&Pos::new(6, 3)));
        }

        #[test]
        fn test_solve_4() {
            let grid = "\
.#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##
";
            let grid: Grid<Roid> = grid.parse().unwrap();
            let best = grid.solve();
            assert_eq!(best.map(|p| grid.count_visible(p)), Some(210));
            assert_eq!(grid.solve(), Some(&Pos::new(11, 13)));
        }
    }
}

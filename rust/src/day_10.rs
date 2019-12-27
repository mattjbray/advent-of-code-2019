use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::f32::consts::PI;

use crate::grid::{Grid,Pos};

pub fn solve(part: u8, data: Result<String, std::io::Error>) {
    let grid: Grid<Roid> = data.expect("Couldn't read data").parse().unwrap();
    let best_pos = grid.solve().unwrap();
    match part {
        1 => println!("{}", grid.count_visible(best_pos)),
        2 => println!("{}", grid.lazer(best_pos)[199]),
        _ => (),
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
            let a = (a * 1000.) as i32;
            if !angles.contains(&a) {
                angles.insert(a);
            }
        }
        angles.len()
    }

    fn to_counts(&self) -> Grid<usize> {
        Grid(
            self.0
                .iter()
                .map(|(p, _)| (*p, self.count_visible(p)))
                .collect(),
        )
    }

    pub fn solve(&self) -> Option<&Pos> {
        self.0.keys().max_by_key(|p| self.count_visible(p))
    }

    pub fn lazer(&self, from: &Pos) -> Vec<&Pos> {
        let mut by_angle: BTreeMap<i32, Vec<&Pos>> = BTreeMap::new();

        for to in self.0.keys() {
            if from == to {
                continue;
            }
            let a = from.angle_to(to);
            let a = (a * 1000.) as i32;
            by_angle
                .entry(a)
                .and_modify(|ps| ps.push(&to))
                .or_insert(vec![to]);
        }

        for (_angle, roids) in by_angle.iter_mut() {
            roids.sort_by_key(|p| from.dist_to(p));
            roids.reverse();
        }

        let mut destroyed_roids = vec![];

        loop {
            let keys: Vec<i32> = by_angle.keys().map(|a| *a).collect();
            if keys.is_empty() {
                break;
            }
            for angle in keys {
                if let Entry::Occupied(mut entry) = by_angle.entry(angle) {
                    let roids = entry.get_mut();
                    if let Some(roid) = roids.pop() {
                        destroyed_roids.push(roid);
                    }
                    if roids.is_empty() {
                        entry.remove_entry();
                    }
                }
            }
        }

        destroyed_roids
    }
}

mod part_1 {

    #[cfg(test)]
    pub mod tests {
        use super::super::*;

        #[test]
        fn test_angle() {
            // ...
            // .x.
            // ...
            let angles = [
                Pos::new(1, 1).angle_to(&Pos::new(1, 0)) / PI,
                Pos::new(1, 1).angle_to(&Pos::new(2, 0)) / PI,
                Pos::new(1, 1).angle_to(&Pos::new(2, 1)) / PI,
                Pos::new(1, 1).angle_to(&Pos::new(2, 2)) / PI,
                Pos::new(1, 1).angle_to(&Pos::new(1, 2)) / PI,
                Pos::new(1, 1).angle_to(&Pos::new(0, 2)) / PI,
                Pos::new(1, 1).angle_to(&Pos::new(0, 1)) / PI,
                Pos::new(1, 1).angle_to(&Pos::new(0, 0)) / PI,
            ];
            assert_eq!(
                angles,
                [0.0, 0.2500001, 0.50000006, 0.7500001, 0.99999994, 1.25, 1.5000001, 1.75]
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

        pub const BIG_GRID: &str = "\
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

        #[test]
        fn test_solve_4() {
            let grid: Grid<Roid> = BIG_GRID.parse().unwrap();
            let best = grid.solve();
            assert_eq!(best.map(|p| grid.count_visible(p)), Some(210));
            assert_eq!(grid.solve(), Some(&Pos::new(11, 13)));
        }
    }
}

mod part_2 {
    #[cfg(test)]
    mod tests {
        use super::super::*;

        #[test]
        fn test_smth() {
            let grid = "\
.#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....#...###..
..#.#.....#....##
";

            let grid: Grid<Roid> = grid.parse().unwrap();
            let from = Pos::new(8, 3);
            let destroyed_roids = grid.lazer(&from);

            assert_eq!(
                destroyed_roids[0..5],
                [
                    &Pos::new(8, 1),
                    &Pos::new(9, 0),
                    &Pos::new(9, 1),
                    &Pos::new(10, 0),
                    &Pos::new(9, 2),
                ]
            );
        }

        #[test]
        fn test_big() {
            let grid: Grid<Roid> = part_1::tests::BIG_GRID.parse().unwrap();
            let from = Pos::new(11, 13);
            let destroyed_roids = grid.lazer(&from);
            assert_eq!(destroyed_roids[0], &Pos::new(11, 12));
            assert_eq!(destroyed_roids[1], &Pos::new(12, 1));
            assert_eq!(destroyed_roids[2], &Pos::new(12, 2));
            assert_eq!(destroyed_roids[9], &Pos::new(12, 8));
            assert_eq!(destroyed_roids[19], &Pos::new(16, 0));
            assert_eq!(destroyed_roids[49], &Pos::new(16, 9));
            assert_eq!(destroyed_roids[99], &Pos::new(10, 16));
            assert_eq!(destroyed_roids[198], &Pos::new(9, 6));
            assert_eq!(destroyed_roids[199], &Pos::new(8, 2));
            assert_eq!(destroyed_roids[200], &Pos::new(10, 9));
            assert_eq!(destroyed_roids[298], &Pos::new(11, 1));
            assert_eq!(destroyed_roids.len(), 299);
        }
    }
}

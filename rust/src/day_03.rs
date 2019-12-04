pub fn solve(part: u8, data: Result<String, std::io::Error>) {
    use part_1::*;
    let data: String = data.expect("couldn't read data file");
    let mut lines = data.lines();
    let moves_1 = lines.next().expect("wrong number of lines in data");
    let moves_1 = parse_moves(&moves_1).expect("couldn't parse moves from data");
    let moves_2 = lines.next().expect("wrong number of lines in data");
    let moves_2 = parse_moves(&moves_2).expect("couldn't parse moves from data");
    match part {
        1 => {
            let result = part_1::solve(&moves_1, &moves_2)
                .map_or(String::from("No solution"), |i| format!("{}", i));
            println!("{}", result)
        }
        2 => {
            let result = part_2::solve(&moves_1, &moves_2)
                .map_or(String::from("No solution"), |i| format!("{}", i));
            println!("{}", result)
        }
        _ => (),
    }
}

mod part_1 {
    use std::str::FromStr;

    #[derive(Debug)]
    pub struct ParseError;

    #[derive(Clone, Copy)]
    enum Dir {
        U,
        R,
        D,
        L,
    }

    impl Dir {
        fn is_vertical(&self) -> bool {
            use Dir::*;
            match self {
                U | D => true,
                L | R => false,
            }
        }
    }

    impl FromStr for Dir {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "U" => Ok(Self::U),
                "R" => Ok(Self::R),
                "D" => Ok(Self::D),
                "L" => Ok(Self::L),
                _ => Err(ParseError),
            }
        }
    }

    #[derive(Clone, Copy)]
    pub struct Move {
        dir: Dir,
        steps: u32,
    }

    impl FromStr for Move {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (dir, steps) = s.split_at(1);
            let dir = dir.parse::<Dir>();
            dir.and_then(|dir| {
                let steps = steps.parse::<u32>().map_err(|_| ParseError);
                steps.map(|steps| Move { dir, steps })
            })
        }
    }

    pub fn parse_moves(s: &str) -> Result<Vec<Move>, ParseError> {
        s.split(",").map(|s| s.parse::<Move>()).collect()
    }

    #[derive(Clone, Copy, PartialEq)]
    pub struct Pos {
        x: i32,
        y: i32,
    }

    impl Pos {
        fn new(x: i32, y: i32) -> Self {
            Self { x, y }
        }

        pub const CENTRAL_PORT: Self = Self { x: 0, y: 0 };

        fn manhattan_dist(&self, other: &Self) -> u32 {
            (other.x - self.x).abs() as u32 + (other.y - self.y).abs() as u32
        }

        fn move_(&self, Move { dir, steps }: Move) -> Self {
            let Pos { x, y } = *self;
            let steps = steps as i32;
            use Dir::*;
            match dir {
                U => Self::new(x, y - steps),
                R => Self::new(x + steps, y),
                D => Self::new(x, y + steps),
                L => Self::new(x - steps, y),
            }
        }
    }

    #[derive(Clone, Copy)]
    pub struct Segment {
        start: Pos,
        move_: Move,
        steps_to_start: u32,
    }

    type Path = Vec<Segment>;

    pub fn path(moves: &[Move]) -> Path {
        let init = (Pos::CENTRAL_PORT, 0);
        let mut path = Vec::new();
        moves.iter().fold(init, |(pos, steps), &move_| {
            let segment = Segment {
                start: pos,
                move_,
                steps_to_start: steps,
            };
            path.push(segment);
            let pos = pos.move_(move_);
            let steps = steps + move_.steps;

            (pos, steps)
        });
        path
    }

    pub struct Intersection {
        pos: Pos,
        steps: u32,
    }

    impl Intersection {
        pub fn pos(&self) -> Pos {
            self.pos
        }
        pub fn steps(&self) -> u32 {
            self.steps
        }
    }

    impl Segment {
        fn ends(&self) -> (Pos, Pos) {
            let Pos { x: x1, y: y1 } = self.start;
            let Pos { x: x2, y: y2 } = self.start.move_(self.move_);
            (
                Pos {
                    x: std::cmp::min(x1, x2),
                    y: std::cmp::min(y1, y2),
                },
                Pos {
                    x: std::cmp::max(x1, x2),
                    y: std::cmp::max(y1, y2),
                },
            )
        }

        fn intersection_perpendicular(h: &Self, v: &Self) -> Option<Intersection> {
            let (Pos { x: hx1, y: hy }, Pos { x: hx2, y: _ }) = h.ends();
            let (Pos { x: vx, y: vy1 }, Pos { x: _, y: vy2 }) = v.ends();
            if (hx1 <= vx && vx <= hx2) && (vy1 <= hy && hy <= vy2) {
                let h_steps: u32 = h.steps_to_start + (vx - h.start.x).abs() as u32;
                let v_steps: u32 = v.steps_to_start + (hy - v.start.y).abs() as u32;
                Some(Intersection {
                    pos: Pos { x: vx, y: hy },
                    steps: h_steps + v_steps,
                })
            } else {
                None
            }
        }

        fn intersection(&self, other: &Self) -> Option<Intersection> {
            match (self.move_.dir.is_vertical(), other.move_.dir.is_vertical()) {
                (true, true) | (false, false) => None,
                (true, false) => Self::intersection_perpendicular(&other, self),
                (false, true) => Self::intersection_perpendicular(self, &other),
            }
        }
    }

    pub fn intersections(p1: &[Segment], p2: &[Segment]) -> Vec<Intersection> {
        let mut is: Vec<Intersection> = Vec::new();
        for (s1, s2) in iproduct!(p1, p2) {
            match s1.intersection(s2) {
                None => (),
                Some(i) => is.push(i),
            }
        }
        is
    }

    fn closest_to<'a>(is: &'a [Intersection], &pos: &Pos) -> Option<&'a Intersection> {
        let mut is: Vec<&Intersection> = is.iter().filter(|i| i.pos != pos).collect();
        is.sort_by_cached_key(|i| i.pos.manhattan_dist(&pos));
        is.first().map(|&i| i)
    }

    pub fn solve(moves1: &[Move], moves2: &[Move]) -> Option<u32> {
        let is = intersections(&path(moves1), &path(moves2));
        closest_to(&is, &Pos::CENTRAL_PORT).map(|i| i.pos.manhattan_dist(&Pos::CENTRAL_PORT))
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_solve() {
            let moves_1 = parse_moves("R8,U5,L5,D3").unwrap();
            let moves_2 = parse_moves("U7,R6,D4,L4").unwrap();
            assert_eq!(solve(&moves_1, &moves_2), Some(6));

            let moves_1 = parse_moves("R75,D30,R83,U83,L12,D49,R71,U7,L72").unwrap();
            let moves_2 = parse_moves("U62,R66,U55,R34,D71,R55,D58,R83").unwrap();
            assert_eq!(solve(&moves_1, &moves_2), Some(159));

            let moves_1 = parse_moves("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51").unwrap();
            let moves_2 = parse_moves("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7").unwrap();
            assert_eq!(solve(&moves_1, &moves_2), Some(135))
        }
    }
}

mod part_2 {
    use super::part_1::*;

    fn least_steps<'a>(is: &'a [Intersection]) -> Option<&'a Intersection> {
        let mut is: Vec<&Intersection> =
            is.iter().filter(|i| i.pos() != Pos::CENTRAL_PORT).collect();
        is.sort_by_key(|i| i.steps());
        is.first().map(|&i| i)
    }

    pub fn solve(moves1: &[Move], moves2: &[Move]) -> Option<u32> {
        let is = intersections(&path(moves1), &path(moves2));
        least_steps(&is).map(|i| i.steps())
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_solve() {
            let moves_1 = parse_moves("R8,U5,L5,D3").unwrap();
            let moves_2 = parse_moves("U7,R6,D4,L4").unwrap();
            assert_eq!(solve(&moves_1, &moves_2), Some(30));

            let moves_1 = parse_moves("R75,D30,R83,U83,L12,D49,R71,U7,L72").unwrap();
            let moves_2 = parse_moves("U62,R66,U55,R34,D71,R55,D58,R83").unwrap();
            assert_eq!(solve(&moves_1, &moves_2), Some(610));

            let moves_1 = parse_moves("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51").unwrap();
            let moves_2 = parse_moves("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7").unwrap();
            assert_eq!(solve(&moves_1, &moves_2), Some(410))
        }
    }
}

use crate::grid::{Grid, Pos};
use crate::intcode;

pub fn solve(part: u8, data: Result<String, std::io::Error>) {
    let memory: Vec<i64> = data
        .expect("couldn't read data file")
        .split(",")
        .map(|s| s.parse::<i64>().expect("bad data"))
        .collect();
    let mut robot = Robot::new();
    let mut program = intcode::Program::new(memory);
    let mut grid = Grid::new();
    match part {
        1 => {
            robot.run(&mut program, &mut grid);
            println!("{}", grid.0.len());
        }
        2 => {
            grid.0.insert(robot.pos, Color::White);
            robot.run(&mut program, &mut grid);
            println!("{}", grid);
        }
        _ => (),
    }
}

enum Direction {
    North,
    East,
    South,
    West,
}

enum Turn {
    Left,
    Right,
}

impl Turn {
    fn of_output(o: i64) -> Option<Turn> {
        match o {
            0 => Some(Turn::Left),
            1 => Some(Turn::Right),
            _ => None,
        }
    }
}

#[derive(Clone, Copy)]
enum Color {
    Black,
    White,
}

impl Color {
    fn to_input(&self) -> i64 {
        match self {
            Color::Black => 0,
            Color::White => 1,
        }
    }

    fn of_output(o: i64) -> Option<Self> {
        match o {
            0 => Some(Color::Black),
            1 => Some(Color::White),
            _ => None,
        }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                Color::Black => " ",
                Color::White => "█",
            }
        )
    }
}

struct Robot {
    pos: Pos,
    facing: Direction,
}

impl Robot {
    fn new() -> Self {
        Self {
            pos: Pos::new(0, 0),
            facing: Direction::North,
        }
    }

    fn turn(&mut self, t: Turn) {
        self.facing = {
            match (&self.facing, t) {
                (Direction::North, Turn::Right) => Direction::East,
                (Direction::East, Turn::Right) => Direction::South,
                (Direction::South, Turn::Right) => Direction::West,
                (Direction::West, Turn::Right) => Direction::North,
                (Direction::North, Turn::Left) => Direction::West,
                (Direction::East, Turn::Left) => Direction::North,
                (Direction::South, Turn::Left) => Direction::East,
                (Direction::West, Turn::Left) => Direction::South,
            }
        }
    }

    fn step(&mut self) {
        self.pos = {
            match &self.facing {
                Direction::North => Pos::new(self.pos.x, self.pos.y - 1),
                Direction::East => Pos::new(self.pos.x + 1, self.pos.y),
                Direction::South => Pos::new(self.pos.x, self.pos.y + 1),
                Direction::West => Pos::new(self.pos.x - 1, self.pos.y),
            }
        }
    }

    fn run(&mut self, program: &mut intcode::Program, grid: &mut Grid<Color>) {
        let mut output_recieved = true;

        loop {
            match program.state {
                intcode::State::Terminated => {
                    break;
                }
                intcode::State::Running => program.step(),
                intcode::State::Output(o) => {
                    if !output_recieved {
                        // First output is to paint the tile
                        let color = Color::of_output(o).expect("Invalid output");
                        grid.0
                            .entry(self.pos)
                            .and_modify(|e| *e = color)
                            .or_insert(color);
                    } else {
                        // Second output is direction to turn
                        let turn = Turn::of_output(o).expect("Invalid output");
                        self.turn(turn);
                        self.step();
                    }
                    program.state = intcode::State::Running;
                    output_recieved = true;
                }
                intcode::State::WaitForInput(addr) => {
                    output_recieved = false;
                    let color = grid.0.entry(self.pos).or_insert(Color::Black);
                    program.memory[addr] = color.to_input();
                    program.state = intcode::State::Running;
                }
            }
        }
    }
}

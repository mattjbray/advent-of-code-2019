use crate::grid::{Grid, Pos};
use crate::intcode::{Program, State};

pub fn solve(part: u8, data: Result<String, std::io::Error>) {
    let memory: Vec<i64> = data
        .expect("couldn't read data file")
        .split(",")
        .map(|s| s.parse::<i64>().expect("bad data"))
        .collect();
    let mut program = Program::new(memory);
    let mut grid = Grid::new();
    match part {
        1 => {
            go(&mut program, &mut grid);
            println!("{}", grid);
            let num_block_tiles = grid.0.values().filter(|t| t.is_block()).count();
            println!("{}", num_block_tiles);
        }
        _ => (),
    }
}

enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl Tile {
    fn of_id(id: i64) -> Option<Self> {
        match id {
            0 => Some(Tile::Empty),
            1 => Some(Tile::Wall),
            2 => Some(Tile::Block),
            3 => Some(Tile::Paddle),
            4 => Some(Tile::Ball),
            _ => None,
        }
    }

    fn is_block(&self) -> bool {
        match self {
            Tile::Block => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Empty => " ",
                Tile::Wall => "█",
                Tile::Block => "x",
                Tile::Paddle => "_",
                Tile::Ball => "o",
            }
        )
    }
}

fn go(program: &mut Program, grid: &mut Grid<Tile>) {
    let mut next_x = None;
    let mut next_y = None;

    loop {
        match program.state {
            State::Terminated => {
                break;
            }
            State::Running => program.step(),
            State::Output(o) => {
                match (next_x, next_y) {
                    (None, _) => next_x = Some(o),
                    (Some(_), None) => next_y = Some(o),
                    (Some(x), Some(y)) => {
                        grid.0
                            .insert(Pos::new(x as i32, y as i32), Tile::of_id(o).unwrap());
                        next_x = None;
                        next_y = None;
                    }
                }
                program.state = State::Running;
            }
            State::WaitForInput(_addr) => {
                program.state = State::Running;
            }
        }
    }
}

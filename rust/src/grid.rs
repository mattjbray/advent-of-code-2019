use std::collections::HashMap;
use std::f32::consts::PI;

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Pos { x, y }
    }

    pub fn angle_to(&self, other: &Pos) -> f32 {
        let dx = other.x as f32 - self.x as f32;
        let dy = other.y as f32 - self.y as f32;
        (dy.atan2(dx) + PI * 5. / 2.) % (2. * PI)
    }

    pub fn dist_to(&self, other: &Pos) -> i32 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        dy.abs() + dx.abs()
    }
}

impl std::fmt::Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

#[derive(Debug, PartialEq)]
pub struct Grid<T>(pub HashMap<Pos, T>);

impl<T> Grid<T> {
    pub fn new() -> Self {
        Grid(HashMap::new())
    }

    fn min_pos(&self) -> Pos {
        let min_x = self.0.keys().map(|p| p.x).min().unwrap_or(0);
        let min_y = self.0.keys().map(|p| p.y).min().unwrap_or(0);
        Pos::new(min_x, min_y)
    }

    fn max_pos(&self) -> Pos {
        let max_x = self.0.keys().map(|p| p.x).max().unwrap_or(0);
        let max_y = self.0.keys().map(|p| p.y).max().unwrap_or(0);
        Pos::new(max_x, max_y)
    }
}

impl<T> std::fmt::Display for Grid<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let min_p = self.min_pos();
        let max_p = self.max_pos();
        for y in min_p.y..max_p.y + 1 {
            for x in min_p.x..max_p.x + 1 {
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
                        let pos = Pos::new(x as i32, y as i32);
                        roids.insert(pos, v);
                    }
                }
            }
        }
        Ok(Grid(roids))
    }
}

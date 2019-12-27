pub fn solve(part: u8, data: Result<String, std::io::Error>) {
    let mut positions = vec![];

    for line in data.expect("Couldn't read data file").lines() {
        let p: Vec3 = line.parse().unwrap();
        positions.push(p);
    }

    let mut system = MoonSystem::from_positions(positions);

    match part {
        1 => {
            for _ in 0..1000 {
                system.step();
            }
            println!("{}", system.total_energy());
        }
        _ => (),
    }
}

#[derive(Debug, PartialEq)]
struct Vec3([i32; 3]);

impl Vec3 {
    fn zero() -> Self {
        Self([0; 3])
    }
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self([x, y, z])
    }

    fn add(&mut self, other: &Self) {
        for i in 0..3 {
            self.0[i] += other.0[i]
        }
    }
}

impl std::str::FromStr for Vec3 {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_start_matches('<');
        let s = s.trim_end_matches('>');
        let parts: Vec<_> = s.split(", ").collect();
        if parts.len() != 3 {
            return Err(())
        }
        let x_parts: Vec<_> = parts[0].split('=').collect();
        let x: i32 = x_parts[1].parse().unwrap();
        let y_parts: Vec<_> = parts[1].split('=').collect();
        let y: i32 = y_parts[1].parse().unwrap();
        let z_parts: Vec<_> = parts[2].split('=').collect();
        let z: i32 = z_parts[1].parse().unwrap();
        Ok(Self::new(x, y, z))
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<{:3},{:3},{:3}>", self.0[0], self.0[1], self.0[2])
    }
}

#[derive(Debug)]
struct Moon {
    pos: Vec3,
    vel: Vec3,
}

impl Moon {
    fn new(pos: Vec3) -> Self {
        Self {
            pos,
            vel: Vec3::zero(),
        }
    }

    fn pull(&mut self, other: &mut Self) {
        for i in 0..3 {
            match self.pos.0[i].cmp(&other.pos.0[i]) {
                std::cmp::Ordering::Less => {
                    self.vel.0[i] += 1;
                    other.vel.0[i] -= 1;
                }
                std::cmp::Ordering::Greater => {
                    self.vel.0[i] -= 1;
                    other.vel.0[i] += 1;
                }
                std::cmp::Ordering::Equal => (),
            }
        }
    }

    fn apply_vel(&mut self) {
        self.pos.add(&self.vel)
    }

    fn potential_energy(&self) -> i32 {
        self.pos.0.iter().map(|i| i.abs()).sum()
    }

    fn kinetic_energy(&self) -> i32 {
        self.vel.0.iter().map(|i| i.abs()).sum()
    }

    fn total_energy(&self) -> i32 {
        self.potential_energy() * self.kinetic_energy()
    }
}

impl std::fmt::Display for Moon {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "pos={}, vel={}", self.pos, self.vel)
    }
}

#[derive(Debug)]
struct MoonSystem {
    moons: Vec<Moon>,
}

impl MoonSystem {
    fn new(moons: Vec<Moon>) -> Self {
        Self { moons }
    }

    fn from_positions(ps: Vec<Vec3>) -> Self {
        let moons = ps.into_iter().map(|p| Moon::new(p)).collect();
        Self{ moons }
    }

    fn step_gravity(&mut self) {
        for i in 0..self.moons.len() - 1 {
            for j in i + 1..self.moons.len() {
                let (ms1, ms2) = self.moons.split_at_mut(j);
                let m1 = &mut ms1[i];
                let m2 = &mut ms2[0];
                m1.pull(m2);
            }
        }
    }

    fn step_velocity(&mut self) {
        for m in self.moons.iter_mut() {
            m.apply_vel()
        }
    }

    fn step(&mut self) {
        self.step_gravity();
        self.step_velocity();
    }

    fn total_energy(&self) -> i32 {
        self.moons.iter().map(|m| m.total_energy()).sum()
    }
}

impl std::fmt::Display for MoonSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for moon in self.moons.iter() {
            writeln!(f, "{}", moon)?;
        }
        Ok(())
    }
}

mod part_1 {
    #[cfg(test)]
    mod tests {
        use super::super::*;

        #[test]
        fn test_1() {
            let input = "\
<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>
";

            let mut positions = vec![];

            for line in input.lines() {
                let p: Vec3 = line.parse().unwrap();
                positions.push(p);
            }

            let mut system = MoonSystem::from_positions(positions);

            assert_eq!(system.moons[0].pos, Vec3::new(-1, 0, 2));

            system.step();

            assert_eq!(system.moons[0].pos, Vec3::new(2, -1, 1));
            assert_eq!(system.moons[0].vel, Vec3::new(3, -1, -1));

            system.step();

            assert_eq!(system.moons[0].pos, Vec3::new(5, -3, -1));
            assert_eq!(system.moons[0].vel, Vec3::new(3, -2, -2));

            system.step();
            system.step();
            system.step();
            system.step();
            system.step();
            system.step();
            system.step();
            system.step();

            assert_eq!(system.moons[0].pos, Vec3::new(2, 1, -3));
            assert_eq!(system.moons[0].vel, Vec3::new(-3, -2, 1));

            assert_eq!(system.moons[0].potential_energy(), 6);
            assert_eq!(system.moons[0].kinetic_energy(), 6);

            assert_eq!(system.total_energy(), 179);
        }

        #[test]
        fn test_2() {
            let input = "\
<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>
";

            let mut positions = vec![];

            for line in input.lines() {
                let p: Vec3 = line.parse().unwrap();
                positions.push(p);
            }

            let mut system = MoonSystem::from_positions(positions);

            for _ in 0..100 {
                system.step();
            }

            assert_eq!(system.moons[0].pos, Vec3::new(8, -12, -9));
            assert_eq!(system.moons[0].vel, Vec3::new(-7, 3, 0));

            assert_eq!(system.moons[0].potential_energy(), 29);
            assert_eq!(system.moons[0].kinetic_energy(), 10);

            assert_eq!(system.total_energy(), 1940);
        }
    }
}

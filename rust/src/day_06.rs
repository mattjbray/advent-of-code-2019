pub fn solve(part: u8, data: Result<String, std::io::Error>) {
    let t: part_1::OrbitTree = data
        .expect("Couldn't read data file")
        .parse()
        .expect("Couldn't parse data");
    match part {
        1 => println!("{}", t.checksum()),
        2 => println!("{}", t.transfers_required("YOU", "SAN").unwrap()),
        _ => (),
    }
}

mod part_1 {
    use std::fmt;

    #[derive(Clone, Debug, PartialEq)]
    pub struct OrbitTree {
        pub object: String,
        pub orbitees: Vec<OrbitTree>,
    }

    impl OrbitTree {
        fn new(object: &str, orbitees: Vec<Self>) -> Self {
            OrbitTree {
                object: object.to_string(),
                orbitees,
            }
        }

        fn leaf(object: &str) -> Self {
            Self::new(object, Vec::new())
        }

        fn find(&mut self, object: &str) -> Option<&mut Self> {
            if self.object.as_str() == object {
                Some(self)
            } else {
                for o in self.orbitees.iter_mut() {
                    match o.find(object) {
                        Some(t) => return Some(t),
                        None => (),
                    }
                }
                return None;
            }
        }

        fn checksum_rec(&self, depth: u32) -> u32 {
            let s: u32 = self
                .orbitees
                .iter()
                .map(|o| o.checksum_rec(depth + 1))
                .sum();
            s + depth
        }

        pub fn checksum(&self) -> u32 {
            self.checksum_rec(0)
        }

        fn fmt_rec(&self, f: &mut fmt::Formatter, depth: u32) -> fmt::Result {
            write!(
                f,
                "{:width$}{}\n",
                "",
                self.object,
                width = (depth as usize)
            )?;
            for o in self.orbitees.iter() {
                o.fmt_rec(f, depth + 1)?;
            }
            write!(f, "")
        }
    }

    impl std::str::FromStr for OrbitTree {
        type Err = String;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut ts: Vec<Self> = Vec::new();

            for line in s.lines() {
                let parts: Vec<&str> = line.split(')').collect();

                if parts.len() != 2 {
                    return Err(format!("Invalid line: {}", line));
                }

                // check if we aleady have the child tree
                let subtree = {
                    ts.iter()
                        .position(|t| t.object.as_str() == parts[1])
                        .map_or_else(|| Self::leaf(parts[1]), |i| ts.remove(i))
                };

                // find the parent tree or add a new partial tree to ts
                match ts.iter_mut().find_map(|t| t.find(parts[0])) {
                    Some(tree) => tree.orbitees.push(subtree),
                    None => ts.push(Self::new(parts[0], vec![subtree])),
                };
            }
            if ts.len() != 1 {
                Err("Disjoint trees".to_string())
            } else {
                Ok(ts[0].clone())
            }
        }
    }

    impl fmt::Display for OrbitTree {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.fmt_rec(f, 0)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn make() -> OrbitTree {
            let new = OrbitTree::new;
            let leaf = OrbitTree::leaf;

            let f = leaf("F");
            let l = leaf("L");
            let k = new("K", vec![l]);
            let j = new("J", vec![k]);
            let e = new("E", vec![f, j]);
            let i = leaf("I");
            let d = new("D", vec![e, i]);
            let c = new("C", vec![d]);
            let h = leaf("H");
            let g = new("G", vec![h]);
            let b = new("B", vec![c, g]);
            let com = new("COM", vec![b]);
            com
        }

        #[test]
        fn test_parse() {
            let input = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
";

            let t: Result<OrbitTree, _> = input.parse();
            let expected = make();

            assert_eq!(t, Ok(expected));
        }

        #[test]
        fn test_parse_unsorted() {
            let input = "COM)B
C)D
B)C
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
";

            let t: Result<OrbitTree, _> = input.parse();
            let expected = make();

            assert_eq!(t, Ok(expected));
        }

        #[test]
        fn test_checksum() {
            let t = make();
            assert_eq!(t.checksum(), 42);
        }
    }
}

mod part_2 {
    use super::part_1::*;

    impl OrbitTree {
        fn contains(&self, a: &str) -> bool {
            self.object.as_str() == a || self.orbitees.iter().any(|o| o.contains(a))
        }

        fn first_common_ancestor(&self, a: &str, b: &str) -> Option<&Self> {
            if self.contains(a) && self.contains(b) {
                self.orbitees
                    .iter()
                    .find_map(|o| o.first_common_ancestor(a, b))
                    .or(Some(&self))
            } else {
                None
            }
        }

        fn depth_to(&self, a: &str) -> Option<u32> {
            if self.object.as_str() == a {
                Some(0)
            } else {
                self.orbitees
                    .iter()
                    .find_map(|o| o.depth_to(a))
                    .map(|d| d + 1)
            }
        }

        pub fn transfers_required(&self, a: &str, b: &str) -> Option<u32> {
            self.first_common_ancestor(a, b)
                .and_then(|ca| match (ca.depth_to(a), ca.depth_to(b)) {
                    (Some(da), Some(db)) => Some(da + db - 2), // exclude the leaf nodes
                    _ => None,
                })
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_transfers() {
            let input = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN
";
            let t: OrbitTree = input.parse().unwrap();

            let ca = t.first_common_ancestor("YOU", "SAN").unwrap();
            assert_eq!(ca.object.as_str(), "D");

            assert_eq!(ca.depth_to("YOU"), Some(4));
            assert_eq!(ca.depth_to("SAN"), Some(2));

            assert_eq!(t.transfers_required("YOU", "SAN"), Some(4));
        }
    }
}

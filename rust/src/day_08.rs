pub fn solve(part: u8, data: Result<String, std::io::Error>) {
    let s = part_1::Size::new(25, 6);
    let i = part_1::Image::from_str(data.expect("Could not read data file").as_str(), s);
    match part {
        1 => println!("{}", i.checksum().unwrap()),
        2 => println!("{}", i),
        _ => (),
    }
}

mod part_1 {

    pub struct Size {
        pub width: usize,
        pub height: usize,
    }

    impl Size {
        pub fn new(width: usize, height: usize) -> Self {
            Self { width, height }
        }

        pub fn size(&self) -> usize {
            self.width * self.height
        }
    }

    struct Layer<'a>(&'a [u8]);

    impl<'a> Layer<'a> {
        fn count(&self, px: u8) -> usize {
            self.0.iter().filter(|&&pix| pix == px).count()
        }
    }

    pub struct Image {
        pub size: Size,
        pub pixels: Vec<u8>,
    }

    impl Image {
        pub fn from_str(s: &str, size: Size) -> Image {
            let pixels: Vec<u8> = s.as_bytes().iter().map(|b| b - 48).collect();
            Image { pixels, size }
        }

        fn layers(&self) -> impl Iterator<Item = Layer> {
            self.pixels
                .chunks(self.size.width * self.size.height)
                .map(|pxs| Layer(pxs))
        }

        fn layer_with_fewest_zeros(&self) -> Option<Layer> {
            self.layers().min_by_key(|l| l.count(0))
        }

        pub fn checksum(&self) -> Option<usize> {
            self.layer_with_fewest_zeros()
                .map(|l| l.count(1) * l.count(2))
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_1() {
            let i = Image::from_str("123456789012", Size::new(3, 2));
            assert_eq!(i.pixels[0], 1);
        }
    }
}

mod part_2 {
    use super::part_1::*;

    impl Image {
        pub fn render(&self) -> Vec<u8> {
            let size = self.size.size();
            let mut result = Vec::new();
            for i in 0..size {
                let mut layer = 0;
                loop {
                    let px = self.pixels[i + layer*size];
                    if px == 2 {
                        layer += 1;
                    } else {
                        result.push(px);
                        break
                    }
                }
            }

            result
        }
    }

    impl std::fmt::Display for Image {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let pixels = self.render();
            let lines = pixels.chunks(self.size.width);
            for line in lines {
                for px in line {
                    write!(f, "{}", if *px == 0 { " " } else { "█" })?;
                }
                writeln!(f, "")?;
            }
            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_1() {
            let i = Image::from_str("0222112222120000", Size::new(2, 2));
            assert_eq!(i.render(), vec![0, 1, 1, 0]);
        }
    }
}

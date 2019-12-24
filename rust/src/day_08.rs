pub fn solve(part: u8, data: Result<String, std::io::Error>) {
    let s = part_1::Size::new(25, 6);
    let i = part_1::Image::from_str(data.expect("Could not read data file").as_str(), s);
    match part {
        1 => println!("{}", i.checksum().unwrap()),
        2 => (),
        _ => (),
    }
}

mod part_1 {

    pub struct Size {
        width: usize,
        height: usize,
    }

    impl Size {
        pub fn new(width: usize, height: usize) -> Self {
            Self { width, height }
        }
    }

    struct Layer<'a>(&'a [u8]);

    impl<'a> Layer<'a> {
        fn count(&self, px: u8) -> usize {
            self.0.iter().filter(|&&pix| pix == px).count()
        }
    }

    pub struct Image {
        size: Size,
        pixels: Vec<u8>,
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

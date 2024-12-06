use std::ops::Index;

#[derive(Debug)]
pub struct RefGrid<'a> {
    width: usize,
    height: usize,
    contents: &'a [u8],
}

impl<'a> RefGrid<'a> {
    pub fn from_str(input: &'a str) -> Self {
        let width = input.lines().next().unwrap().len();
        let height = input.len() / (width + 1);
        Self {
            width,
            height,
            contents: input.as_bytes(),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

impl Index<(usize, usize)> for RefGrid<'_> {
    type Output = u8;
    fn index(&self, (x, y): (usize, usize)) -> &u8 {
        assert!(x < self.width);
        &self.contents[y * (self.width + 1) + x]
    }
}
use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
pub struct Grid<T> {
    width: usize,
    height: usize,
    contents: Vec<T>,
}

impl<T: Default> Grid<T> {
    pub fn new(width: usize, height: usize) -> Self {
        let mut contents = Vec::with_capacity(width * height);

        for _ in 0..(width * height) {
            contents.push(Default::default());
        }

        Self {
            width,
            height,
            contents,
        }
    }
}

impl<T> Grid<T> {
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn contains(&self, (x, y): (usize, usize)) -> bool {
        x < self.width && y < self.height
    }

    pub fn iter(&self) -> impl Iterator<Item = ((usize, usize), &T)> {
        let width = self.width();
        self.contents.iter().enumerate().map(move |(n, x)| {
            let j = n / width;
            let i = n % width;
            ((i, j), x)
        })
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = ((usize, usize), &mut T)> {
        let width = self.width();
        self.contents.iter_mut().enumerate().map(move |(n, x)| {
            let j = n / width;
            let i = n % width;
            ((i, j), x)
        })
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;
    fn index(&self, (x, y): (usize, usize)) -> &T {
        assert!(x < self.width);
        &self.contents[y * self.width + x]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut T {
        assert!(x < self.width);
        &mut self.contents[y * self.width + x]
    }
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for ((x, _), v) in self.iter() {
            write!(f, "{v}")?;
            if x == self.width - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

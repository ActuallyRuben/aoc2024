use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, Clone)]
pub struct RefGrid<T> {
    width: usize,
    height: usize,
    contents: T,
}

impl<'a> RefGrid<&'a [u8]> {
    pub fn from_str(input: &'a str) -> Self {
        let width = input.lines().next().unwrap().len();
        let height = input.len() / (width + 1);
        Self {
            width,
            height,
            contents: input.as_bytes(),
        }
    }
}

impl RefGrid<Vec<u8>> {
    pub fn from_str_cloned(input: &str) -> Self {
        Self::from_string(input.to_string())
    }
    
    pub fn from_string(input: String) -> Self {
        let width = input.lines().next().unwrap().len();
        let height = input.len() / (width + 1);
        Self {
            width,
            height,
            contents: input.into_bytes(),
        }
    }

    #[allow(unused)]
    pub fn new(width: usize, height: usize) -> Self {
        let mut contents = vec![0; (width + 1) * height];
        for j in 0..height {
            contents[width + width * j] = b'\n';
        }
        Self {
            width,
            height,
            contents,
        }
    }
}

impl<T> RefGrid<T> {
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn contains(&self, (x, y): (usize, usize)) -> bool {
        x < self.width && y < self.height
    }

    pub fn iter(&self) -> impl Iterator<Item = ((usize, usize), &u8)>
    where
        T: Deref<Target = [u8]>,
    {
        let width = self.width();
        self.contents
            .iter()
            .enumerate()
            .map(move |(n, x)| {
                let j = n / (width + 1);
                let i = n % (width + 1);
                ((i, j), x)
            })
            .filter(move |((i, _), _)| *i < width)
    }

    #[allow(unused)]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = ((usize, usize), &mut u8)>
    where
        T: Deref<Target = [u8]> + DerefMut,
    {
        let width = self.width();
        self.contents
            .iter_mut()
            .enumerate()
            .map(move |(n, x)| {
                let j = n / (width + 1);
                let i = n % (width + 1);
                ((i, j), x)
            })
            .filter(move |((i, _), _)| *i < width)
    }
}

impl<T: Deref<Target = [u8]>> Index<(usize, usize)> for RefGrid<T> {
    type Output = u8;
    fn index(&self, (x, y): (usize, usize)) -> &u8 {
        assert!(x < self.width);
        &self.contents[y * (self.width + 1) + x]
    }
}

impl<T: DerefMut + Deref<Target = [u8]>> IndexMut<(usize, usize)> for RefGrid<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut u8 {
        assert!(x < self.width);
        &mut self.contents[y * (self.width + 1) + x]
    }
}

impl<T: Deref<Target = [u8]>> Display for RefGrid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let text = String::from_utf8_lossy(&self.contents);
        write!(f, "{text}")
    }
}

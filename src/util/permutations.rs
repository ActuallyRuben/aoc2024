pub trait Permutable: Iterator + Clone + Sized
where
    Self::Item: Copy,
{
    fn permutations(self) -> Permutations<Self> {
        Permutations {
            iter: self,
            state: None,
        }
    }
}
impl<I: Iterator + Clone> Permutable for I where I::Item: Copy {}

pub struct Permutations<I: Iterator + Clone>
where
    I::Item: Copy,
{
    iter: I,
    state: Option<(I::Item, I)>,
}

impl<I: Iterator + Clone> Iterator for Permutations<I>
where
    I::Item: Copy,
{
    type Item = (I::Item, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some((next_a, next_iter)) = &mut self.state {
                if let Some(next_b) = self.iter.next() {
                    return Some((*next_a, next_b));
                } else if let Some(next) = next_iter.next() {
                    *next_a = next;
                    self.iter = next_iter.clone();
                } else {
                    return None;
                }
            } else if let Some(next) = self.iter.next() {
                self.state = Some((next, self.iter.clone()))
            } else {
                return None
            }
        }
    }
}

#[test]
fn permutations() {
    let input = [1, 2, 3, 4, 5];
    let output: Vec<(i32, i32)> = input.into_iter().permutations().collect();
    assert_eq!(output, vec![(1, 2), (1, 3), (1, 4), (1, 5), (2, 3), (2, 4), (2, 5), (3, 4), (3, 5), (4, 5)])
}
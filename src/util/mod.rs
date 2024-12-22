mod direction;
mod grid;
mod permutations;
mod refgrid;
mod sortedvec;

pub use direction::Direction;
pub use grid::Grid;
pub use permutations::Permutable;
pub use refgrid::RefGrid;
use std::collections::LinkedList;
use std::ops::ControlFlow;

pub fn map_linkedlist<T>(
    list: &mut LinkedList<T>,
    mut f: impl FnMut(&mut LinkedList<T>) -> ControlFlow<()>,
) {
    let mut before = LinkedList::new();
    while !list.is_empty() {
        let mut after = list.split_off(1);
        let result = f(list);
        before.append(list);
        if result.is_continue() {
            *list = after;
        } else {
            before.append(&mut after);
        }
    }
    *list = before;
}

pub fn count_digits(mut value: usize) -> usize {
    let mut digits = 1;
    while value >= 10 {
        digits += 1;
        value /= 10;
    }
    digits
}

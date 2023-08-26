#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

use rand::prelude::*;

/// This trait provides the sorting function
pub trait RandSort<T> {
    /// Randsort is a next gen sorting function designed for computers of the 21st century requiring
    /// luck and faith in order to sort your Vec
    fn randsort(&mut self);
}

/// The randsort implementation for Vec<T: PartialOrd>
impl<T: PartialOrd> RandSort<T> for Vec<T> {
    fn randsort(&mut self) {
        let mut rng = rand::thread_rng();
        loop {
            self.shuffle(&mut rng);
            let mut is_sorted = true;
            for i in 1..self.len() {
                if self[i] < self[i - 1] {
                    is_sorted = false;
                    break;
                }
            }
            if is_sorted {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut vec = vec![5, 4, 8, 9, 12, 1, 3909, 567, 5, 6];
        vec.randsort();
        assert_eq!(vec, [1, 4, 5, 5, 6, 8, 9, 12, 567, 3909]);
    }
}

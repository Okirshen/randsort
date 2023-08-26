use rand::prelude::*;

pub trait RandSort<T> {
    fn rand_sort(&mut self);
}

impl<T: PartialOrd> RandSort<T> for Vec<T> {
    fn rand_sort(&mut self) {
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
        vec.rand_sort();
        assert_eq!(vec, [1, 4, 5, 5, 6, 8, 9, 12, 567, 3909]);
    }
}

/// Trait to add permutation functionality to a type.
/// Will return an iterator over all permutations of the type.
pub trait Permutable<T> {
    /// Create a permutation iterator over the type.
    fn permute(&self) -> Permuter<T>;
}

/// Iterator over all permutations of a vector.
pub struct Permuter<T> {
    /// Stackpointer for flatteing of the revcursive Heap's algorithm.
    i: usize,

    /// Stack of indices for flattening the recursive Heap's algorithm.
    c: Vec<usize>,

    /// Last returned permutation of the vector
    v: Vec<T>,
}

/// Implement the permutation iterator
impl<T> Permuter<T> {
    /// Create a new permutation iterator over a vector.
    fn new(v: Vec<T>) -> Self {
        Self {
            i: 0, // Initialize to 0 to be able to return the original vector
            c: vec![0; v.len()],
            v,
        }
    }
}

/// Implement the iterator trait for the permutation iterator
impl<T> Iterator for Permuter<T>
where
    T: Clone,
{
    type Item = Vec<T>;

    /// Return the next permutation of the vector.
    fn next(&mut self) -> Option<Self::Item> {
        // see Heap's algorithm:
        // https://en.wikipedia.org/wiki/Heap%27s_algorithm

        if self.i == 0 {
            self.i = 1;
            return Some(self.v.clone());
        }

        while self.i < self.v.len() {
            if self.c[self.i] < self.i {
                if self.i % 2 == 0 {
                    self.v.swap(0, self.i);
                } else {
                    self.v.swap(self.c[self.i], self.i);
                }

                self.c[self.i] += 1;
                self.i = 1;

                return Some(self.v.clone());
            } else {
                self.c[self.i] = 0;
                self.i += 1;
            }
        }

        None
    }
}

/// Implement the permutation trait for vectors.
impl<T> Permutable<T> for Vec<T>
where
    T: Clone,
{
    /// Create a permutation iterator over a vector.
    fn permute(&self) -> Permuter<T> {
        Permuter::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permute() {
        let v = vec![1, 2, 3];
        let mut p = v.permute();

        assert_eq!(p.next(), Some(vec![1, 2, 3]));
        assert_eq!(p.next(), Some(vec![2, 1, 3]));
        assert_eq!(p.next(), Some(vec![3, 1, 2]));
        assert_eq!(p.next(), Some(vec![1, 3, 2]));
        assert_eq!(p.next(), Some(vec![2, 3, 1]));
        assert_eq!(p.next(), Some(vec![3, 2, 1]));
        assert_eq!(p.next(), None);
    }
}

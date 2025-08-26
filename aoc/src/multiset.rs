pub trait Multiset<T> {
    fn multiset(&self, length: usize) -> MultisetIterator<'_, T>;
}

pub struct MultisetIterator<'a, T> {
    set: &'a [T],
    indices: Vec<usize>,
    done: bool,
}

impl<T> Iterator for MultisetIterator<'_, T>
where
    T: Copy,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let indices = self.indices.clone();

        let mut index = 0;
        loop {
            self.indices[index] += 1;
            if self.indices[index] == self.set.len() {
                self.indices[index] = 0;
                index += 1;
                if index == self.indices.len() {
                    self.done = true;
                    break;
                }
            } else {
                break;
            }
        }

        Some(indices.iter().map(|&i| self.set[i]).collect())
    }
}

impl<T> Multiset<T> for [T] {
    fn multiset(&self, length: usize) -> MultisetIterator<'_, T> {
        MultisetIterator {
            set: self,
            indices: vec![0; length],
            done: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiset() {
        let set = [1, 2, 3];
        let mut multiset = set.multiset(3);
        assert_eq!(multiset.next(), Some(vec![1, 1, 1]));
        assert_eq!(multiset.next(), Some(vec![2, 1, 1]));
        assert_eq!(multiset.next(), Some(vec![3, 1, 1]));
        assert_eq!(multiset.next(), Some(vec![1, 2, 1]));
        assert_eq!(multiset.next(), Some(vec![2, 2, 1]));
        assert_eq!(multiset.next(), Some(vec![3, 2, 1]));
        assert_eq!(multiset.next(), Some(vec![1, 3, 1]));
        assert_eq!(multiset.next(), Some(vec![2, 3, 1]));
        assert_eq!(multiset.next(), Some(vec![3, 3, 1]));
        assert_eq!(multiset.next(), Some(vec![1, 1, 2]));
        assert_eq!(multiset.next(), Some(vec![2, 1, 2]));
        assert_eq!(multiset.next(), Some(vec![3, 1, 2]));
        assert_eq!(multiset.next(), Some(vec![1, 2, 2]));
        assert_eq!(multiset.next(), Some(vec![2, 2, 2]));
        assert_eq!(multiset.next(), Some(vec![3, 2, 2]));
        assert_eq!(multiset.next(), Some(vec![1, 3, 2]));
        assert_eq!(multiset.next(), Some(vec![2, 3, 2]));
        assert_eq!(multiset.next(), Some(vec![3, 3, 2]));
        assert_eq!(multiset.next(), Some(vec![1, 1, 3]));
        assert_eq!(multiset.next(), Some(vec![2, 1, 3]));
        assert_eq!(multiset.next(), Some(vec![3, 1, 3]));
        assert_eq!(multiset.next(), Some(vec![1, 2, 3]));
        assert_eq!(multiset.next(), Some(vec![2, 2, 3]));
        assert_eq!(multiset.next(), Some(vec![3, 2, 3]));
        assert_eq!(multiset.next(), Some(vec![1, 3, 3]));
        assert_eq!(multiset.next(), Some(vec![2, 3, 3]));
        assert_eq!(multiset.next(), Some(vec![3, 3, 3]));
        assert_eq!(multiset.next(), None);
    }
}

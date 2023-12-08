pub trait Permutable<T> {
    fn permute(&self) -> Permuter<T>;
}

pub struct Permuter<T> {
    // see Heap's algorithm:
    // https://en.wikipedia.org/wiki/Heap%27s_algorithm
    i: usize,
    c: Vec<usize>,
    v: Vec<T>,
}

impl<T> Permuter<T> {
    pub fn new(v: Vec<T>) -> Self {
        Self {
            i: 0,
            c: vec![0; v.len()],
            v,
        }
    }
}

impl<T> Iterator for Permuter<T>
where
    T: Clone,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
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

impl<T> Permutable<T> for Vec<T>
where
    T: Clone,
{
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

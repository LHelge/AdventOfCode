pub trait Pairable<T> {
    fn pairs(&self) -> Pairer<T>;
}

pub struct Pairer<T> {
    v: Vec<T>,
    i: usize,
    j: usize,
}

impl<T> Pairer<T> {
    pub fn new(v: Vec<T>) -> Self {
        Self { v, i: 0, j: 1 }
    }
}

impl<T> Iterator for Pairer<T>
where
    T: Clone,
{
    type Item = (T, T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.v.len() {
            return None;
        }

        if self.j >= self.v.len() {
            self.i += 1;
            self.j = self.i + 1;
        }

        if self.j >= self.v.len() {
            return None;
        }

        let pair = (self.v[self.i].clone(), self.v[self.j].clone());
        self.j += 1;

        Some(pair)
    }
}

impl<T> Pairable<T> for Vec<T>
where
    T: Clone,
{
    fn pairs(&self) -> Pairer<T> {
        Pairer::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pairs() {
        let v: Vec<i32> = vec![1, 2, 3, 4, 5];
        let mut pairs = v.pairs();

        assert_eq!(pairs.next(), Some((1, 2)));
        assert_eq!(pairs.next(), Some((1, 3)));
        assert_eq!(pairs.next(), Some((1, 4)));
        assert_eq!(pairs.next(), Some((1, 5)));
        assert_eq!(pairs.next(), Some((2, 3)));
        assert_eq!(pairs.next(), Some((2, 4)));
        assert_eq!(pairs.next(), Some((2, 5)));
        assert_eq!(pairs.next(), Some((3, 4)));
        assert_eq!(pairs.next(), Some((3, 5)));
        assert_eq!(pairs.next(), Some((4, 5)));
        assert_eq!(pairs.next(), None);
    }
}

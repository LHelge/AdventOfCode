pub trait Pairable<T> {
    fn pairs(&self) -> Pairs<'_, T>;
}

pub struct Pairs<'a, T> {
    s: &'a [T],
    i: usize,
    j: usize,
}

impl<'a, T> Iterator for Pairs<'a, T> {
    type Item = (&'a T, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.j >= self.s.len() {
            self.i += 1;
            self.j = self.i + 1;
            if self.j >= self.s.len() {
                return None;
            }
        }
        let pair = (&self.s[self.i], &self.s[self.j]);
        self.j += 1;
        Some(pair)
    }
}

impl<T> Pairable<T> for [T] {
    fn pairs(&self) -> Pairs<'_, T> {
        Pairs {
            s: self,
            i: 0,
            j: 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pairs() {
        let v: Vec<i32> = vec![1, 2, 3, 4, 5];
        let mut pairs = v.pairs();

        assert_eq!(pairs.next(), Some((&1, &2)));
        assert_eq!(pairs.next(), Some((&1, &3)));
        assert_eq!(pairs.next(), Some((&1, &4)));
        assert_eq!(pairs.next(), Some((&1, &5)));
        assert_eq!(pairs.next(), Some((&2, &3)));
        assert_eq!(pairs.next(), Some((&2, &4)));
        assert_eq!(pairs.next(), Some((&2, &5)));
        assert_eq!(pairs.next(), Some((&3, &4)));
        assert_eq!(pairs.next(), Some((&3, &5)));
        assert_eq!(pairs.next(), Some((&4, &5)));
        assert_eq!(pairs.next(), None);
    }
}

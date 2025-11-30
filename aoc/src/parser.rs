use super::error::*;
use std::str::FromStr;

pub trait Parser<T> {
    fn parse_delimited(&self, delimiter: char) -> Result<Vec<T>>;
    fn parse_lines(&self) -> Result<Vec<T>>;
}

impl<T, E> Parser<T> for &str
where
    T: FromStr<Err = E>,
    E: Into<AoCError>,
{
    fn parse_delimited(&self, delimiter: char) -> Result<Vec<T>> {
        self.split(delimiter)
            .map(|s| s.parse().map_err(Into::into))
            .collect()
    }

    fn parse_lines(&self) -> Result<Vec<T>> {
        self.lines()
            .map(|s| s.parse().map_err(Into::into))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delimited() {
        let s = "1,2,3,4";
        let v1: Vec<u8> = s.parse_delimited(',').unwrap();
        let v2 = vec![1u8, 2u8, 3u8, 4u8];
        assert_eq!(v1, v2);
    }

    #[test]
    fn lines() {
        let s = r#"1
2
3
4"#;
        let v1: Vec<u8> = s.parse_lines().unwrap();
        let v2 = vec![1u8, 2u8, 3u8, 4u8];
        assert_eq!(v1, v2);
    }
}

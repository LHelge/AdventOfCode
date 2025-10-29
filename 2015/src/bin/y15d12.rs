const YEAR: u16 = 2015;
const DAY: u8 = 12;
use std::collections::HashMap;

use aoc::*;

#[derive(Debug)]
enum JsonToken {
    LeftBrace,
    RightBrace,
    LeftSquareBrace,
    RightSquareBrace,
    Colon,
    Period,
    StringLiteral(String),
    NumericLiteral(i64),
    EndOfString,
}

#[derive(Debug)]
enum JsonValue {
    Numeric(i64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

impl JsonValue {
    fn sum(&self) -> i64 {
        match self {
            JsonValue::Numeric(num) => *num,
            JsonValue::String(_) => 0,
            JsonValue::Array(a) => a.iter().map(|v| v.sum()).sum(),
            JsonValue::Object(o) => o.values().map(|v| v.sum()).sum(),
        }
    }

    fn sum_non_red(&self) -> i64 {
        match self {
            JsonValue::Numeric(num) => *num,
            JsonValue::String(_) => 0,
            JsonValue::Array(a) => a.iter().map(|v| v.sum_non_red()).sum(),
            JsonValue::Object(o) => {
                if o.values()
                    .any(|v| matches!(v, JsonValue::String(s) if s == "red"))
                {
                    0
                } else {
                    o.values().map(|v| v.sum_non_red()).sum()
                }
            }
        }
    }
}

struct Scanner {
    json: Vec<char>,
    current: usize,
}

impl Iterator for Scanner {
    type Item = (char, Option<char>);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(c) = self.json.get(self.current) {
            self.current += 1;
            Some((*c, self.json.get(self.current).copied()))
        } else {
            None
        }
    }
}

impl Scanner {
    fn new(input: &str) -> Self {
        Self {
            json: input.chars().collect(),
            current: 0,
        }
    }

    fn tokens(&mut self) -> Result<Vec<JsonToken>> {
        let mut tokens = Vec::new();

        while let Some((curr, next)) = self.next() {
            let token = match (curr, next) {
                (c, _) if c.is_whitespace() => None,

                ('{', _) => Some(JsonToken::LeftBrace),
                ('}', _) => Some(JsonToken::RightBrace),
                ('[', _) => Some(JsonToken::LeftSquareBrace),
                (']', _) => Some(JsonToken::RightSquareBrace),
                (':', _) => Some(JsonToken::Colon),
                (',', _) => Some(JsonToken::Period),

                ('"', _) => Some(self.scan_string_litteral()?),
                ('-', Some(next)) if next.is_numeric() => {
                    Some(self.scan_numeric_litteral('-', Some(next))?)
                }
                (curr, next) if curr.is_numeric() => Some(self.scan_numeric_litteral(curr, next)?),

                (c, _) => {
                    println!("unexpected char: {c}");
                    Err(AoCError::BadInput)?
                }
            };

            if let Some(token) = token {
                tokens.push(token);
            }
        }

        tokens.push(JsonToken::EndOfString);

        Ok(tokens)
    }

    fn scan_numeric_litteral(&mut self, curr: char, next: Option<char>) -> Result<JsonToken> {
        let mut number = String::from(curr);

        if next.is_some_and(|n| n.is_numeric()) {
            for (curr, next) in self.by_ref() {
                number.push(curr);
                if !next.is_some_and(|n| n.is_numeric()) {
                    break;
                }
            }
        }

        Ok(JsonToken::NumericLiteral(number.parse()?))
    }

    fn scan_string_litteral(&mut self) -> Result<JsonToken> {
        let mut value = String::new();

        for (curr, _) in self.by_ref() {
            if curr == '"' {
                break;
            }
            value.push(curr);
        }

        Ok(JsonToken::StringLiteral(value))
    }
}

type ResultType = i64;
type DataType<'a> = JsonValue;

fn parse_array(tokens: &mut impl Iterator<Item = JsonToken>) -> Result<JsonValue> {
    let mut array = Vec::new();

    loop {
        match tokens.next() {
            Some(JsonToken::RightSquareBrace) => break,
            Some(JsonToken::Period) => continue, // Skip commas
            Some(JsonToken::LeftBrace) => array.push(parse_object(tokens)?),
            Some(JsonToken::LeftSquareBrace) => array.push(parse_array(tokens)?),
            Some(JsonToken::NumericLiteral(n)) => array.push(JsonValue::Numeric(n)),
            Some(JsonToken::StringLiteral(s)) => array.push(JsonValue::String(s)),
            _ => Err(AoCError::BadInput)?,
        }
    }

    Ok(JsonValue::Array(array))
}

fn parse_object(tokens: &mut impl Iterator<Item = JsonToken>) -> Result<JsonValue> {
    let mut object = HashMap::new();

    loop {
        match tokens.next() {
            Some(JsonToken::RightBrace) => break,
            Some(JsonToken::Period) => continue, // Skip commas
            Some(JsonToken::StringLiteral(key)) => {
                // Expect colon
                if !matches!(tokens.next(), Some(JsonToken::Colon)) {
                    Err(AoCError::BadInput)?;
                }

                // Parse value
                let value = match tokens.next() {
                    Some(JsonToken::LeftBrace) => parse_object(tokens)?,
                    Some(JsonToken::LeftSquareBrace) => parse_array(tokens)?,
                    Some(JsonToken::NumericLiteral(n)) => JsonValue::Numeric(n),
                    Some(JsonToken::StringLiteral(s)) => JsonValue::String(s),
                    _ => Err(AoCError::BadInput)?,
                };

                object.insert(key, value);
            }
            _ => Err(AoCError::BadInput)?,
        }
    }

    Ok(JsonValue::Object(object))
}

fn parse(input: &str) -> Result<DataType<'_>> {
    let mut tokens = Scanner::new(input).tokens()?.into_iter();

    Ok(match tokens.next() {
        Some(JsonToken::LeftBrace) => parse_object(&mut tokens)?,
        Some(JsonToken::LeftSquareBrace) => parse_array(&mut tokens)?,
        _ => Err(AoCError::BadInput)?,
    })
}

fn task1(data: &DataType) -> Result<ResultType> {
    Ok(data.sum())
}

fn task2(data: &DataType) -> Result<ResultType> {
    Ok(data.sum_non_red())
}

fn main() -> Result<()> {
    let mut solution = Solution::<ResultType, DataType<'_>>::new(&parse, &task1, &task2);
    solution.solve_for_answer(YEAR, DAY)?;

    println!("Advent of Code {YEAR} day {DAY}");
    println!("-------------------------");
    println!("{solution}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples1() {
        let mut solution = Solution::<ResultType, DataType>::new(&parse, &task1, &task2);

        let (task1, _) = solution.solve_for_test("[1,2,3]").unwrap();
        assert_eq!(task1, Some(6));

        let (task1, _) = solution.solve_for_test(r#"{"a":2,"b":4}"#).unwrap();
        assert_eq!(task1, Some(6));

        let (task1, _) = solution.solve_for_test(r#"[[[3]]]"#).unwrap();
        assert_eq!(task1, Some(3));

        let (task1, _) = solution.solve_for_test(r#"{"a":{"b":4},"c":-1}"#).unwrap();
        assert_eq!(task1, Some(3));

        let (task1, _) = solution.solve_for_test(r#"{"a":[-1,1]}"#).unwrap();
        assert_eq!(task1, Some(0));

        let (task1, _) = solution.solve_for_test(r#"[-1,{"a":1}]"#).unwrap();
        assert_eq!(task1, Some(0));

        let (task1, _) = solution.solve_for_test(r#"[]"#).unwrap();
        assert_eq!(task1, Some(0));

        let (task1, _) = solution.solve_for_test(r#"{}"#).unwrap();
        assert_eq!(task1, Some(0));
    }

    #[test]
    fn examples2() {
        let mut solution = Solution::<ResultType, DataType>::new(&parse, &task1, &task2);

        let (_, task2) = solution.solve_for_test("[1,2,3]").unwrap();
        assert_eq!(task2, Some(6));

        let (_, task2) = solution
            .solve_for_test(r#"[1,{"c":"red","b":2},3]"#)
            .unwrap();
        assert_eq!(task2, Some(4));

        let (_, task2) = solution
            .solve_for_test(r#"{"d":"red","e":[1,2,3,4],"f":5}"#)
            .unwrap();
        assert_eq!(task2, Some(0));

        let (_, task2) = solution.solve_for_test(r#"[1,"red",5]"#).unwrap();
        assert_eq!(task2, Some(6));
    }
}

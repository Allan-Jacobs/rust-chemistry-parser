use super::token_types::{ParenType, Tokens};
use std::{iter::{Iterator, Peekable}, str::Chars};

pub struct LazyTokenStream<'a> {
    string_iter: Peekable<Chars<'a>>,
}

impl <'a> LazyTokenStream<'a> {
    pub fn new(string: &'a String) -> Self {
        Self {
            string_iter: string.chars().peekable()
        }
    }
}

impl From<LazyTokenStream<'_>> for Result<Vec<Box<Tokens>>, String> {
    fn from(other: LazyTokenStream<'_>) -> Self {
        let mut vec = Vec::new();
        for token in other {
            if token.is_err() {return Err(token.err().unwrap())};
            vec.push(token.unwrap());
        }
        return Ok(vec);
    }
}

impl Iterator for LazyTokenStream<'_> {
    type Item = Result<Box<Tokens>, String>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.string_iter.next() {
            // numbers
            Some(val @ '0'..='9') => {
                let mut temp = String::new();
                temp.push(val);
                loop {
                    match self.string_iter.peek() {
                        Some(inner_val @ '0'..='9') => {
                            temp.push(*inner_val);
                            self.string_iter.next();
                        },
                        Some(_) | None => { break Some(temp.parse::<u16>()
                            .map_err(|e| format!("Could not parse number: {e}"))
                            .map(|value| Box::new(Tokens::Number(value))));
                        }
                    }
                }
            },

            // parens
            Some('(') => Some(Ok(Box::new(Tokens::Paren(ParenType::OPEN)))),
            Some(')') => Some(Ok(Box::new(Tokens::Paren(ParenType::CLOSE)))),

            // plus
            Some('+') => Some(Ok(Box::new(Tokens::Plus))),

            // yields
            Some('-') => {
                match self.string_iter.peek() {
                    Some('>') => {
                        self.string_iter.next();
                        Some(Ok(Box::new(Tokens::Yields)))
                    },
                    Some(_) => Some(Err("Yield sign (->) unfinished".to_owned())),
                    None => None
                }
            }

            // elements
            Some(val @ 'A'..='Z') => {
                let mut temp = String::new();
                temp.push(val);

                if let Some(inner_val @'a'..='z') = self.string_iter.peek() {
                    temp.push(*inner_val);
                    self.string_iter.next();
                    if let Some('a'..='z') = self.string_iter.peek() { // should not have 3 letter element names
                        return Some(Err("Formula should not have 3 letter element names".to_owned()));
                    };
                }
                
                Some(Ok(Box::new(Tokens::Element(temp))))
            },

            Some(c) => Some(Err(format!("Invalid Character: {}", c))),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_parse_simple_element() {
        let to_parse = &String::from("Fe");
        let stream = LazyTokenStream::new(to_parse);

        let res = Result::from(stream);

        assert!(res.is_ok(), "An error occurred while parsing");

        let exp = vec!(Box::new(Tokens::Element("Fe".to_owned())));
        
        assert_eq!(exp, res.unwrap());
    }

    #[test]
    fn can_parse_compound_with_subscript_and_coeffiecient() {
        let to_parse = &String::from("2FeCO3");
        let stream = LazyTokenStream::new(to_parse);

        let res = Result::from(stream);

        assert!(res.is_ok(), "An error occurred while parsing");

        let exp = vec!(
            Box::new(Tokens::Number(2)),
            Box::new(Tokens::Element("Fe".to_owned())),
            Box::new(Tokens::Element("C".to_owned())),
            Box::new(Tokens::Element("O".to_owned())),
            Box::new(Tokens::Number(3)),
        );
        
        assert_eq!(exp, res.unwrap());
    }

    #[test]
    fn can_parse_forumula() {
        let to_parse = &String::from("2Fe+Na2F3->2FeNa+F3");
        let stream = LazyTokenStream::new(to_parse);

        let res = Result::from(stream);

        assert!(res.is_ok(), "An error occurred while parsing: {}", res.err().unwrap() );

        let exp = vec!(
            Box::new(Tokens::Number(2)),
            Box::new(Tokens::Element("Fe".to_owned())),
            Box::new(Tokens::Plus),
            Box::new(Tokens::Element("Na".to_owned())),
            Box::new(Tokens::Number(2)),
            Box::new(Tokens::Element("F".to_owned())),
            Box::new(Tokens::Number(3)),
            Box::new(Tokens::Yields),
            Box::new(Tokens::Number(2)),
            Box::new(Tokens::Element("Fe".to_owned())),
            Box::new(Tokens::Element("Na".to_owned())),
            Box::new(Tokens::Plus),
            Box::new(Tokens::Element("F".to_owned())),
            Box::new(Tokens::Number(3)),
        );
        
        assert_eq!(exp, res.unwrap());
    }
}
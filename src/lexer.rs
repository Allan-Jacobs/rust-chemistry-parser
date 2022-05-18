use crate::token_types::{ParenType, Tokens, TokenMetadata};
use std::{iter::{Iterator, Peekable, Enumerate}, str::Chars};

/// An iterator that iterates over a string and parses it lazily
pub struct LazyTokenStream<'a> {
    string_iter: Peekable<Enumerate<Chars<'a>>>,
}

impl <'a> LazyTokenStream<'a> {
    /// Construct a new `LazyTokenStream` from the string
    pub fn new(string: &'a String) -> Self {
        Self {
            string_iter: string.chars().enumerate().peekable()
        }
    }
}

impl From<LazyTokenStream<'_>> for Result<Vec<Tokens>, String> {
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
    type Item = Result<Tokens, String>;

    fn next(&mut self) -> Option<Self::Item> {
        let t = self.string_iter.enumerate(); 

        match self.string_iter.next() {
            // numbers
            Some((loc,val @ '0'..='9')) => {
                let mut temp = String::new();
                temp.push(val);
                loop {
                    match self.string_iter.peek() {
                        Some((_,inner_val @ '0'..='9')) => {
                            temp.push(*inner_val);
                            self.string_iter.next();
                        },
                        Some(_) | None => {
                            break Some(temp.parse::<u16>()
                            .map_err(|e| format!("Could not parse number: {e}"))
                            .map(|value| Tokens::Number { data: value, meta: TokenMetadata::new(&temp, loc) }));
                        }
                    }
                }
            },

            // parens
            Some((loc, raw @ '(')) => Some(Ok(Tokens::Paren { data: ParenType::OPEN, meta: TokenMetadata::new(&raw.to_string(), loc) })),
            Some((loc, raw @ ')')) => Some(Ok(Tokens::Paren { data: ParenType::CLOSE, meta: TokenMetadata::new(&raw.to_string(), loc) })),

            // plus
            Some((loc, raw @ '+')) => Some(Ok(Tokens::Plus { meta: TokenMetadata::new(&raw.to_string(), loc) })),

            // yields
            Some((loc, '-')) => {
                match self.string_iter.peek() {
                    Some((_, '>')) => {
                        self.string_iter.next();
                        Some(Ok(Tokens::Yields { meta: TokenMetadata::new("->", loc) }))
                    },
                    Some(_) => Some(Err("Yield sign (->) unfinished".to_owned())),
                    None => None
                }
            }

            // elements
            Some((loc, val @ 'A'..='Z')) => {
                let mut temp = String::new();
                temp.push(val);

                if let Some((_, inner_val @'a'..='z')) = self.string_iter.peek() {
                    temp.push(*inner_val);
                    self.string_iter.next();
                    if let Some((_, 'a'..='z')) = self.string_iter.peek() { // should not have 3 letter element names
                        return Some(Err("Formula should not have 3 letter element names".to_owned()));
                    };
                }
                
                Some(Ok(Tokens::Element { data: temp, meta: TokenMetadata::new(&temp, loc) }))
            },

            Some((_, c)) => Some(Err(format!("Invalid Character: {}", c))),
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

        let exp = vec!(Tokens::Element { data: "Fe".to_owned(), meta: TokenMetadata::new("Fe",0)});
        
        assert_eq!(exp, res.unwrap());
    }

    #[test]
    fn can_parse_compound_with_subscript_and_coeffiecient() {
        let to_parse = &String::from("2FeCO3");
        let stream = LazyTokenStream::new(to_parse);

        let res = Result::from(stream);

        assert!(res.is_ok(), "An error occurred while parsing");

        let exp = vec!(
            Tokens::Number{ data: 2, meta: TokenMetadata::new("2", 0) },
            Tokens::Element{ data: "Fe".to_owned(), meta: TokenMetadata::new("Fe", 1)},
            Tokens::Element{ data: "C".to_owned(), meta: TokenMetadata::new("C", 3)},
            Tokens::Element{ data: "O".to_owned(), meta: TokenMetadata::new("O", 4)},
            Tokens::Number{ data: 3, meta: TokenMetadata::new("3", 5)},
        );
        
        assert_eq!(exp, res.unwrap());
    }

    #[test]
    fn can_parse_forumula() {
        let to_parse = &String::from("2Fe+Na2F3->2FeNa+F3");
        let stream = LazyTokenStream::new(to_parse);

        let res = Result::from(stream);

        assert!(res.is_ok(), "An error occurred while parsing: {}", res.err().unwrap() );

        let exp = vec![
            Tokens::Number{ data: 2, meta: TokenMetadata::new("2", 0)},
            Tokens::Element{ data: "Fe".to_owned(), meta: TokenMetadata::new("Fe", 1)},
            Tokens::Plus{ meta: TokenMetadata::new("+", 3) },
            Tokens::Element{ data: "Na".to_owned(), meta: TokenMetadata::new("Na", 4)},
            Tokens::Number{ data: 2, meta: TokenMetadata::new("2", 6)},
            Tokens::Element{ data: "F".to_owned(), meta: TokenMetadata::new("F", 7)},
            Tokens::Number{ data: 3, meta: TokenMetadata::new("3", 8) },
            Tokens::Yields{ meta: TokenMetadata::new("->", 9)},
            Tokens::Number{ data: 2, meta: TokenMetadata::new("2", 11)},
            Tokens::Element{ data: "Fe".to_owned(), meta: TokenMetadata::new("Fe", 12)},
            Tokens::Element{ data: "Na".to_owned(), meta: TokenMetadata::new("Na", 14)},
            Tokens::Plus{ meta: TokenMetadata::new("+", 16) },
            Tokens::Element{ data: "F".to_owned(), meta: TokenMetadata::new("F", 17)},
            Tokens::Number{ data: 3, meta: TokenMetadata::new("3", 17)},
        ];
        
        assert_eq!(exp, res.unwrap());
    }
}
use crate::ast_types::Node;
use crate::token_types::Tokens;

/// Using an iterator (usually `LazyTokenStream`), parse tokens and return a result with the root node
pub fn parse<'a, T: Iterator<Item = Result<Tokens, String>>>(
    stream: T,
) -> Result<Box<Node>, String> {
    let mut stream = stream.peekable();
    let mut paren_level = 0;
    let mut current_stack = vec![Box::new(Node::ForumulaUnit(1, vec![]))];
    loop {
        match stream.next() {
            Some(Err(val)) => return Err(val),
            Some(Ok(Tokens::Number { data, meta })) => {
                let fu = current_stack.pop().unwrap();
                if let box Node::ForumulaUnit(_, vec) = fu {
                    current_stack.push(Box::new(Node::ForumulaUnit(data, vec)));
                } else {
                    return Err("Invalid parent".to_owned());
                }
            }
            Some(Ok(Tokens::Element { data, meta })) => {
                let mut fu = *current_stack.pop().unwrap();

                if let Node::ForumulaUnit(_, ref mut vec) = fu {
                    if let Some(Ok(Tokens::Number { data: count, meta })) = stream.peek() {
                        vec.push(Node::Element(*count, data));
                        stream.next();
                    } else {
                        vec.push(Node::Element(1, data));
                    }
                    current_stack.push(Box::new(fu));
                } else {
                    return Err("Invalid Parent".to_owned());
                }
            }
            Some(Ok(Tokens::Plus { meta })) => {
                let fu = *current_stack.pop().unwrap();
                let mut maybe_reactants = current_stack.pop().map(|val| *val);
                match maybe_reactants {
                    Some(Node::Reactants(ref mut vec)) => {
                        vec.push(fu);
                        current_stack.push(Box::new(maybe_reactants.unwrap()));
                        current_stack.push(Box::new(Node::ForumulaUnit(1, vec![])));
                    }
                    Some(Node::Products(ref mut vec)) => {
                        vec.push(fu);
                        current_stack.push(Box::new(maybe_reactants.unwrap()));
                        current_stack.push(Box::new(Node::ForumulaUnit(1, vec![])));
                    }
                    Some(_) => return Err("Invalid plus".to_owned()),
                    None => {
                        if let Node::ForumulaUnit(_, _) = fu {
                            current_stack.push(Box::new(Node::Reactants(vec![fu])));
                            current_stack.push(Box::new(Node::ForumulaUnit(1, vec![])));
                        }
                    }
                }
            }
            Some(Ok(Tokens::Yields { meta })) => {
                let fu = *current_stack.pop().unwrap();
                let mut maybe_reactants = current_stack.pop();

                match maybe_reactants {
                    Some(box Node::Reactants(ref mut vec)) => {
                        vec.push(fu);
                        current_stack.push(maybe_reactants.unwrap());
                        current_stack.push(Box::new(Node::Products(vec![])));
                        current_stack.push(Box::new(Node::ForumulaUnit(1, vec![])));
                    }
                    Some(_) => return Err("Invalid yields location".to_owned()),
                    None => {
                        current_stack.push(Box::new(Node::Reactants(vec![fu])));
                        current_stack.push(Box::new(Node::ForumulaUnit(1, vec![])));
                    }
                }
            }
            Some(Ok(Tokens::Paren {
                data: super::token_types::ParenType::OPEN,
                meta,
            })) => {
                paren_level += 1;
                current_stack.push(Box::new(Node::Group(1, vec![])));
            }
            Some(Ok(Tokens::Paren {
                data: super::token_types::ParenType::CLOSE,
                meta,
            })) => {
                if paren_level == 0 {
                    return Err("Invalid closing paren".to_owned());
                };
                paren_level -= 1;
                let group = *current_stack.pop().unwrap();
                let mut maybe_fu_or_group = current_stack.pop();

                match maybe_fu_or_group {
                    Some(box Node::Group(_, ref mut vec)) => {
                        if let Some(Ok(Tokens::Number { data, meta: _ })) = stream.peek() {
                            if let Node::Group(_, inner_vec) = group {
                                vec.push(Node::Group(*data, inner_vec));
                            }
                        }
                        current_stack.push(maybe_fu_or_group.unwrap());
                    }
                    Some(box Node::ForumulaUnit(_, ref mut vec)) => {
                        vec.push(group);
                        current_stack.push(maybe_fu_or_group.unwrap());
                    }

                    Some(_) => return Err("Invalid Parent".to_owned()),
                    None => return Err("Stack Underflow in group".to_owned()),
                }
            }
            None => {
                let fu = *current_stack.pop().unwrap();
                let mut products_or_none = current_stack.pop();
                match products_or_none {
                    Some(box Node::Products(ref mut vec)) => {
                        let reactants = current_stack.pop().unwrap();
                        vec.push(fu);
                        current_stack.push(Box::new(Node::Equation(
                            reactants,
                            products_or_none.unwrap(),
                        )));
                    }
                    Some(_) => return Err("Stack had unexpected value".to_owned()),
                    None => {
                        current_stack.push(Box::new(fu));
                    }
                }
                break;
            }
        }
    }
    Ok(current_stack.pop().unwrap())
}

#[cfg(test)]
mod tests {
    use crate::token_types::TokenMetadata;

    use super::*;
    #[test]
    fn can_parse_equation() {
        let stream = vec![
            Tokens::Number {
                data: 2,
                meta: TokenMetadata::new("2", 0),
            },
            Tokens::Element {
                data: "Fe".to_owned(),
                meta: TokenMetadata::new("Fe", 1),
            },
            Tokens::Plus {
                meta: TokenMetadata::new("+", 3),
            },
            Tokens::Element {
                data: "Na".to_owned(),
                meta: TokenMetadata::new("Na", 4),
            },
            Tokens::Number {
                data: 2,
                meta: TokenMetadata::new("2", 6),
            },
            Tokens::Element {
                data: "F".to_owned(),
                meta: TokenMetadata::new("F", 7),
            },
            Tokens::Number {
                data: 3,
                meta: TokenMetadata::new("3", 8),
            },
            Tokens::Yields {
                meta: TokenMetadata::new("->", 9),
            },
            Tokens::Number {
                data: 2,
                meta: TokenMetadata::new("2", 11),
            },
            Tokens::Element {
                data: "Fe".to_owned(),
                meta: TokenMetadata::new("Fe", 12),
            },
            Tokens::Element {
                data: "Na".to_owned(),
                meta: TokenMetadata::new("Na", 14),
            },
            Tokens::Plus {
                meta: TokenMetadata::new("+", 16),
            },
            Tokens::Element {
                data: "F".to_owned(),
                meta: TokenMetadata::new("F", 17),
            },
            Tokens::Number {
                data: 3,
                meta: TokenMetadata::new("3", 18),
            },
        ];

        let exp = Node::Equation(
            Box::new(Node::Reactants(vec![
                Node::ForumulaUnit(2, vec![Node::Element(1, "Fe".to_owned())]),
                Node::ForumulaUnit(
                    1,
                    vec![
                        Node::Element(2, "Na".to_owned()),
                        Node::Element(3, "F".to_owned()),
                    ],
                ),
            ])),
            Box::new(Node::Products(vec![
                Node::ForumulaUnit(
                    2,
                    vec![
                        Node::Element(1, "Fe".to_owned()),
                        Node::Element(1, "Na".to_owned()),
                    ],
                ),
                Node::ForumulaUnit(1, vec![Node::Element(3, "F".to_owned())]),
            ])),
        );

        let res = parse(stream.into_iter().map(|box_tokens| Ok(box_tokens)));

        assert!(res.is_ok());

        assert_eq!(exp, *res.unwrap());
    }
    #[test]
    fn can_parse_formula_unit() {
        let stream = vec![
            Tokens::Number {
                data: 2,
                meta: TokenMetadata::new("2", 0),
            },
            Tokens::Element {
                data: "Fe".to_owned(),
                meta: TokenMetadata::new("Fe", 1),
            },
            Tokens::Element {
                data: "C".to_owned(),
                meta: TokenMetadata::new("C", 3),
            },
            Tokens::Element {
                data: "O".to_owned(),
                meta: TokenMetadata::new("O", 4),
            },
            Tokens::Number {
                data: 3,
                meta: TokenMetadata::new("3", 5),
            },
        ];

        let exp = Node::ForumulaUnit(
            2,
            vec![
                Node::Element(1, "Fe".to_owned()),
                Node::Element(1, "C".to_owned()),
                Node::Element(3, "O".to_owned()),
            ],
        );

        let res = parse(stream.into_iter().map(|box_tokens| Ok(box_tokens)));

        assert!(res.is_ok());

        assert_eq!(exp, *res.unwrap());
    }
}

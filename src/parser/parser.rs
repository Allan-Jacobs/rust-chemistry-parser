use peekmore::PeekMore;

use super::{
    ast_types::Node,
    token_types::Tokens,
};

pub fn parse<'a, T: Iterator<Item = Result<Box<Tokens>, String>>>(stream: T) -> Result<Box<Node>, String> {
    let mut stream = stream.peekmore();
    let mut paren_level = 0;
    let mut current_stack = vec!(Box::new(Node::ForumulaUnit(1, vec!())));
    loop {
        match stream.next() {
            Some(Err(val)) => return Err(val),
            Some(Ok(box Tokens::Number(val))) => {
                let fu = current_stack.pop().unwrap();
                if let box Node::ForumulaUnit(_,vec) = fu {
                    current_stack.push(Box::new(Node::ForumulaUnit(val, vec)));
                } else {
                    return Err("Invalid parent".to_owned());
                }
            },
            Some(Ok(box Tokens::Element(val))) => {
                let mut fu = *current_stack.pop().unwrap();

                if let Node::ForumulaUnit(_,ref mut vec) = fu {
                    if let Some(Ok(box Tokens::Number(count))) = stream.peek() {
                        vec.push(Node::Element(*count, val));
                        stream.next();
                    } else {
                        vec.push(Node::Element(1, val));
                    }
                    current_stack.push(Box::new(fu));
                } else {
                    return Err("Invalid Parent".to_owned());
                }
                
            },
            Some(Ok(box Tokens::Plus)) => {
                let fu = *current_stack.pop().unwrap();
                let mut maybe_reactants = current_stack.pop().map(|val| *val);
                match maybe_reactants {
                    Some(Node::Reactants(ref mut vec)) => {
                        vec.push(fu);
                        current_stack.push(Box::new(maybe_reactants.unwrap()));
                        current_stack.push(Box::new(Node::ForumulaUnit(1, vec!())));
                    },
                    Some(Node::Products(ref mut vec)) => {
                        vec.push(fu);
                        current_stack.push(Box::new(maybe_reactants.unwrap()));
                        current_stack.push(Box::new(Node::ForumulaUnit(1, vec!())));
                    },
                    Some(_) => return Err("Invalid plus".to_owned()),
                    None => {
                        if let Node::ForumulaUnit(_,_) = fu {
                            current_stack.push(Box::new(Node::Reactants(vec!(fu))));
                            current_stack.push(Box::new(Node::ForumulaUnit(1, vec!())));
                        }
                    },
                }
            }
            Some(Ok(box Tokens::Yields)) => {
                let fu = *current_stack.pop().unwrap();
                let mut maybe_reactants = current_stack.pop();

                match maybe_reactants {
                    Some(box Node::Reactants(ref mut vec)) => {
                        vec.push(fu);
                        current_stack.push(maybe_reactants.unwrap());
                        current_stack.push(Box::new(Node::Products(vec!())));
                        current_stack.push(Box::new(Node::ForumulaUnit(1, vec!())));
                    },
                    Some(_) => return Err("Invalid yields location".to_owned()),
                    None => {
                        current_stack.push(Box::new(Node::Reactants(vec!(fu))));
                        current_stack.push(Box::new(Node::ForumulaUnit(1, vec!())));
                    },
                }
            }
            Some(Ok(box Tokens::Paren(super::token_types::ParenType::OPEN))) => {
                paren_level += 1;
                current_stack.push(Box::new(Node::Group(1, vec!())));
            },
            Some(Ok(box Tokens::Paren(super::token_types::ParenType::CLOSE))) => {
                if paren_level == 0 {
                    return Err("Invalid closing paren".to_owned());
                };
                paren_level -= 1;
                let group = *current_stack.pop().unwrap();
                let mut maybe_fu_or_group = current_stack.pop();

                match maybe_fu_or_group {
                    Some(box Node::Group(_,ref mut vec)) => {
                        if let Some(Ok(box Tokens::Number(val))) = stream.peek() {
                            if let Node::Group(_,inner_vec) = group {
                                vec.push(Node::Group(*val, inner_vec));
                            }
                        }
                        current_stack.push(maybe_fu_or_group.unwrap());
                    },
                    Some(box Node::ForumulaUnit(_, ref mut vec)) => {
                        vec.push(group);
                        current_stack.push(maybe_fu_or_group.unwrap());
                    },

                    Some(_) => return Err("Invalid Parent".to_owned()),
                    None => return Err("Stack Underflow in group".to_owned()),
                }
            },
            None => {
                let fu = *current_stack.pop().unwrap();
                let mut products_or_none = current_stack.pop();
                match products_or_none {
                    Some(box Node::Products(ref mut vec)) => {
                        let reactants = current_stack.pop().unwrap();
                        vec.push(fu);
                        current_stack.push(Box::new(Node::Equation(reactants, products_or_none.unwrap())));
                    },
                    Some(_) => return Err("Stack had unexpected value".to_owned()),
                    None => {
                        current_stack.push(Box::new(fu));
                    },
                }
                break
            },
        }
    }
    Ok(current_stack.pop().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_parse_equation() {
        let stream = vec!(
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

        let exp = Node::Equation(
            Box::new(Node::Reactants(vec!(
                Node::ForumulaUnit(2, vec!(Node::Element(1, "Fe".to_owned()))),
                Node::ForumulaUnit(1, vec!(Node::Element(2, "Na".to_owned()), Node::Element(3, "F".to_owned()))),
            ))),
            Box::new(Node::Products(vec!(
                Node::ForumulaUnit(2, vec!(
                    Node::Element(1, "Fe".to_owned()),
                    Node::Element(1, "Na".to_owned()),
                )),
                Node::ForumulaUnit(1, vec!(Node::Element(3, "F".to_owned())))
            )))
        );

        let res = parse(
            stream.into_iter()
            .map(|box_tokens| Ok(box_tokens))
        );

        assert!(res.is_ok());

        assert_eq!(exp, *res.unwrap());
    }
    #[test]
    fn can_parse_formula_unit() {
        let stream = vec!(
            Box::new(Tokens::Number(2)),
            Box::new(Tokens::Element("Fe".to_owned())),
            Box::new(Tokens::Element("C".to_owned())),
            Box::new(Tokens::Element("O".to_owned())),
            Box::new(Tokens::Number(3)),
        );

        let exp = Node::ForumulaUnit(2, vec!(
            Node::Element(1, "Fe".to_owned()),
            Node::Element(1, "C".to_owned()),
            Node::Element(3, "O".to_owned())
        ));

        let res = parse(
            stream.into_iter()
            .map(|box_tokens| Ok(box_tokens))
        );

        assert!(res.is_ok());

        assert_eq!(exp, *res.unwrap());
    }
}
//! CSS selector parser

use super::types::{
    AttributeOp, AttributeSelector, Combinator, PseudoClass, Selector, SelectorParseError,
    SelectorPart,
};

/// Parse a single selector
pub fn parse_selector(input: &str) -> Result<Selector, SelectorParseError> {
    let mut parser = SelectorParser::new(input);
    parser.parse_selector()
}

/// Parse comma-separated selectors
pub fn parse_selectors(input: &str) -> Result<Vec<Selector>, SelectorParseError> {
    let mut selectors = Vec::new();

    for part in input.split(',') {
        let trimmed = part.trim();
        if !trimmed.is_empty() {
            selectors.push(parse_selector(trimmed)?);
        }
    }

    Ok(selectors)
}

struct SelectorParser<'a> {
    _input: &'a str,
    chars: Vec<char>,
    pos: usize,
}

impl<'a> SelectorParser<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            _input: input,
            chars: input.chars().collect(),
            pos: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.peek();
        if ch.is_some() {
            self.pos += 1;
        }
        ch
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn parse_identifier(&mut self) -> String {
        let mut ident = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() || ch == '-' || ch == '_' {
                ident.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        ident
    }

    fn parse_selector(&mut self) -> Result<Selector, SelectorParseError> {
        let mut parts = Vec::new();

        loop {
            self.skip_whitespace();

            if self.peek().is_none() {
                break;
            }

            // Parse combinator if not first part
            let combinator = if !parts.is_empty() {
                self.parse_combinator()
            } else {
                None
            };

            self.skip_whitespace();

            // Parse selector part
            let part = self.parse_selector_part()?;

            if part.is_empty() && combinator.is_some() {
                return Err(SelectorParseError {
                    message: "Expected selector after combinator".to_string(),
                    position: self.pos,
                });
            }

            if part.is_empty() {
                break;
            }

            // Add combinator to previous part
            if let Some(comb) = combinator {
                if let Some(last) = parts.last_mut() {
                    let (_, ref mut prev_comb) = last;
                    *prev_comb = Some(comb);
                }
            } else if !parts.is_empty() {
                // Default to descendant combinator
                if let Some(last) = parts.last_mut() {
                    let (_, ref mut prev_comb) = last;
                    if prev_comb.is_none() {
                        *prev_comb = Some(Combinator::Descendant);
                    }
                }
            }

            parts.push((part, None));
        }

        Ok(Selector { parts })
    }

    fn parse_combinator(&mut self) -> Option<Combinator> {
        self.skip_whitespace();
        match self.peek() {
            Some('>') => {
                self.advance();
                Some(Combinator::Child)
            }
            Some('+') => {
                self.advance();
                Some(Combinator::AdjacentSibling)
            }
            Some('~') => {
                self.advance();
                Some(Combinator::GeneralSibling)
            }
            _ => None,
        }
    }

    fn parse_selector_part(&mut self) -> Result<SelectorPart, SelectorParseError> {
        let mut part = SelectorPart::new();

        loop {
            match self.peek() {
                Some('*') => {
                    self.advance();
                    part.universal = true;
                }
                Some('#') => {
                    self.advance();
                    let id = self.parse_identifier();
                    if id.is_empty() {
                        return Err(SelectorParseError {
                            message: "Expected ID after #".to_string(),
                            position: self.pos,
                        });
                    }
                    part.id = Some(id);
                }
                Some('.') => {
                    self.advance();
                    let class = self.parse_identifier();
                    if class.is_empty() {
                        return Err(SelectorParseError {
                            message: "Expected class name after .".to_string(),
                            position: self.pos,
                        });
                    }
                    part.classes.push(class);
                }
                Some(':') => {
                    self.advance();
                    let pseudo = self.parse_pseudo_class()?;
                    part.pseudo_classes.push(pseudo);
                }
                Some('[') => {
                    self.advance();
                    let attr = self.parse_attribute_selector()?;
                    part.attributes.push(attr);
                }
                Some(ch) if ch.is_alphabetic() || ch == '_' => {
                    if part.element.is_none() && !part.universal {
                        let elem = self.parse_identifier();
                        part.element = Some(elem);
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }

        Ok(part)
    }

    fn parse_pseudo_class(&mut self) -> Result<PseudoClass, SelectorParseError> {
        let name = self.parse_identifier();

        let pseudo = match name.to_lowercase().as_str() {
            "focus" => PseudoClass::Focus,
            "hover" => PseudoClass::Hover,
            "active" => PseudoClass::Active,
            "disabled" => PseudoClass::Disabled,
            "enabled" => PseudoClass::Enabled,
            "checked" => PseudoClass::Checked,
            "selected" => PseudoClass::Selected,
            "empty" => PseudoClass::Empty,
            "first-child" => PseudoClass::FirstChild,
            "last-child" => PseudoClass::LastChild,
            "only-child" => PseudoClass::OnlyChild,
            "nth-child" => {
                let n = self.parse_nth_argument()?;
                PseudoClass::NthChild(n)
            }
            "nth-last-child" => {
                let n = self.parse_nth_argument()?;
                PseudoClass::NthLastChild(n)
            }
            "not" => {
                // Simple :not() - only supports single pseudo-class for now
                if self.peek() == Some('(') {
                    self.advance();
                    self.skip_whitespace();
                    if self.peek() == Some(':') {
                        self.advance();
                        let inner = self.parse_pseudo_class()?;
                        self.skip_whitespace();
                        if self.peek() == Some(')') {
                            self.advance();
                        }
                        PseudoClass::Not(Box::new(inner))
                    } else {
                        return Err(SelectorParseError {
                            message: "Expected pseudo-class in :not()".to_string(),
                            position: self.pos,
                        });
                    }
                } else {
                    return Err(SelectorParseError {
                        message: "Expected ( after :not".to_string(),
                        position: self.pos,
                    });
                }
            }
            _ => {
                return Err(SelectorParseError {
                    message: format!("Unknown pseudo-class: {}", name),
                    position: self.pos,
                });
            }
        };

        Ok(pseudo)
    }

    fn parse_nth_argument(&mut self) -> Result<usize, SelectorParseError> {
        if self.peek() != Some('(') {
            return Err(SelectorParseError {
                message: "Expected ( for nth argument".to_string(),
                position: self.pos,
            });
        }
        self.advance();
        self.skip_whitespace();

        let mut num = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_numeric() {
                num.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        self.skip_whitespace();
        if self.peek() == Some(')') {
            self.advance();
        }

        num.parse().map_err(|_| SelectorParseError {
            message: "Invalid nth argument".to_string(),
            position: self.pos,
        })
    }

    fn parse_attribute_selector(&mut self) -> Result<AttributeSelector, SelectorParseError> {
        self.skip_whitespace();
        let name = self.parse_identifier();

        if name.is_empty() {
            return Err(SelectorParseError {
                message: "Expected attribute name".to_string(),
                position: self.pos,
            });
        }

        self.skip_whitespace();

        let (op, value) = if self.peek() == Some(']') {
            (AttributeOp::Exists, None)
        } else {
            let op = self.parse_attribute_op()?;
            self.skip_whitespace();
            let value = self.parse_attribute_value()?;
            (op, Some(value))
        };

        self.skip_whitespace();

        // Check for case insensitive flag
        let case_insensitive = if self.peek() == Some('i') || self.peek() == Some('I') {
            self.advance();
            self.skip_whitespace();
            true
        } else {
            false
        };

        if self.peek() == Some(']') {
            self.advance();
        }

        Ok(AttributeSelector {
            name,
            op,
            value,
            case_insensitive,
        })
    }

    fn parse_attribute_op(&mut self) -> Result<AttributeOp, SelectorParseError> {
        match self.peek() {
            Some('=') => {
                self.advance();
                Ok(AttributeOp::Equals)
            }
            Some('~') => {
                self.advance();
                if self.peek() == Some('=') {
                    self.advance();
                    Ok(AttributeOp::ContainsWord)
                } else {
                    Err(SelectorParseError {
                        message: "Expected = after ~".to_string(),
                        position: self.pos,
                    })
                }
            }
            Some('|') => {
                self.advance();
                if self.peek() == Some('=') {
                    self.advance();
                    Ok(AttributeOp::StartsWithWord)
                } else {
                    Err(SelectorParseError {
                        message: "Expected = after |".to_string(),
                        position: self.pos,
                    })
                }
            }
            Some('^') => {
                self.advance();
                if self.peek() == Some('=') {
                    self.advance();
                    Ok(AttributeOp::StartsWith)
                } else {
                    Err(SelectorParseError {
                        message: "Expected = after ^".to_string(),
                        position: self.pos,
                    })
                }
            }
            Some('$') => {
                self.advance();
                if self.peek() == Some('=') {
                    self.advance();
                    Ok(AttributeOp::EndsWith)
                } else {
                    Err(SelectorParseError {
                        message: "Expected = after $".to_string(),
                        position: self.pos,
                    })
                }
            }
            Some('*') => {
                self.advance();
                if self.peek() == Some('=') {
                    self.advance();
                    Ok(AttributeOp::Contains)
                } else {
                    Err(SelectorParseError {
                        message: "Expected = after *".to_string(),
                        position: self.pos,
                    })
                }
            }
            _ => Err(SelectorParseError {
                message: "Expected attribute operator".to_string(),
                position: self.pos,
            }),
        }
    }

    fn parse_attribute_value(&mut self) -> Result<String, SelectorParseError> {
        let quote = match self.peek() {
            Some('"') | Some('\'') => self.advance(), // Safe: peek confirmed char exists
            _ => None,
        };

        let mut value = String::new();
        while let Some(ch) = self.peek() {
            if let Some(q) = quote {
                if ch == q {
                    self.advance();
                    break;
                }
            } else if ch == ']' || ch.is_whitespace() {
                break;
            }
            value.push(ch);
            self.advance();
        }

        Ok(value)
    }
}

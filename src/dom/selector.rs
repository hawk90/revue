//! CSS selector parsing and representation
//!
//! Supports full CSS selector syntax:
//! - Type: `Button`, `Input`
//! - ID: `#submit`, `#main-content`
//! - Class: `.primary`, `.btn-large`
//! - Universal: `*`
//! - Attribute: `[disabled]`, `[type="text"]`
//! - Pseudo-class: `:focus`, `:hover`, `:nth-child(2)`
//! - Combinators: ` ` (descendant), `>` (child), `+` (adjacent), `~` (sibling)
//! - Grouping: `Button, Input` (comma-separated)

use std::fmt;

/// Pseudo-class types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PseudoClass {
    /// :focus - has keyboard focus
    Focus,
    /// :hover - mouse hovering
    Hover,
    /// :active - being activated/pressed
    Active,
    /// :disabled - disabled state
    Disabled,
    /// :enabled - not disabled
    Enabled,
    /// :checked - checkbox/radio checked
    Checked,
    /// :selected - selected item
    Selected,
    /// :empty - no children
    Empty,
    /// :first-child
    FirstChild,
    /// :last-child
    LastChild,
    /// :only-child
    OnlyChild,
    /// :nth-child(n)
    NthChild(usize),
    /// :nth-last-child(n)
    NthLastChild(usize),
    /// :not(selector)
    Not(Box<PseudoClass>),
}

impl fmt::Display for PseudoClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PseudoClass::Focus => write!(f, ":focus"),
            PseudoClass::Hover => write!(f, ":hover"),
            PseudoClass::Active => write!(f, ":active"),
            PseudoClass::Disabled => write!(f, ":disabled"),
            PseudoClass::Enabled => write!(f, ":enabled"),
            PseudoClass::Checked => write!(f, ":checked"),
            PseudoClass::Selected => write!(f, ":selected"),
            PseudoClass::Empty => write!(f, ":empty"),
            PseudoClass::FirstChild => write!(f, ":first-child"),
            PseudoClass::LastChild => write!(f, ":last-child"),
            PseudoClass::OnlyChild => write!(f, ":only-child"),
            PseudoClass::NthChild(n) => write!(f, ":nth-child({})", n),
            PseudoClass::NthLastChild(n) => write!(f, ":nth-last-child({})", n),
            PseudoClass::Not(inner) => write!(f, ":not({})", inner),
        }
    }
}

/// Attribute selector operator
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttributeOp {
    /// [attr] - has attribute
    Exists,
    /// [attr=value] - exact match
    Equals,
    /// [attr~=value] - contains word
    ContainsWord,
    /// [attr|=value] - starts with word
    StartsWithWord,
    /// [attr^=value] - starts with
    StartsWith,
    /// [attr$=value] - ends with
    EndsWith,
    /// [attr*=value] - contains
    Contains,
}

/// Attribute selector
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttributeSelector {
    /// Attribute name
    pub name: String,
    /// Operator (None for existence check)
    pub op: AttributeOp,
    /// Value to match (None for existence check)
    pub value: Option<String>,
    /// Case insensitive flag [attr=value i]
    pub case_insensitive: bool,
}

/// A single selector part (between combinators)
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SelectorPart {
    /// Type/element name (None = universal)
    pub element: Option<String>,
    /// ID selector
    pub id: Option<String>,
    /// Class selectors
    pub classes: Vec<String>,
    /// Attribute selectors
    pub attributes: Vec<AttributeSelector>,
    /// Pseudo-classes
    pub pseudo_classes: Vec<PseudoClass>,
    /// Universal selector (*)
    pub universal: bool,
}

impl SelectorPart {
    /// Create a new empty selector part
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a type selector
    pub fn element(name: impl Into<String>) -> Self {
        Self {
            element: Some(name.into()),
            ..Default::default()
        }
    }

    /// Create a universal selector
    pub fn universal() -> Self {
        Self {
            universal: true,
            ..Default::default()
        }
    }

    /// Create an ID selector
    pub fn id(id: impl Into<String>) -> Self {
        Self {
            id: Some(id.into()),
            ..Default::default()
        }
    }

    /// Create a class selector
    pub fn class(class: impl Into<String>) -> Self {
        Self {
            classes: vec![class.into()],
            ..Default::default()
        }
    }

    /// Add an ID
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Add a class
    pub fn with_class(mut self, class: impl Into<String>) -> Self {
        self.classes.push(class.into());
        self
    }

    /// Add a pseudo-class
    pub fn with_pseudo(mut self, pseudo: PseudoClass) -> Self {
        self.pseudo_classes.push(pseudo);
        self
    }

    /// Check if this part is empty (no matchers)
    pub fn is_empty(&self) -> bool {
        self.element.is_none()
            && self.id.is_none()
            && self.classes.is_empty()
            && self.attributes.is_empty()
            && self.pseudo_classes.is_empty()
            && !self.universal
    }

    /// Calculate specificity (a, b, c)
    /// - a: ID selectors
    /// - b: class selectors, attribute selectors, pseudo-classes
    /// - c: type selectors, pseudo-elements
    pub fn specificity(&self) -> (usize, usize, usize) {
        let a = if self.id.is_some() { 1 } else { 0 };
        let b = self.classes.len() + self.attributes.len() + self.pseudo_classes.len();
        let c = if self.element.is_some() { 1 } else { 0 };
        (a, b, c)
    }
}

impl fmt::Display for SelectorPart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.universal {
            write!(f, "*")?;
        }
        if let Some(ref elem) = self.element {
            write!(f, "{}", elem)?;
        }
        if let Some(ref id) = self.id {
            write!(f, "#{}", id)?;
        }
        for class in &self.classes {
            write!(f, ".{}", class)?;
        }
        for pseudo in &self.pseudo_classes {
            write!(f, "{}", pseudo)?;
        }
        Ok(())
    }
}

/// Combinator between selector parts
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Combinator {
    /// ` ` - descendant (any level)
    Descendant,
    /// `>` - direct child
    Child,
    /// `+` - adjacent sibling (immediately after)
    AdjacentSibling,
    /// `~` - general sibling (any sibling after)
    GeneralSibling,
}

impl fmt::Display for Combinator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Combinator::Descendant => write!(f, " "),
            Combinator::Child => write!(f, " > "),
            Combinator::AdjacentSibling => write!(f, " + "),
            Combinator::GeneralSibling => write!(f, " ~ "),
        }
    }
}

/// A complete selector (chain of parts with combinators)
#[derive(Debug, Clone)]
pub struct Selector {
    /// Parts and combinators: [(part, combinator_to_next), ...]
    /// Last part has no combinator (None)
    pub parts: Vec<(SelectorPart, Option<Combinator>)>,
}

impl Selector {
    /// Create a new selector with a single part
    pub fn new(part: SelectorPart) -> Self {
        Self {
            parts: vec![(part, None)],
        }
    }

    /// Create an empty selector
    pub fn empty() -> Self {
        Self { parts: Vec::new() }
    }

    /// Add a part with a combinator
    pub fn then(mut self, combinator: Combinator, part: SelectorPart) -> Self {
        if let Some(last) = self.parts.last_mut() {
            last.1 = Some(combinator);
        }
        self.parts.push((part, None));
        self
    }

    /// Add a descendant part
    pub fn descendant(self, part: SelectorPart) -> Self {
        self.then(Combinator::Descendant, part)
    }

    /// Add a child part
    pub fn child(self, part: SelectorPart) -> Self {
        self.then(Combinator::Child, part)
    }

    /// Get the last (most specific) part
    pub fn target(&self) -> Option<&SelectorPart> {
        self.parts.last().map(|(part, _)| part)
    }

    /// Calculate total specificity
    pub fn specificity(&self) -> (usize, usize, usize) {
        self.parts.iter().fold((0, 0, 0), |acc, (part, _)| {
            let (a, b, c) = part.specificity();
            (acc.0 + a, acc.1 + b, acc.2 + c)
        })
    }

    /// Check if selector is empty
    pub fn is_empty(&self) -> bool {
        self.parts.is_empty()
    }
}

impl fmt::Display for Selector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, (part, _combinator)) in self.parts.iter().enumerate() {
            if i > 0 {
                if let Some(prev_comb) = self.parts.get(i - 1).and_then(|(_, c)| c.as_ref()) {
                    write!(f, "{}", prev_comb)?;
                }
            }
            write!(f, "{}", part)?;
        }
        Ok(())
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Selector Parser
// ─────────────────────────────────────────────────────────────────────────────

/// Parse error
#[derive(Debug, Clone)]
pub struct SelectorParseError {
    pub message: String,
    pub position: usize,
}

impl fmt::Display for SelectorParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Selector parse error at {}: {}", self.position, self.message)
    }
}

impl std::error::Error for SelectorParseError {}

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
    input: &'a str,
    chars: Vec<char>,
    pos: usize,
}

impl<'a> SelectorParser<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            input,
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
            Some('"') | Some('\'') => {
                let q = self.advance().unwrap();
                Some(q)
            }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_type_selector() {
        let sel = parse_selector("Button").unwrap();
        assert_eq!(sel.parts.len(), 1);
        assert_eq!(sel.parts[0].0.element, Some("Button".to_string()));
    }

    #[test]
    fn test_parse_id_selector() {
        let sel = parse_selector("#submit").unwrap();
        assert_eq!(sel.parts[0].0.id, Some("submit".to_string()));
    }

    #[test]
    fn test_parse_class_selector() {
        let sel = parse_selector(".primary").unwrap();
        assert_eq!(sel.parts[0].0.classes, vec!["primary".to_string()]);
    }

    #[test]
    fn test_parse_combined_selector() {
        let sel = parse_selector("Button#submit.primary.large").unwrap();
        let part = &sel.parts[0].0;
        assert_eq!(part.element, Some("Button".to_string()));
        assert_eq!(part.id, Some("submit".to_string()));
        assert!(part.classes.contains(&"primary".to_string()));
        assert!(part.classes.contains(&"large".to_string()));
    }

    #[test]
    fn test_parse_pseudo_class() {
        let sel = parse_selector("Button:focus").unwrap();
        assert_eq!(sel.parts[0].0.pseudo_classes, vec![PseudoClass::Focus]);
    }

    #[test]
    fn test_parse_descendant() {
        let sel = parse_selector(".sidebar Button").unwrap();
        assert_eq!(sel.parts.len(), 2);
        assert_eq!(sel.parts[0].1, Some(Combinator::Descendant));
    }

    #[test]
    fn test_parse_child() {
        let sel = parse_selector(".sidebar > Button").unwrap();
        assert_eq!(sel.parts.len(), 2);
        assert_eq!(sel.parts[0].1, Some(Combinator::Child));
    }

    #[test]
    fn test_parse_adjacent_sibling() {
        let sel = parse_selector("Label + Input").unwrap();
        assert_eq!(sel.parts.len(), 2);
        assert_eq!(sel.parts[0].1, Some(Combinator::AdjacentSibling));
    }

    #[test]
    fn test_specificity() {
        // Type only
        let sel = parse_selector("Button").unwrap();
        assert_eq!(sel.specificity(), (0, 0, 1));

        // Class only
        let sel = parse_selector(".primary").unwrap();
        assert_eq!(sel.specificity(), (0, 1, 0));

        // ID only
        let sel = parse_selector("#submit").unwrap();
        assert_eq!(sel.specificity(), (1, 0, 0));

        // Combined
        let sel = parse_selector("Button#submit.primary").unwrap();
        assert_eq!(sel.specificity(), (1, 1, 1));

        // Multiple classes
        let sel = parse_selector(".btn.primary.large").unwrap();
        assert_eq!(sel.specificity(), (0, 3, 0));
    }

    #[test]
    fn test_parse_nth_child() {
        let sel = parse_selector(":nth-child(3)").unwrap();
        assert_eq!(sel.parts[0].0.pseudo_classes, vec![PseudoClass::NthChild(3)]);
    }

    #[test]
    fn test_parse_multiple_selectors() {
        let sels = parse_selectors("Button, Input, .primary").unwrap();
        assert_eq!(sels.len(), 3);
    }
}

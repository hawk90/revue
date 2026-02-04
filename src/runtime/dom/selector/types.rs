//! CSS selector types

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
    /// `[attr]` - has attribute
    Exists,
    /// `[attr=value]` - exact match
    Equals,
    /// `[attr~=value]` - contains word
    ContainsWord,
    /// `[attr|=value]` - starts with word
    StartsWithWord,
    /// `[attr^=value]` - starts with
    StartsWith,
    /// `[attr$=value]` - ends with
    EndsWith,
    /// `[attr*=value]` - contains
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

/// Parse error
#[derive(Debug, Clone)]
pub struct SelectorParseError {
    pub message: String,
    pub position: usize,
}

impl fmt::Display for SelectorParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Selector parse error at {}: {}",
            self.position, self.message
        )
    }
}

impl std::error::Error for SelectorParseError {}

//! Query DSL for filtering and searching items
//!
//! Provides a powerful search query language inspired by eilmeldung's
//! article filtering with enhanced capabilities.
//!
//! # Query Syntax
//!
//! - **Free text**: `hello world` - matches items containing these words
//! - **Field match**: `author:john` - exact field match
//! - **Contains**: `title~rust` - field contains value
//! - **Not equal**: `status:!draft` - field not equal to value
//! - **Comparison**: `age:>18`, `price:<100` - numeric comparisons
//! - **Boolean**: `active:true`, `published:false`
//! - **Date**: `after:2024-01-01`, `before:2024-12-31`
//!
//! # Case
//!
//! Strings are compared without case by default: `author:john`, `title~rust`,
//! free text, `name:>=bob` and sorting all ignore it, and agree with each
//! other. [`Case`] picks another rule for the whole query - pass it to
//! [`Query::filter_items_with`] or [`Query::matches_with`]:
//! [`Case::Sensitive`], or [`Case::Smart`] (fzf's and vim's `smartcase`: a
//! searched value with an upper-case letter is matched exactly).
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::query::{Query, Queryable};
//!
//! struct Article {
//!     title: String,
//!     author: String,
//!     published: bool,
//! }
//!
//! impl Queryable for Article {
//!     fn field_value(&self, field: &str) -> Option<QueryValue> {
//!         match field {
//!             "title" => Some(QueryValue::String(self.title.clone())),
//!             "author" => Some(QueryValue::String(self.author.clone())),
//!             "published" => Some(QueryValue::Bool(self.published)),
//!             _ => None,
//!         }
//!     }
//!
//!     fn full_text(&self) -> String {
//!         format!("{} {}", self.title, self.author)
//!     }
//! }
//!
//! let query = Query::parse("author:john published:true").unwrap();
//! let articles = vec![/* ... */];
//! let filtered: Vec<_> = articles.iter().filter(|a| query.matches(a)).collect();
//! ```

mod parser;

pub use parser::ParseError;

/// Value types for query matching
#[derive(Debug, Clone, PartialEq)]
pub enum QueryValue {
    /// String value
    String(String),
    /// Integer value
    Int(i64),
    /// Float value
    Float(f64),
    /// Boolean value
    Bool(bool),
    /// Date value (ISO 8601 string)
    Date(String),
    /// Null/None value
    Null,
}

impl QueryValue {
    /// Create a string value
    pub fn string(s: impl Into<String>) -> Self {
        Self::String(s.into())
    }

    /// Create an integer value
    pub fn int(n: i64) -> Self {
        Self::Int(n)
    }

    /// Create a float value
    pub fn float(n: f64) -> Self {
        Self::Float(n)
    }

    /// Create a boolean value
    pub fn bool(b: bool) -> Self {
        Self::Bool(b)
    }

    /// Check if value contains substring (case-insensitive)
    pub fn contains(&self, needle: &str) -> bool {
        self.contains_with(needle, Case::Insensitive)
    }

    /// [`contains`](Self::contains), comparing strings by `case`
    pub fn contains_with(&self, needle: &str, case: Case) -> bool {
        match self {
            Self::String(s) if case.sensitive_for(needle) => s.contains(needle),
            Self::String(s) => s.to_lowercase().contains(&needle.to_lowercase()),
            Self::Int(n) => n.to_string().contains(needle),
            Self::Float(n) => n.to_string().contains(needle),
            Self::Bool(b) => b.to_string() == needle.to_lowercase(),
            Self::Date(d) => d.contains(needle),
            Self::Null => false,
        }
    }

    /// Check equality with a string (case-insensitive)
    pub fn equals_str(&self, other: &str) -> bool {
        self.equals_str_with(other, Case::Insensitive)
    }

    /// [`equals_str`](Self::equals_str), comparing strings by `case`
    pub fn equals_str_with(&self, other: &str, case: Case) -> bool {
        match self {
            Self::String(s) => cmp_str(s, other, case.sensitive_for(other)).is_eq(),
            Self::Int(n) => other.parse::<i64>().map(|o| *n == o).unwrap_or(false),
            Self::Float(n) => other
                .parse::<f64>()
                .map(|o| (*n - o).abs() < f64::EPSILON)
                .unwrap_or(false),
            Self::Bool(b) => {
                let other_lower = other.to_lowercase();
                (*b && (other_lower == "true" || other_lower == "yes" || other_lower == "1"))
                    || (!*b
                        && (other_lower == "false" || other_lower == "no" || other_lower == "0"))
            }
            Self::Date(d) => d == other,
            Self::Null => other.to_lowercase() == "null" || other.is_empty(),
        }
    }

    /// Compare values (for >, <, >=, <=)
    ///
    /// Strings compare without case, like [`equals_str`](Self::equals_str):
    /// a value equal to `other` compares `Equal`.
    pub fn compare(&self, other: &str) -> Option<std::cmp::Ordering> {
        self.compare_with(other, Case::Insensitive)
    }

    /// [`compare`](Self::compare), comparing strings by `case`
    pub fn compare_with(&self, other: &str, case: Case) -> Option<std::cmp::Ordering> {
        match self {
            Self::Int(n) => other.parse::<i64>().ok().map(|o| n.cmp(&o)),
            Self::Float(n) => other.parse::<f64>().ok().and_then(|o| n.partial_cmp(&o)),
            Self::String(s) => Some(cmp_str(s, other, case.sensitive_for(other))),
            Self::Date(d) => Some(d.cmp(&other.to_string())),
            _ => None,
        }
    }
}

/// How string values are compared: in equality (`field:value`), `~`
/// contains, free text, range comparisons (`>`, `<`, `>=`, `<=`) and sorting.
///
/// Every string comparison of a query follows the same rule, so `name:bob`,
/// `name:>=bob` and a sort by `name` always agree. The default is
/// [`Insensitive`](Self::Insensitive), as in most search boxes;
/// [`Smart`](Self::Smart) is fzf's and vim's `smartcase`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Case {
    /// `rust` matches `Rust` and `RUST`
    #[default]
    Insensitive,
    /// `rust` matches only `rust`
    Sensitive,
    /// Insensitive, unless the searched value has an upper-case letter:
    /// `rust` matches `Rust`, but `Rust` matches only `Rust`. Sorting, which
    /// has no searched value, ignores case.
    Smart,
}

impl Case {
    /// Whether `needle`, searched for under this rule, is compared with case.
    fn sensitive_for(self, needle: &str) -> bool {
        match self {
            Case::Insensitive => false,
            Case::Sensitive => true,
            Case::Smart => needle.chars().any(char::is_uppercase),
        }
    }

    /// Compare two strings under this rule. Ignoring case, strings that differ
    /// only in case fall back to a case-sensitive order, so a sort is
    /// deterministic; they still count as equal for filters.
    fn cmp_for_sort(self, a: &str, b: &str) -> std::cmp::Ordering {
        match self {
            Case::Sensitive => a.cmp(b),
            Case::Insensitive | Case::Smart => a
                .to_lowercase()
                .cmp(&b.to_lowercase())
                .then_with(|| a.cmp(b)),
        }
    }
}

/// `a` and `b` compared with or without case.
fn cmp_str(a: &str, b: &str, sensitive: bool) -> std::cmp::Ordering {
    if sensitive {
        a.cmp(b)
    } else {
        a.to_lowercase().cmp(&b.to_lowercase())
    }
}

/// Comparison operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    /// Equals (field:value)
    Eq,
    /// Not equals (field:!value)
    Ne,
    /// Greater than (field:>value)
    Gt,
    /// Less than (field:<value)
    Lt,
    /// Greater or equal (field:>=value)
    Ge,
    /// Less or equal (field:<=value)
    Le,
    /// Contains (field~value)
    Contains,
}

/// A single filter condition
#[derive(Debug, Clone)]
pub enum Filter {
    /// Free text search
    Text(String),
    /// Field comparison
    Field {
        /// Field name
        name: String,
        /// Comparison operator
        op: Operator,
        /// Value to compare against
        value: String,
    },
    /// AND combination
    And(Box<Filter>, Box<Filter>),
    /// OR combination
    Or(Box<Filter>, Box<Filter>),
    /// NOT negation
    Not(Box<Filter>),
}

impl Filter {
    /// Create a text filter
    pub fn text(s: impl Into<String>) -> Self {
        Self::Text(s.into())
    }

    /// Create an equality filter
    pub fn eq(field: impl Into<String>, value: impl Into<String>) -> Self {
        Self::Field {
            name: field.into(),
            op: Operator::Eq,
            value: value.into(),
        }
    }

    /// Create a not-equals filter
    pub fn ne(field: impl Into<String>, value: impl Into<String>) -> Self {
        Self::Field {
            name: field.into(),
            op: Operator::Ne,
            value: value.into(),
        }
    }

    /// Create a contains filter
    pub fn contains(field: impl Into<String>, value: impl Into<String>) -> Self {
        Self::Field {
            name: field.into(),
            op: Operator::Contains,
            value: value.into(),
        }
    }

    /// Create a greater-than filter
    pub fn gt(field: impl Into<String>, value: impl Into<String>) -> Self {
        Self::Field {
            name: field.into(),
            op: Operator::Gt,
            value: value.into(),
        }
    }

    /// Create a less-than filter
    pub fn lt(field: impl Into<String>, value: impl Into<String>) -> Self {
        Self::Field {
            name: field.into(),
            op: Operator::Lt,
            value: value.into(),
        }
    }

    /// Combine with AND
    pub fn and(self, other: Filter) -> Self {
        Self::And(Box::new(self), Box::new(other))
    }

    /// Combine with OR
    pub fn or(self, other: Filter) -> Self {
        Self::Or(Box::new(self), Box::new(other))
    }

    /// Negate
    pub fn negate(self) -> Self {
        Self::Not(Box::new(self))
    }

    /// Check if item matches this filter, ignoring case
    pub fn matches<T: Queryable>(&self, item: &T) -> bool {
        self.matches_with(item, Case::Insensitive)
    }

    /// Check if item matches this filter, comparing strings by `case`
    pub fn matches_with<T: Queryable>(&self, item: &T, case: Case) -> bool {
        match self {
            Self::Text(text) => {
                let full_text = item.full_text();
                // Convert full_text once instead of per-word
                let full_text_lower = full_text.to_lowercase();
                text.split_whitespace().all(|word| {
                    if case.sensitive_for(word) {
                        full_text.contains(word)
                    } else {
                        full_text_lower.contains(&word.to_lowercase())
                    }
                })
            }
            Self::Field { name, op, value } => {
                if let Some(field_value) = item.field_value(name) {
                    let compare = || field_value.compare_with(value, case);
                    match op {
                        Operator::Eq => field_value.equals_str_with(value, case),
                        Operator::Ne => !field_value.equals_str_with(value, case),
                        Operator::Contains => field_value.contains_with(value, case),
                        Operator::Gt => compare() == Some(std::cmp::Ordering::Greater),
                        Operator::Lt => compare() == Some(std::cmp::Ordering::Less),
                        Operator::Ge => matches!(
                            compare(),
                            Some(std::cmp::Ordering::Greater | std::cmp::Ordering::Equal)
                        ),
                        Operator::Le => matches!(
                            compare(),
                            Some(std::cmp::Ordering::Less | std::cmp::Ordering::Equal)
                        ),
                    }
                } else {
                    false
                }
            }
            Self::And(a, b) => a.matches_with(item, case) && b.matches_with(item, case),
            Self::Or(a, b) => a.matches_with(item, case) || b.matches_with(item, case),
            Self::Not(f) => !f.matches_with(item, case),
        }
    }
}

/// Sort direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SortDirection {
    /// Ascending order (A-Z, 0-9)
    #[default]
    Ascending,
    /// Descending order (Z-A, 9-0)
    Descending,
}

/// Sort specification
#[derive(Debug, Clone)]
pub struct SortBy {
    /// Field to sort by
    pub field: String,
    /// Sort direction
    pub direction: SortDirection,
}

impl SortBy {
    /// Create ascending sort
    pub fn asc(field: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            direction: SortDirection::Ascending,
        }
    }

    /// Create descending sort
    pub fn desc(field: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            direction: SortDirection::Descending,
        }
    }
}

/// A complete query with filters, sorting, and pagination
#[derive(Debug, Clone, Default)]
pub struct Query {
    /// Filter conditions (ANDed together)
    pub filters: Vec<Filter>,
    /// Sort specification
    pub sort: Option<SortBy>,
    /// Maximum results
    pub limit: Option<usize>,
    /// Skip first N results
    pub offset: Option<usize>,
}

impl Query {
    /// Create an empty query (matches everything)
    pub fn new() -> Self {
        Self::default()
    }

    /// Parse a query string
    ///
    /// # Errors
    ///
    /// Returns `Err(ParseError)` if:
    /// - The query syntax is invalid
    /// - A field operator is not recognized
    /// - A value cannot be parsed for its expected type
    pub fn parse(input: &str) -> Result<Self, ParseError> {
        parser::parse(input)
    }

    /// Add a filter
    pub fn filter(mut self, filter: Filter) -> Self {
        self.filters.push(filter);
        self
    }

    /// Add text search
    pub fn text(self, text: impl Into<String>) -> Self {
        self.filter(Filter::text(text))
    }

    /// Add field equality filter
    pub fn field_eq(self, field: impl Into<String>, value: impl Into<String>) -> Self {
        self.filter(Filter::eq(field, value))
    }

    /// Add field contains filter
    pub fn field_contains(self, field: impl Into<String>, value: impl Into<String>) -> Self {
        self.filter(Filter::contains(field, value))
    }

    /// Set sort
    pub fn sort_by(mut self, sort: SortBy) -> Self {
        self.sort = Some(sort);
        self
    }

    /// Sort ascending
    pub fn sort_asc(self, field: impl Into<String>) -> Self {
        self.sort_by(SortBy::asc(field))
    }

    /// Sort descending
    pub fn sort_desc(self, field: impl Into<String>) -> Self {
        self.sort_by(SortBy::desc(field))
    }

    /// Set limit
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set offset
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Check if an item matches all filters, ignoring case
    pub fn matches<T: Queryable>(&self, item: &T) -> bool {
        self.matches_with(item, Case::Insensitive)
    }

    /// Check if an item matches all filters, comparing strings by `case`
    pub fn matches_with<T: Queryable>(&self, item: &T, case: Case) -> bool {
        self.filters.iter().all(|f| f.matches_with(item, case))
    }

    /// Filter, sort and page a slice of items, ignoring case
    pub fn filter_items<'a, T: Queryable>(&self, items: &'a [T]) -> Vec<&'a T> {
        self.filter_items_with(items, Case::Insensitive)
    }

    /// Filter, sort and page a slice of items, comparing strings by `case`
    ///
    /// ```
    /// use revue::query::{Case, Query, QueryValue, Queryable};
    ///
    /// struct Lang(&'static str);
    /// impl Queryable for Lang {
    ///     fn field_value(&self, _: &str) -> Option<QueryValue> {
    ///         Some(QueryValue::String(self.0.into()))
    ///     }
    ///     fn full_text(&self) -> String {
    ///         self.0.into()
    ///     }
    /// }
    ///
    /// let langs = [Lang("Rust"), Lang("rust"), Lang("Go")];
    /// let names = |case| -> Vec<&str> {
    ///     Query::parse("rust").unwrap().filter_items_with(&langs, case).iter().map(|l| l.0).collect()
    /// };
    /// assert_eq!(names(Case::Insensitive), ["Rust", "rust"]);
    /// assert_eq!(names(Case::Sensitive), ["rust"]);
    /// // Smart: a lower-case query ignores case, an upper-case letter makes it exact.
    /// assert_eq!(names(Case::Smart), ["Rust", "rust"]);
    /// let exact: Vec<&str> = Query::parse("Rust").unwrap().filter_items_with(&langs, Case::Smart).iter().map(|l| l.0).collect();
    /// assert_eq!(exact, ["Rust"]);
    /// ```
    pub fn filter_items_with<'a, T: Queryable>(&self, items: &'a [T], case: Case) -> Vec<&'a T> {
        let mut result: Vec<_> = items
            .iter()
            .filter(|item| self.matches_with(*item, case))
            .collect();

        // Apply sorting
        if let Some(ref sort) = self.sort {
            result.sort_by(|a, b| {
                let a_val = a.field_value(&sort.field);
                let b_val = b.field_value(&sort.field);

                let ordering = match (&a_val, &b_val) {
                    (Some(QueryValue::String(a)), Some(QueryValue::String(b))) => {
                        case.cmp_for_sort(a, b)
                    }
                    (Some(QueryValue::Int(a)), Some(QueryValue::Int(b))) => a.cmp(b),
                    (Some(QueryValue::Float(a)), Some(QueryValue::Float(b))) => {
                        a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)
                    }
                    (Some(QueryValue::Date(a)), Some(QueryValue::Date(b))) => a.cmp(b),
                    (Some(_), None) => std::cmp::Ordering::Less,
                    (None, Some(_)) => std::cmp::Ordering::Greater,
                    _ => std::cmp::Ordering::Equal,
                };

                match sort.direction {
                    SortDirection::Ascending => ordering,
                    SortDirection::Descending => ordering.reverse(),
                }
            });
        }

        // Apply offset
        if let Some(offset) = self.offset {
            result = result.into_iter().skip(offset).collect();
        }

        // Apply limit
        if let Some(limit) = self.limit {
            result.truncate(limit);
        }

        result
    }

    /// Check if query is empty (matches everything)
    pub fn is_empty(&self) -> bool {
        self.filters.is_empty()
            && self.sort.is_none()
            && self.limit.is_none()
            && self.offset.is_none()
    }
}

/// Trait for items that can be queried
pub trait Queryable {
    /// Get field value by name
    fn field_value(&self, field: &str) -> Option<QueryValue>;

    /// Get full text representation for free-text search
    fn full_text(&self) -> String;
}

/// Helper macro for implementing Queryable
#[macro_export]
macro_rules! impl_queryable {
    ($type:ty, full_text: $full_text:expr, fields: { $($field:literal => $accessor:expr),* $(,)? }) => {
        impl $crate::query::Queryable for $type {
            fn field_value(&self, field: &str) -> Option<$crate::query::QueryValue> {
                match field {
                    $($field => Some($accessor(self)),)*
                    _ => None,
                }
            }

            fn full_text(&self) -> String {
                $full_text(self)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_with_offset_is_not_empty() {
        assert!(Query::new().is_empty());
        let query = Query::new().offset(2);
        assert!(!query.is_empty());

        // It does not match everything: it skips the first two items
        let items = [1, 2, 3].map(|age| TestItem {
            name: String::new(),
            age,
            active: true,
        });
        assert_eq!(query.filter_items(&items).len(), 1);
    }

    struct TestItem {
        name: String,
        age: i64,
        active: bool,
    }

    impl Queryable for TestItem {
        fn field_value(&self, field: &str) -> Option<QueryValue> {
            match field {
                "name" => Some(QueryValue::String(self.name.clone())),
                "age" => Some(QueryValue::Int(self.age)),
                "active" => Some(QueryValue::Bool(self.active)),
                _ => None,
            }
        }

        fn full_text(&self) -> String {
            self.name.clone()
        }
    }

    fn field(name: &str, op: Operator, value: &str) -> Filter {
        Filter::Field {
            name: name.into(),
            op,
            value: value.into(),
        }
    }

    fn named(name: &str) -> TestItem {
        TestItem {
            name: name.into(),
            age: 0,
            active: true,
        }
    }

    /// Equality ignores case, so ordering must too: whatever `name:x` matches,
    /// `name:>=x` and `name:<=x` match as well, and nothing is both `> x` and
    /// equal to it.
    #[test]
    fn string_comparisons_ignore_case_like_equality() {
        for (name, value) in [("Bob", "bob"), ("bob", "BOB"), ("Émile", "émile")] {
            let item = named(name);
            assert!(Filter::eq("name", value).matches(&item), "{name} = {value}");
            assert!(
                field("name", Operator::Ge, value).matches(&item),
                "{name} >= {value} although they are equal"
            );
            assert!(
                field("name", Operator::Le, value).matches(&item),
                "{name} <= {value} although they are equal"
            );
            assert!(
                !Filter::gt("name", value).matches(&item),
                "{name} > {value}"
            );
            assert!(
                !Filter::lt("name", value).matches(&item),
                "{name} < {value}"
            );
        }
        // Upper case no longer sorts before every lower-case letter.
        assert!(Filter::gt("name", "alice").matches(&named("Bob")));
        assert!(Filter::lt("name", "Carol").matches(&named("bob")));
    }

    #[test]
    fn every_comparison_follows_the_case_rule() {
        let bob = named("Bob");
        let cases = [
            // (filter, Insensitive, Sensitive, Smart)
            (Filter::eq("name", "bob"), true, false, true),
            (Filter::eq("name", "BOB"), true, false, false),
            (Filter::eq("name", "Bob"), true, true, true),
            (Filter::ne("name", "bob"), false, true, false),
            (Filter::contains("name", "ob"), true, true, true),
            (Filter::contains("name", "OB"), true, false, false),
            (Filter::text("bob"), true, false, true),
            (Filter::text("BOB"), true, false, false),
            (field("name", Operator::Ge, "bob"), true, false, true),
            // Sensitive: "B" (0x42) sorts before "a" (0x61).
            (Filter::lt("name", "alice"), false, true, false),
        ];
        for (filter, insensitive, sensitive, smart) in cases {
            assert_eq!(
                filter.matches_with(&bob, Case::Insensitive),
                insensitive,
                "{filter:?} Insensitive"
            );
            assert_eq!(
                filter.matches_with(&bob, Case::Sensitive),
                sensitive,
                "{filter:?} Sensitive"
            );
            assert_eq!(
                filter.matches_with(&bob, Case::Smart),
                smart,
                "{filter:?} Smart"
            );
            // The default is Insensitive.
            assert_eq!(filter.matches(&bob), insensitive, "{filter:?} default");
        }
    }

    #[test]
    fn sorting_follows_the_case_rule() {
        let items = ["banana", "Cherry", "apple"].map(named);
        let sorted = |case| -> Vec<String> {
            Query::new()
                .sort_asc("name")
                .filter_items_with(&items, case)
                .iter()
                .map(|i| i.name.clone())
                .collect()
        };
        assert_eq!(sorted(Case::Insensitive), ["apple", "banana", "Cherry"]);
        assert_eq!(sorted(Case::Smart), ["apple", "banana", "Cherry"]);
        assert_eq!(sorted(Case::Sensitive), ["Cherry", "apple", "banana"]);
    }

    #[test]
    fn sorting_strings_ignores_case_and_is_stable_on_ties() {
        let items = ["banana", "Cherry", "apple", "Banana", "cherry"].map(named);
        let names = |q: Query| -> Vec<String> {
            q.filter_items(&items)
                .iter()
                .map(|i| i.name.clone())
                .collect()
        };
        assert_eq!(
            names(Query::new().sort_asc("name")),
            ["apple", "Banana", "banana", "Cherry", "cherry"]
        );
        assert_eq!(
            names(Query::new().sort_desc("name")),
            ["cherry", "Cherry", "banana", "Banana", "apple"]
        );
        // A range filter selects a contiguous run of the sorted order.
        assert_eq!(
            names(
                Query::new()
                    .filter(field("name", Operator::Ge, "BANANA"))
                    .sort_asc("name")
            ),
            ["Banana", "banana", "Cherry", "cherry"]
        );
    }

    #[test]
    fn test_text_filter() {
        let item = TestItem {
            name: "John Doe".into(),
            age: 30,
            active: true,
        };

        assert!(Filter::text("john").matches(&item));
        assert!(Filter::text("doe").matches(&item));
        assert!(Filter::text("john doe").matches(&item));
        assert!(!Filter::text("jane").matches(&item));
    }

    #[test]
    fn test_field_eq() {
        let item = TestItem {
            name: "John".into(),
            age: 30,
            active: true,
        };

        assert!(Filter::eq("name", "john").matches(&item));
        assert!(Filter::eq("age", "30").matches(&item));
        assert!(Filter::eq("active", "true").matches(&item));
        assert!(!Filter::eq("name", "jane").matches(&item));
    }

    #[test]
    fn test_field_ne() {
        let item = TestItem {
            name: "John".into(),
            age: 30,
            active: true,
        };

        assert!(Filter::ne("name", "jane").matches(&item));
        assert!(!Filter::ne("name", "john").matches(&item));
    }

    #[test]
    fn test_field_contains() {
        let item = TestItem {
            name: "John Doe".into(),
            age: 30,
            active: true,
        };

        assert!(Filter::contains("name", "oh").matches(&item));
        assert!(!Filter::contains("name", "xyz").matches(&item));
    }

    #[test]
    fn test_field_comparison() {
        let item = TestItem {
            name: "John".into(),
            age: 30,
            active: true,
        };

        assert!(Filter::gt("age", "20").matches(&item));
        assert!(!Filter::gt("age", "40").matches(&item));
        assert!(Filter::lt("age", "40").matches(&item));
        assert!(!Filter::lt("age", "20").matches(&item));
    }

    #[test]
    fn test_and_or() {
        let item = TestItem {
            name: "John".into(),
            age: 30,
            active: true,
        };

        let and_filter = Filter::eq("name", "john").and(Filter::eq("active", "true"));
        assert!(and_filter.matches(&item));

        let or_filter = Filter::eq("name", "jane").or(Filter::eq("name", "john"));
        assert!(or_filter.matches(&item));
    }

    #[test]
    fn test_not() {
        let item = TestItem {
            name: "John".into(),
            age: 30,
            active: true,
        };

        assert!(Filter::eq("name", "jane").negate().matches(&item));
        assert!(!Filter::eq("name", "john").negate().matches(&item));
    }

    #[test]
    fn test_query() {
        let items = vec![
            TestItem {
                name: "Alice".into(),
                age: 25,
                active: true,
            },
            TestItem {
                name: "Bob".into(),
                age: 30,
                active: false,
            },
            TestItem {
                name: "Charlie".into(),
                age: 35,
                active: true,
            },
        ];

        let query = Query::new().filter(Filter::eq("active", "true"));
        let result = query.filter_items(&items);
        assert_eq!(result.len(), 2);

        let query = Query::new().filter(Filter::gt("age", "27"));
        let result = query.filter_items(&items);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_query_sort() {
        let items = vec![
            TestItem {
                name: "Charlie".into(),
                age: 35,
                active: true,
            },
            TestItem {
                name: "Alice".into(),
                age: 25,
                active: true,
            },
            TestItem {
                name: "Bob".into(),
                age: 30,
                active: false,
            },
        ];

        let query = Query::new().sort_asc("age");
        let result = query.filter_items(&items);
        assert_eq!(result[0].name, "Alice");
        assert_eq!(result[1].name, "Bob");
        assert_eq!(result[2].name, "Charlie");

        let query = Query::new().sort_desc("age");
        let result = query.filter_items(&items);
        assert_eq!(result[0].name, "Charlie");
    }

    #[test]
    fn test_query_limit_offset() {
        let items = vec![
            TestItem {
                name: "A".into(),
                age: 1,
                active: true,
            },
            TestItem {
                name: "B".into(),
                age: 2,
                active: true,
            },
            TestItem {
                name: "C".into(),
                age: 3,
                active: true,
            },
            TestItem {
                name: "D".into(),
                age: 4,
                active: true,
            },
        ];

        let query = Query::new().limit(2);
        let result = query.filter_items(&items);
        assert_eq!(result.len(), 2);

        let query = Query::new().offset(1).limit(2);
        let result = query.filter_items(&items);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].name, "B");
    }

    // QueryValue method tests
    #[test]
    fn test_query_value_string() {
        let val = QueryValue::string("hello");
        assert_eq!(val, QueryValue::String("hello".to_string()));
    }

    #[test]
    fn test_query_value_int() {
        let val = QueryValue::int(42);
        assert_eq!(val, QueryValue::Int(42));
    }

    #[test]
    fn test_query_value_float() {
        let val = QueryValue::float(1.25);
        assert_eq!(val, QueryValue::Float(1.25));
    }

    #[test]
    fn test_query_value_bool() {
        let val = QueryValue::bool(true);
        assert_eq!(val, QueryValue::Bool(true));
    }

    #[test]
    fn test_query_value_contains_string() {
        let val = QueryValue::string("Hello World");
        assert!(val.contains("hello"));
        assert!(val.contains("WORLD"));
        assert!(!val.contains("xyz"));
    }

    #[test]
    fn test_query_value_contains_int() {
        let val = QueryValue::int(12345);
        assert!(val.contains("123"));
        assert!(!val.contains("999"));
    }

    #[test]
    fn test_query_value_contains_float() {
        let val = QueryValue::float(1.25);
        assert!(val.contains("1.25"));
        assert!(val.contains("1"));
        assert!(!val.contains("999"));
    }

    #[test]
    fn test_query_value_contains_bool() {
        let val = QueryValue::bool(true);
        assert!(val.contains("true"));
        assert!(val.contains("TRUE"));
        assert!(!val.contains("false"));

        let val = QueryValue::bool(false);
        assert!(val.contains("false"));
        assert!(!val.contains("true"));
    }

    #[test]
    fn test_query_value_contains_null() {
        let val = QueryValue::Null;
        assert!(!val.contains("anything"));
    }

    #[test]
    fn test_query_value_equals_str_string() {
        let val = QueryValue::string("Hello");
        assert!(val.equals_str("hello"));
        assert!(val.equals_str("HELLO"));
        assert!(!val.equals_str("world"));
    }

    #[test]
    fn test_query_value_equals_str_int() {
        let val = QueryValue::int(42);
        assert!(val.equals_str("42"));
        assert!(!val.equals_str("999"));
    }

    #[test]
    fn test_query_value_equals_str_bool() {
        let val = QueryValue::bool(true);
        assert!(val.equals_str("true"));
        assert!(!val.equals_str("false"));
    }

    #[test]
    fn test_query_value_equals_str_null() {
        let val = QueryValue::Null;
        assert!(val.equals_str("null"));
        assert!(val.equals_str(""));
        assert!(!val.equals_str("value"));
    }

    #[test]
    fn test_query_value_compare_int() {
        let val = QueryValue::int(50);
        assert_eq!(val.compare("30"), Some(std::cmp::Ordering::Greater));
        assert_eq!(val.compare("50"), Some(std::cmp::Ordering::Equal));
        assert_eq!(val.compare("70"), Some(std::cmp::Ordering::Less));
    }

    #[test]
    fn test_query_value_compare_float() {
        let val = QueryValue::float(3.5);
        assert_eq!(val.compare("2.5"), Some(std::cmp::Ordering::Greater));
        assert_eq!(val.compare("3.5"), Some(std::cmp::Ordering::Equal));
        assert_eq!(val.compare("4.5"), Some(std::cmp::Ordering::Less));
    }
}

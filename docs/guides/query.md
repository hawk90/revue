# Query Guide

Revue provides a powerful query DSL for filtering, sorting, and paginating data collections.

## Overview

The Query system lets you filter items using a string-based query language or a programmatic builder API. It supports free-text search, field comparisons, boolean combinators, sorting, and pagination.

## Quick Start

```rust
use revue::query::{Query, Queryable, QueryValue};

struct Article {
    title: String,
    author: String,
    views: i64,
    published: bool,
}

impl Queryable for Article {
    fn field_value(&self, field: &str) -> Option<QueryValue> {
        match field {
            "title" => Some(QueryValue::string(&self.title)),
            "author" => Some(QueryValue::string(&self.author)),
            "views" => Some(QueryValue::int(self.views)),
            "published" => Some(QueryValue::bool(self.published)),
            _ => None,
        }
    }

    fn full_text(&self) -> String {
        format!("{} {}", self.title, self.author)
    }
}

// Parse a query string
let query = Query::parse("author:john published:true").unwrap();
let results = query.filter_items(&articles);
```

## Query Syntax

### Free Text Search

Matches items whose `full_text()` contains all given words (case-insensitive):

```
hello world          // Items containing both "hello" AND "world"
rust programming     // Items containing both "rust" AND "programming"
```

### Field Match

Exact field comparison (case-insensitive for strings):

```
author:john          // author equals "john"
status:active        // status equals "active"
active:true          // boolean field check
age:30               // numeric equality
```

### Field Contains

Substring match within a field:

```
title~rust           // title contains "rust"
name~doe             // name contains "doe"
```

### Not Equal

Negated equality:

```
status:!draft        // status is NOT "draft"
published:!false     // published is NOT false
```

### Numeric Comparisons

```
age:>18              // age greater than 18
price:<100           // price less than 100
age:>=21             // age greater than or equal to 21
price:<=50           // price less than or equal to 50
```

### Date Filters

```
after:2024-01-01     // date after 2024-01-01
before:2024-12-31    // date before 2024-12-31
```

### Boolean Values

```
active:true          // Matches: true, yes, 1
active:false         // Matches: false, no, 0
```

## Builder API

Build queries programmatically:

```rust
use revue::query::{Query, Filter, SortBy};

let query = Query::new()
    .field_eq("status", "active")
    .field_contains("title", "rust")
    .sort_asc("date")
    .limit(10)
    .offset(20);
```

### Query Methods

| Method | Description |
|--------|-------------|
| `Query::new()` | Empty query (matches everything) |
| `Query::parse(input)` | Parse a query string |
| `.filter(f)` | Add a `Filter` |
| `.text(s)` | Add free-text search |
| `.field_eq(field, value)` | Add equality filter |
| `.field_contains(field, value)` | Add contains filter |
| `.sort_by(sort)` | Set sort specification |
| `.sort_asc(field)` | Sort ascending by field |
| `.sort_desc(field)` | Sort descending by field |
| `.limit(n)` | Limit to N results |
| `.offset(n)` | Skip first N results |
| `.matches(item)` | Check if item matches all filters |
| `.filter_items(items)` | Filter, sort, and paginate a slice |
| `.is_empty()` | True if no filters, sort, or limit set |

## Filter Combinators

Build complex filters with AND, OR, and NOT:

```rust
use revue::query::Filter;

// AND: both conditions must match
let filter = Filter::eq("status", "active")
    .and(Filter::gt("age", "18"));

// OR: either condition matches
let filter = Filter::eq("role", "admin")
    .or(Filter::eq("role", "moderator"));

// NOT: negate a condition
let filter = Filter::eq("status", "banned").negate();

// Complex combinations
let filter = Filter::eq("published", "true")
    .and(
        Filter::contains("title", "rust")
            .or(Filter::contains("title", "cargo"))
    );
```

### Filter Constructors

| Constructor | Syntax Equivalent |
|-------------|-------------------|
| `Filter::text("hello")` | `hello` |
| `Filter::eq("field", "value")` | `field:value` |
| `Filter::ne("field", "value")` | `field:!value` |
| `Filter::contains("field", "val")` | `field~val` |
| `Filter::gt("field", "10")` | `field:>10` |
| `Filter::lt("field", "10")` | `field:<10` |

## QueryValue Types

Fields return typed values for proper comparison:

```rust
use revue::query::QueryValue;

QueryValue::string("hello")      // String comparison (case-insensitive)
QueryValue::int(42)              // Integer comparison
QueryValue::float(3.14)          // Float comparison
QueryValue::bool(true)           // Boolean comparison
QueryValue::Date("2024-01-01".into())  // Date comparison
QueryValue::Null                 // Null value
```

## Implementing Queryable

### Manual Implementation

```rust
use revue::query::{Queryable, QueryValue};

struct User {
    name: String,
    email: String,
    age: i64,
    active: bool,
}

impl Queryable for User {
    fn field_value(&self, field: &str) -> Option<QueryValue> {
        match field {
            "name" => Some(QueryValue::string(&self.name)),
            "email" => Some(QueryValue::string(&self.email)),
            "age" => Some(QueryValue::int(self.age)),
            "active" => Some(QueryValue::bool(self.active)),
            _ => None,
        }
    }

    fn full_text(&self) -> String {
        format!("{} {}", self.name, self.email)
    }
}
```

### Using the Macro

```rust
use revue::impl_queryable;

impl_queryable!(User,
    full_text: |u: &User| format!("{} {}", u.name, u.email),
    fields: {
        "name" => |u: &User| QueryValue::string(&u.name),
        "email" => |u: &User| QueryValue::string(&u.email),
        "age" => |u: &User| QueryValue::int(u.age),
        "active" => |u: &User| QueryValue::bool(u.active),
    }
);
```

## Sorting

Sort results by any queryable field:

```rust
let query = Query::new()
    .filter(Filter::eq("active", "true"))
    .sort_asc("name");    // A-Z

let query = Query::new()
    .sort_desc("created"); // Newest first
```

Sorting works with String, Int, Float, and Date values. Items with missing field values sort after items with values.

## Pagination

Combine `offset` and `limit` for pagination:

```rust
let page = 3;
let per_page = 20;

let query = Query::new()
    .sort_asc("name")
    .offset((page - 1) * per_page)
    .limit(per_page);

let results = query.filter_items(&all_items);
```

## Integration with Widgets

### With DataGrid

```rust
use revue::query::Query;

let query = Query::parse(&search_input).unwrap_or_default();
let filtered: Vec<_> = query.filter_items(&data);

let grid = DataGrid::new()
    .rows(filtered.iter().map(|item| {
        vec![item.name.clone(), item.age.to_string()]
    }).collect());
```

### With VirtualList

```rust
let query = Query::new()
    .text(&search_term)
    .sort_asc("name");

let items: Vec<_> = query.filter_items(&all_items)
    .into_iter()
    .cloned()
    .collect();

let list = VirtualList::new(items, |item, ctx| {
    // render each item
});
```

## Error Handling

`Query::parse` returns a `Result<Query, ParseError>`:

```rust
match Query::parse("invalid:>:>syntax") {
    Ok(query) => { /* use query */ }
    Err(e) => eprintln!("Parse error: {}", e),
}

// Or use default (matches everything) on parse failure
let query = Query::parse(&input).unwrap_or_default();
```

## See Also

- [State Management Guide](state.md) - Reactive state with Signals
- [Store Guide](store.md) - Centralized state management

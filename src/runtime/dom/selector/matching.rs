//! The one selector matcher.
//!
//! There used to be two - one behind [`DomTree::query`](crate::dom::DomTree)
//! and one inside the cascade - and they disagreed in both directions. The
//! query matcher backtracked but ignored attribute selectors; the cascade
//! matcher checked attributes but committed to the first candidate it found, so
//! `.outer > .mid .leaf` failed whenever the *nearest* `.mid` was not the one
//! that satisfied the `>`. Only the cascade paints, so its answer is the one
//! users saw, and `query()` reported a different set.
//!
//! Matching is right-to-left and recursive, because the "any" combinators
//! (descendant and general sibling) need to try every candidate rather than the
//! first: a candidate that matches its own part can still fail further left.

use crate::dom::selector::{AttributeOp, AttributeSelector, Combinator, Selector, SelectorPart};
use crate::dom::{DomId, DomNode};

/// Does `selector` match `node`?
///
/// `get_node` resolves a [`DomId`] to its node, which is what lets the tree and
/// the cascade share this - they reach their nodes differently.
/// `node` is deliberately *not* tied to `'a`: candidates fetched through
/// `get_node` outlive the call and coerce down when the recursion passes them
/// on, so a caller can match a node it borrowed for less time than the tree.
pub(crate) fn matches<'a, F>(node: &DomNode, selector: &Selector, get_node: &F) -> bool
where
    F: Fn(DomId) -> Option<&'a DomNode>,
{
    if selector.parts.is_empty() {
        return false;
    }
    matches_from(node, selector, selector.parts.len() - 1, get_node)
}

/// Match `selector`'s parts from `part_idx` leftwards, with `node` as the
/// candidate for `part_idx`.
fn matches_from<'a, F>(node: &DomNode, selector: &Selector, part_idx: usize, get_node: &F) -> bool
where
    F: Fn(DomId) -> Option<&'a DomNode>,
{
    let (part, _) = &selector.parts[part_idx];

    if !matches_part(part, node) {
        return false;
    }

    // Leftmost part matched, so the whole selector did.
    if part_idx == 0 {
        return true;
    }

    // The combinator joining this part to the one on its left is stored on that
    // left part.
    match selector.parts[part_idx - 1].1 {
        Some(Combinator::Descendant) => {
            // Any ancestor. Trying only the nearest match is not enough: it can
            // satisfy its own part and still fail further left.
            let mut current = node.parent;
            while let Some(parent_id) = current {
                let Some(parent) = get_node(parent_id) else {
                    break;
                };
                if matches_from(parent, selector, part_idx - 1, get_node) {
                    return true;
                }
                current = parent.parent;
            }
            false
        }
        Some(Combinator::Child) => node
            .parent
            .and_then(get_node)
            .is_some_and(|parent| matches_from(parent, selector, part_idx - 1, get_node)),
        Some(Combinator::AdjacentSibling) => previous_sibling(node, get_node)
            .is_some_and(|prev| matches_from(prev, selector, part_idx - 1, get_node)),
        Some(Combinator::GeneralSibling) => {
            // Any earlier sibling, same reasoning as descendant.
            let mut current = previous_sibling(node, get_node);
            while let Some(sibling) = current {
                if matches_from(sibling, selector, part_idx - 1, get_node) {
                    return true;
                }
                current = previous_sibling(sibling, get_node);
            }
            false
        }
        // Only the rightmost part has no combinator, and `part_idx == 0` above
        // already returned for the leftmost.
        None => true,
    }
}

/// Does one compound part match this node, ignoring combinators?
pub(crate) fn matches_part(part: &SelectorPart, node: &DomNode) -> bool {
    // `*` on its own matches everything; `*.foo` still has to check `.foo`.
    if part.universal
        && part.id.is_none()
        && part.classes.is_empty()
        && part.pseudo_classes.is_empty()
        && part.element.is_none()
        && part.attributes.is_empty()
    {
        return true;
    }

    if let Some(ref elem) = part.element {
        if node.widget_type() != elem {
            return false;
        }
    }

    if let Some(ref id) = part.id {
        if node.element_id() != Some(id.as_str()) {
            return false;
        }
    }

    for class in &part.classes {
        if !node.has_class(class) {
            return false;
        }
    }

    for pseudo in &part.pseudo_classes {
        if !node.matches_pseudo(pseudo) {
            return false;
        }
    }

    for attr in &part.attributes {
        if !matches_attribute(attr, node) {
            return false;
        }
    }

    true
}

/// The sibling immediately before `node`, if any.
fn previous_sibling<'a, F>(node: &DomNode, get_node: &F) -> Option<&'a DomNode>
where
    F: Fn(DomId) -> Option<&'a DomNode>,
{
    let parent = get_node(node.parent?)?;
    let idx = parent.children.iter().position(|&id| id == node.id)?;
    get_node(*parent.children.get(idx.checked_sub(1)?)?)
}

/// Does an attribute selector match this node?
///
/// The DOM has no attribute bag, so these map onto what a node actually has:
/// `class`, `id`, `type` and the boolean states.
fn matches_attribute(attr: &AttributeSelector, node: &DomNode) -> bool {
    use AttributeOp;

    // Helper for case-insensitive comparison
    let compare = |a: &str, b: &str, case_insensitive: bool| -> bool {
        if case_insensitive {
            a.eq_ignore_ascii_case(b)
        } else {
            a == b
        }
    };

    match attr.name.as_str() {
        "class" => {
            match &attr.op {
                AttributeOp::Exists => !node.meta.classes.is_empty(),
                AttributeOp::ContainsWord => {
                    if let Some(ref val) = attr.value {
                        node.meta
                            .classes
                            .iter()
                            .any(|c| compare(c, val, attr.case_insensitive))
                    } else {
                        false
                    }
                }
                AttributeOp::Equals => {
                    // Exact match: classes joined with space equals value
                    if let Some(ref val) = attr.value {
                        let classes: Vec<_> = node.meta.classes.iter().collect();
                        let joined = classes
                            .iter()
                            .map(|s| s.as_str())
                            .collect::<Vec<_>>()
                            .join(" ");
                        compare(&joined, val, attr.case_insensitive)
                    } else {
                        false
                    }
                }
                AttributeOp::Contains => {
                    if let Some(ref val) = attr.value {
                        node.meta.classes.iter().any(|c| {
                            if attr.case_insensitive {
                                c.to_lowercase().contains(val.to_lowercase().as_str())
                            } else {
                                c.contains(val.as_str())
                            }
                        })
                    } else {
                        false
                    }
                }
                AttributeOp::StartsWith => {
                    if let Some(ref val) = attr.value {
                        node.meta.classes.iter().any(|c| {
                            if attr.case_insensitive {
                                c.to_lowercase().starts_with(&val.to_lowercase())
                            } else {
                                c.starts_with(val.as_str())
                            }
                        })
                    } else {
                        false
                    }
                }
                AttributeOp::EndsWith => {
                    if let Some(ref val) = attr.value {
                        node.meta.classes.iter().any(|c| {
                            if attr.case_insensitive {
                                c.to_lowercase().ends_with(&val.to_lowercase())
                            } else {
                                c.ends_with(val.as_str())
                            }
                        })
                    } else {
                        false
                    }
                }
                AttributeOp::StartsWithWord => {
                    // CSS |= operator: exact match or starts with value followed by hyphen
                    if let Some(ref val) = attr.value {
                        node.meta.classes.iter().any(|c| {
                            compare(c, val, attr.case_insensitive) || {
                                let prefix = format!("{}-", val);
                                if attr.case_insensitive {
                                    c.to_lowercase().starts_with(&prefix.to_lowercase())
                                } else {
                                    c.starts_with(&prefix)
                                }
                            }
                        })
                    } else {
                        false
                    }
                }
            }
        }
        "id" => {
            let node_id = node.element_id().unwrap_or("");
            match &attr.op {
                AttributeOp::Exists => node.element_id().is_some(),
                AttributeOp::Equals => {
                    if let Some(ref val) = attr.value {
                        compare(node_id, val, attr.case_insensitive)
                    } else {
                        false
                    }
                }
                AttributeOp::StartsWith => {
                    if let Some(ref val) = attr.value {
                        if attr.case_insensitive {
                            node_id.to_lowercase().starts_with(&val.to_lowercase())
                        } else {
                            node_id.starts_with(val.as_str())
                        }
                    } else {
                        false
                    }
                }
                AttributeOp::EndsWith => {
                    if let Some(ref val) = attr.value {
                        if attr.case_insensitive {
                            node_id.to_lowercase().ends_with(&val.to_lowercase())
                        } else {
                            node_id.ends_with(val.as_str())
                        }
                    } else {
                        false
                    }
                }
                AttributeOp::Contains => {
                    if let Some(ref val) = attr.value {
                        if attr.case_insensitive {
                            node_id.to_lowercase().contains(val.to_lowercase().as_str())
                        } else {
                            node_id.contains(val.as_str())
                        }
                    } else {
                        false
                    }
                }
                _ => false,
            }
        }
        "type" => {
            // Widget type matching
            let widget_type = node.widget_type();
            match &attr.op {
                AttributeOp::Exists => !widget_type.is_empty(),
                AttributeOp::Equals => {
                    if let Some(ref val) = attr.value {
                        compare(widget_type, val, attr.case_insensitive)
                    } else {
                        false
                    }
                }
                AttributeOp::Contains => {
                    if let Some(ref val) = attr.value {
                        if attr.case_insensitive {
                            widget_type
                                .to_lowercase()
                                .contains(val.to_lowercase().as_str())
                        } else {
                            widget_type.contains(val.as_str())
                        }
                    } else {
                        false
                    }
                }
                _ => false,
            }
        }
        "disabled" => match &attr.op {
            AttributeOp::Exists => node.state.disabled,
            AttributeOp::Equals => {
                if let Some(ref val) = attr.value {
                    let is_true = val.as_str() == "true" || val.as_str() == "1" || val.is_empty();
                    node.state.disabled == is_true
                } else {
                    node.state.disabled
                }
            }
            _ => false,
        },
        "checked" => match &attr.op {
            AttributeOp::Exists => node.state.checked,
            AttributeOp::Equals => {
                if let Some(ref val) = attr.value {
                    let is_true = val.as_str() == "true" || val.as_str() == "1" || val.is_empty();
                    node.state.checked == is_true
                } else {
                    node.state.checked
                }
            }
            _ => false,
        },
        "selected" => match &attr.op {
            AttributeOp::Exists => node.state.selected,
            AttributeOp::Equals => {
                if let Some(ref val) = attr.value {
                    let is_true = val.as_str() == "true" || val.as_str() == "1" || val.is_empty();
                    node.state.selected == is_true
                } else {
                    node.state.selected
                }
            }
            _ => false,
        },
        "focused" | "focus" => match &attr.op {
            AttributeOp::Exists => node.state.focused,
            _ => false,
        },
        "hovered" | "hover" => match &attr.op {
            AttributeOp::Exists => node.state.hovered,
            _ => false,
        },
        _ => false,
    }
}

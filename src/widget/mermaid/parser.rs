//! Mermaid-like syntax parser

use super::{edge::DiagramEdge, node::DiagramNode};

/// Mermaid-like syntax parser
pub struct MermaidParser;

impl MermaidParser {
    /// Parse node definition like `A[Label]` or `B{Decision}`
    pub fn parse_node_def(s: &str) -> (String, Option<String>) {
        let s = s.trim();

        // [Label]
        if let Some(bracket_start) = s.find('[') {
            if let Some(bracket_end) = s.find(']') {
                let id = s[..bracket_start].trim().to_string();
                let label = s[bracket_start + 1..bracket_end].to_string();
                return (id, Some(label));
            }
        }

        // {Label}
        if let Some(brace_start) = s.find('{') {
            if let Some(brace_end) = s.find('}') {
                let id = s[..brace_start].trim().to_string();
                let label = s[brace_start + 1..brace_end].to_string();
                return (id, Some(label));
            }
        }

        // (Label)
        if let Some(paren_start) = s.find('(') {
            if let Some(paren_end) = s.rfind(')') {
                let id = s[..paren_start].trim().to_string();
                let label = s[paren_start + 1..paren_end].to_string();
                return (id, Some(label));
            }
        }

        (s.to_string(), None)
    }
}

/// Extension trait for Diagram to add parsing functionality
pub trait ParseMermaid {
    /// Parse mermaid-like syntax
    fn parse(self, source: &str) -> Self;
}

impl ParseMermaid for super::Diagram {
    fn parse(mut self, source: &str) -> Self {
        use std::collections::HashMap;

        for line in source.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("%%") {
                continue;
            }

            // Parse arrows: A --> B or A -->|label| B
            if let Some((left, right)) = line.split_once("-->") {
                let from = left.trim();
                let (label, to) = if right.contains('|') {
                    let parts: Vec<&str> = right.split('|').collect();
                    if parts.len() >= 3 {
                        (Some(parts[1].to_string()), parts[2].trim())
                    } else {
                        (None, right.trim())
                    }
                } else {
                    (None, right.trim())
                };

                // Extract node IDs and labels
                let (from_id, from_label) = MermaidParser::parse_node_def(from);
                let (to_id, to_label) = MermaidParser::parse_node_def(to);

                // Add nodes if not exists
                if !self.nodes.iter().any(|n| n.id == from_id) {
                    self.nodes.push(DiagramNode::new(
                        &from_id,
                        from_label.unwrap_or_else(|| from_id.clone()),
                    ));
                }
                if !self.nodes.iter().any(|n| n.id == to_id) {
                    self.nodes.push(DiagramNode::new(
                        &to_id,
                        to_label.unwrap_or_else(|| to_id.clone()),
                    ));
                }

                let mut edge = DiagramEdge::new(from_id, to_id);
                if let Some(l) = label {
                    edge = edge.label(l);
                }
                self.edges.push(edge);
            }
        }
        self
    }
}

//! Layout computation for diagrams

use std::collections::HashMap;

/// Extension trait for Diagram to add layout computation
pub trait ComputeLayout {
    /// Compute layout for nodes
    fn compute_layout(&mut self, width: u16, height: u16);
}

impl ComputeLayout for super::Diagram {
    fn compute_layout(&mut self, width: u16, height: u16) {
        self.positions.clear();
        self.sizes.clear();

        if self.nodes.is_empty() {
            return;
        }

        // Simple grid layout
        let rows = ((self.nodes.len() as f32).sqrt().ceil() as u16).max(1);
        let cols = (self.nodes.len() as u16).div_ceil(rows).max(1);

        let cell_width = width / cols;
        let cell_height = height / rows;

        for (i, node) in self.nodes.iter().enumerate() {
            let row = i as u16 / cols;
            let col = i as u16 % cols;

            let node_width = (node.label.chars().count() as u16 + 4).min(cell_width - 2);
            let node_height = 3u16;

            let x = col * cell_width + (cell_width - node_width) / 2;
            let y = row * cell_height + (cell_height - node_height) / 2;

            self.positions.insert(node.id.clone(), (x, y));
            self.sizes.insert(node.id.clone(), (node_width, node_height));
        }
    }
}

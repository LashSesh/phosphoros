//! Infogenom - Network of coupled Gabriel Cells

use super::GabrielCell;
use crate::geometry::Point5D;

/// Network of coupled Gabriel Cells forming an Infogenom
pub struct Infogenom {
    /// Infogenom identifier
    pub id: String,
    /// Collection of Gabriel Cells
    pub(crate) cells: Vec<GabrielCell>,
    /// Coupling strength between cells
    pub coupling_strength: f64,
}

impl Infogenom {
    /// Create a new Infogenom with specified number of cells
    pub fn new(id: String, num_cells: usize) -> Self {
        let cells = (0..num_cells)
            .map(|i| GabrielCell::new(format!("{}:cell_{}", id, i)))
            .collect();

        Self {
            id,
            cells,
            coupling_strength: 0.5,
        }
    }

    /// Evaluate all cells with coupling effects
    pub fn evaluate(&mut self, embedding: &Point5D) -> Vec<f64> {
        let mut scores = Vec::new();

        for i in 0..self.cells.len() {
            let score = self.cells[i].evaluate(embedding);
            scores.push(score);

            // Couple to neighbors
            if i > 0 {
                let neighbor_output = self.cells[i - 1].output;
                self.cells[i].feedback(neighbor_output * self.coupling_strength);
            }
        }

        scores
    }

    /// Calculate total weighted resonance
    pub fn total_resonance(&mut self, embedding: &Point5D) -> f64 {
        let scores = self.evaluate(embedding);
        scores.iter().sum::<f64>() / scores.len() as f64
    }

    /// Contract keyspace using Solve-Coagula threshold
    pub fn contract_keyspace(&mut self, keyspace: Vec<Point5D>, threshold: f64) -> Vec<Point5D> {
        keyspace
            .into_iter()
            .filter(|emb| self.total_resonance(emb) >= threshold)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_infogenom_creation() {
        let infogenom = Infogenom::new("test".to_string(), 8);
        assert_eq!(infogenom.cells.len(), 8);
        assert_eq!(infogenom.coupling_strength, 0.5);
    }

    #[test]
    fn test_infogenom_evaluation() {
        let mut infogenom = Infogenom::new("test".to_string(), 4);
        let point = Point5D::new(0.5, 0.5, 0.5, 0.5, 0.5);
        let scores = infogenom.evaluate(&point);
        assert_eq!(scores.len(), 4);
    }
}

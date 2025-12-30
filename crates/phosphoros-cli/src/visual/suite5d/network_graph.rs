use rand::prelude::*;

const EDGE_PROBABILITY: f64 = 0.15;

#[derive(Debug, Clone, PartialEq)]
pub struct CellNode {
    pub id: String,
    pub cell_type: String,
    pub activity: f64,
}

impl CellNode {
    pub fn new(id: impl Into<String>, cell_type: impl Into<String>, activity: f64) -> Self {
        Self {
            id: id.into(),
            cell_type: cell_type.into(),
            activity,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CellNetworkSnapshot {
    pub nodes: Vec<CellNode>,
    pub edges: Vec<(String, String)>,
}

#[derive(Debug, Default, Clone)]
pub struct CellNetworkGraph {
    nodes: Vec<CellNode>,
    edges: Vec<(usize, usize)>,
}

impl CellNetworkGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn simulate_with_rng<R: Rng + ?Sized>(&mut self, cells: &[CellNode], rng: &mut R) {
        self.simulate_with_probability(cells, rng, EDGE_PROBABILITY);
    }

    pub fn simulate_with_probability<R: Rng + ?Sized>(
        &mut self,
        cells: &[CellNode],
        rng: &mut R,
        edge_probability: f64,
    ) {
        self.nodes = cells.to_vec();
        self.edges.clear();
        let len = self.nodes.len();
        for i in 0..len {
            for j in (i + 1)..len {
                if rng.gen::<f64>() < edge_probability {
                    self.edges.push((i, j));
                }
            }
        }
    }

    pub fn simulate(&mut self, cells: &[CellNode]) {
        let mut rng = thread_rng();
        self.simulate_with_rng(cells, &mut rng);
    }

    pub fn snapshot(&self) -> CellNetworkSnapshot {
        let edges = self
            .edges
            .iter()
            .map(|(a, b)| (self.nodes[*a].id.clone(), self.nodes[*b].id.clone()))
            .collect();
        CellNetworkSnapshot {
            nodes: self.nodes.clone(),
            edges,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn network_builds_edges() {
        let mut graph = CellNetworkGraph::new();
        let cells = vec![
            CellNode::new("A", "NavigatorCell", 0.5),
            CellNode::new("B", "MutatorCell", 0.8),
            CellNode::new("C", "WatcherCell", 0.2),
        ];
        let mut rng = StdRng::seed_from_u64(42);
        graph.simulate_with_probability(&cells, &mut rng, 1.0);
        let snapshot = graph.snapshot();
        assert_eq!(snapshot.nodes.len(), 3);
        assert_eq!(snapshot.edges.len(), 3);
    }
}

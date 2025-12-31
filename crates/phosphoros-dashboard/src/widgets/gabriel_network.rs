//! Gabriel Cell Network Visualization Widget
//!
//! Renders a network graph of coupled Gabriel Cells showing:
//! - Cell nodes with (ψ, ρ, ω) state coloring
//! - Coupling connections between cells
//! - Real-time animation of cell activity
//! - Resonance flow visualization

use iced::widget::canvas::{self, Canvas, Frame, Geometry, Path, Stroke, Fill};
use iced::{Element, Length, Point, Size, Color, Theme, Rectangle, Renderer};
use iced::mouse;
use std::f32::consts::PI;

/// Gabriel Cell state for visualization
#[derive(Debug, Clone)]
pub struct GabrielCellViz {
    /// Cell identifier
    pub id: usize,
    /// Psi (activation)
    pub psi: f64,
    /// Rho (coherence)
    pub rho: f64,
    /// Omega (rhythm)
    pub omega: f64,
    /// Output value
    pub output: f64,
    /// Neighbor cell indices
    pub neighbors: Vec<usize>,
    /// Position in visualization (computed)
    pub position: Option<(f32, f32)>,
}

impl GabrielCellViz {
    pub fn new(id: usize, psi: f64, rho: f64, omega: f64) -> Self {
        Self {
            id,
            psi,
            rho,
            omega,
            output: psi * rho * omega,
            neighbors: Vec::new(),
            position: None,
        }
    }

    /// Compute resonance D = ψ·ρ·ω
    pub fn resonance(&self) -> f64 {
        self.psi * self.rho * self.omega
    }
}

/// Gabriel Cell Network visualization data
#[derive(Debug, Clone)]
pub struct GabrielNetworkData {
    /// All cells in the network
    pub cells: Vec<GabrielCellViz>,
    /// Network identifier
    pub network_id: String,
    /// Total network resonance
    pub total_resonance: f64,
    /// Animation time
    pub time: f32,
    /// Animation enabled
    pub animated: bool,
    /// Show cell labels
    pub show_labels: bool,
    /// Show resonance values
    pub show_resonance: bool,
    /// Layout mode
    pub layout: NetworkLayout,
}

/// Network layout mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkLayout {
    /// Circular arrangement
    Circular,
    /// Grid arrangement
    Grid,
    /// Force-directed (springs)
    ForceDirected,
    /// Star topology
    Star,
}

impl Default for GabrielNetworkData {
    fn default() -> Self {
        Self {
            cells: Vec::new(),
            network_id: "default".to_string(),
            total_resonance: 0.0,
            time: 0.0,
            animated: true,
            show_labels: true,
            show_resonance: true,
            layout: NetworkLayout::Circular,
        }
    }
}

impl GabrielNetworkData {
    /// Create a new network with the given cells
    pub fn new(cells: Vec<GabrielCellViz>) -> Self {
        let total_resonance: f64 = cells.iter().map(|c| c.resonance()).sum();
        Self {
            cells,
            total_resonance,
            ..Default::default()
        }
    }

    /// Create a default 8-cell coupled network
    pub fn create_default_network() -> Self {
        let mut cells = Vec::new();

        // Create 8 cells with varied parameters
        for i in 0..8 {
            let phase = (i as f64 * PI as f64 / 4.0);
            let psi = 0.5 + 0.3 * phase.sin();
            let rho = 0.6 + 0.2 * phase.cos();
            let omega = 0.4 + 0.3 * (phase * 2.0).sin();

            let mut cell = GabrielCellViz::new(i, psi, rho, omega);

            // Couple adjacent cells (ring topology)
            cell.neighbors.push((i + 7) % 8);
            cell.neighbors.push((i + 1) % 8);

            cells.push(cell);
        }

        Self::new(cells)
    }

    /// Update cell positions based on layout
    pub fn compute_layout(&mut self, width: f32, height: f32) {
        let center_x = width / 2.0;
        let center_y = height / 2.0;
        let radius = (width.min(height) / 2.0) * 0.7;

        match self.layout {
            NetworkLayout::Circular => {
                let n = self.cells.len();
                for (i, cell) in self.cells.iter_mut().enumerate() {
                    let angle = (i as f32 / n as f32) * 2.0 * PI - PI / 2.0;
                    cell.position = Some((
                        center_x + radius * angle.cos(),
                        center_y + radius * angle.sin(),
                    ));
                }
            }
            NetworkLayout::Grid => {
                let n = self.cells.len();
                let cols = (n as f32).sqrt().ceil() as usize;
                let cell_width = width / (cols + 1) as f32;
                let cell_height = height / (cols + 1) as f32;

                for (i, cell) in self.cells.iter_mut().enumerate() {
                    let row = i / cols;
                    let col = i % cols;
                    cell.position = Some((
                        cell_width * (col + 1) as f32,
                        cell_height * (row + 1) as f32,
                    ));
                }
            }
            NetworkLayout::Star => {
                // First cell in center, others around
                if !self.cells.is_empty() {
                    self.cells[0].position = Some((center_x, center_y));

                    let n = self.cells.len() - 1;
                    for (i, cell) in self.cells.iter_mut().enumerate().skip(1) {
                        let angle = ((i - 1) as f32 / n as f32) * 2.0 * PI - PI / 2.0;
                        cell.position = Some((
                            center_x + radius * angle.cos(),
                            center_y + radius * angle.sin(),
                        ));
                    }
                }
            }
            NetworkLayout::ForceDirected => {
                // Simple initial placement (would need iterative refinement)
                let n = self.cells.len();
                for (i, cell) in self.cells.iter_mut().enumerate() {
                    let angle = (i as f32 / n as f32) * 2.0 * PI;
                    let r = radius * (0.5 + 0.5 * (i as f32 / n as f32));
                    cell.position = Some((
                        center_x + r * angle.cos(),
                        center_y + r * angle.sin(),
                    ));
                }
            }
        }
    }

    /// Update animation time
    pub fn tick(&mut self, dt: f32) {
        if self.animated {
            self.time += dt;
        }
    }

    /// Get network statistics
    pub fn statistics(&self) -> NetworkStatistics {
        let n = self.cells.len();
        let total_connections: usize = self.cells.iter().map(|c| c.neighbors.len()).sum();
        let avg_resonance = if n > 0 {
            self.cells.iter().map(|c| c.resonance()).sum::<f64>() / n as f64
        } else {
            0.0
        };

        let min_resonance = self.cells.iter()
            .map(|c| c.resonance())
            .fold(f64::INFINITY, f64::min);
        let max_resonance = self.cells.iter()
            .map(|c| c.resonance())
            .fold(f64::NEG_INFINITY, f64::max);

        NetworkStatistics {
            num_cells: n,
            num_connections: total_connections / 2, // Each connection counted twice
            avg_resonance,
            min_resonance: if min_resonance.is_infinite() { 0.0 } else { min_resonance },
            max_resonance: if max_resonance.is_infinite() { 0.0 } else { max_resonance },
            total_resonance: self.total_resonance,
        }
    }
}

/// Network statistics
#[derive(Debug, Clone)]
pub struct NetworkStatistics {
    pub num_cells: usize,
    pub num_connections: usize,
    pub avg_resonance: f64,
    pub min_resonance: f64,
    pub max_resonance: f64,
    pub total_resonance: f64,
}

/// Gabriel Network visualization state
#[derive(Debug, Default)]
pub struct GabrielNetworkState {
    cache: canvas::Cache,
}

impl GabrielNetworkState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn request_redraw(&mut self) {
        self.cache.clear();
    }
}

/// Create the Gabriel network visualization widget
pub fn gabriel_network_view<'a, Message: 'a + Clone>(
    data: &'a GabrielNetworkData,
    state: &'a GabrielNetworkState,
    width: f32,
    height: f32,
) -> Element<'a, Message> {
    Canvas::new(GabrielNetworkCanvas::<Message> {
        data,
        state,
        _marker: std::marker::PhantomData,
    })
    .width(Length::Fixed(width))
    .height(Length::Fixed(height))
    .into()
}

struct GabrielNetworkCanvas<'a, Message> {
    data: &'a GabrielNetworkData,
    state: &'a GabrielNetworkState,
    _marker: std::marker::PhantomData<Message>,
}

impl<'a, Message> canvas::Program<Message> for GabrielNetworkCanvas<'a, Message> {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry<Renderer>> {
        let geometry = self.state.cache.draw(renderer, bounds.size(), |frame| {
            self.draw_network(frame, bounds.size());
        });

        vec![geometry]
    }
}

impl<'a, Message> GabrielNetworkCanvas<'a, Message> {
    fn draw_network(&self, frame: &mut Frame, size: Size) {
        // Background
        let bg_path = Path::rectangle(Point::ORIGIN, size);
        frame.fill(&bg_path, Color::from_rgba(0.05, 0.08, 0.12, 1.0));

        if self.data.cells.is_empty() {
            return;
        }

        // Draw connections first (behind nodes)
        self.draw_connections(frame);

        // Draw cells
        self.draw_cells(frame);

        // Draw legend/info
        self.draw_info(frame, size);
    }

    fn draw_connections(&self, frame: &mut Frame) {
        // Default connection color (overridden per-connection based on resonance)
        let _connection_color = Color::from_rgba(0.3, 0.5, 0.7, 0.5);

        for cell in &self.data.cells {
            if let Some((x1, y1)) = cell.position {
                for &neighbor_idx in &cell.neighbors {
                    if neighbor_idx < self.data.cells.len() && neighbor_idx > cell.id {
                        // Only draw each connection once
                        if let Some((x2, y2)) = self.data.cells[neighbor_idx].position {
                            // Connection strength based on resonance similarity
                            let r1 = cell.resonance();
                            let r2 = self.data.cells[neighbor_idx].resonance();
                            let strength = 1.0 - (r1 - r2).abs().min(1.0);

                            // Animated pulse
                            let pulse = if self.data.animated {
                                let phase = (cell.id as f32 + self.data.time) * 0.5;
                                0.3 + 0.7 * phase.sin().abs()
                            } else {
                                1.0
                            };

                            let color = Color::from_rgba(
                                (0.2 + 0.3 * strength as f32) * pulse,
                                (0.4 + 0.4 * strength as f32) * pulse,
                                (0.6 + 0.3 * strength as f32) * pulse,
                                0.5 + 0.3 * strength as f32,
                            );

                            let path = Path::line(Point::new(x1, y1), Point::new(x2, y2));
                            frame.stroke(
                                &path,
                                Stroke::default()
                                    .with_color(color)
                                    .with_width(1.0 + 2.0 * strength as f32),
                            );

                            // Draw flow particles along connection (animated)
                            if self.data.animated {
                                let t = (self.data.time * 0.3 + cell.id as f32 * 0.1) % 1.0;
                                let px = x1 + (x2 - x1) * t;
                                let py = y1 + (y2 - y1) * t;
                                let particle = Path::circle(Point::new(px, py), 3.0);
                                frame.fill(&particle, Color::from_rgba(0.6, 0.8, 1.0, 0.8));
                            }
                        }
                    }
                }
            }
        }
    }

    fn draw_cells(&self, frame: &mut Frame) {
        for cell in &self.data.cells {
            if let Some((x, y)) = cell.position {
                let resonance = cell.resonance();

                // Base radius + resonance-based scaling
                let base_radius = 20.0;
                let radius = base_radius + (resonance as f32 * 10.0).min(15.0);

                // Animation pulse
                let pulse = if self.data.animated {
                    let phase = cell.id as f32 * 0.5 + self.data.time;
                    1.0 + 0.1 * phase.sin()
                } else {
                    1.0
                };

                let animated_radius = radius * pulse;

                // Cell color based on (ψ, ρ, ω)
                let cell_color = Color::from_rgba(
                    (cell.psi as f32 * 0.8).min(1.0),
                    (cell.rho as f32 * 0.9).min(1.0),
                    (cell.omega as f32 * 1.0).min(1.0),
                    0.9,
                );

                // Outer glow (resonance indicator)
                let glow_radius = animated_radius * 1.5;
                let glow_color = Color::from_rgba(
                    cell_color.r * 0.5,
                    cell_color.g * 0.5,
                    cell_color.b * 0.5,
                    0.2 + 0.3 * resonance as f32,
                );
                let glow = Path::circle(Point::new(x, y), glow_radius);
                frame.fill(&glow, glow_color);

                // Main cell body
                let cell_path = Path::circle(Point::new(x, y), animated_radius);
                frame.fill(&cell_path, cell_color);

                // Cell border
                let border_color = Color::from_rgba(0.8, 0.9, 1.0, 0.6);
                frame.stroke(
                    &cell_path,
                    Stroke::default().with_color(border_color).with_width(2.0),
                );

                // Inner indicator (shows output level)
                let inner_radius = animated_radius * 0.4 * (cell.output as f32).min(1.0);
                let inner = Path::circle(Point::new(x, y), inner_radius);
                frame.fill(&inner, Color::from_rgba(1.0, 1.0, 1.0, 0.5));

                // Draw component bars (mini bar chart for ψ, ρ, ω)
                self.draw_component_bars(frame, x, y + animated_radius + 8.0, cell);
            }
        }
    }

    fn draw_component_bars(&self, frame: &mut Frame, x: f32, y: f32, cell: &GabrielCellViz) {
        let bar_width = 4.0;
        let max_height = 15.0;
        let spacing = 2.0;
        let start_x = x - (bar_width * 3.0 + spacing * 2.0) / 2.0;

        // Psi bar (red-ish)
        let psi_height = (cell.psi as f32 * max_height).min(max_height);
        let psi_bar = Path::rectangle(
            Point::new(start_x, y - psi_height),
            Size::new(bar_width, psi_height),
        );
        frame.fill(&psi_bar, Color::from_rgba(0.9, 0.3, 0.3, 0.8));

        // Rho bar (green-ish)
        let rho_height = (cell.rho as f32 * max_height).min(max_height);
        let rho_bar = Path::rectangle(
            Point::new(start_x + bar_width + spacing, y - rho_height),
            Size::new(bar_width, rho_height),
        );
        frame.fill(&rho_bar, Color::from_rgba(0.3, 0.9, 0.3, 0.8));

        // Omega bar (blue-ish)
        let omega_height = (cell.omega as f32 * max_height).min(max_height);
        let omega_bar = Path::rectangle(
            Point::new(start_x + (bar_width + spacing) * 2.0, y - omega_height),
            Size::new(bar_width, omega_height),
        );
        frame.fill(&omega_bar, Color::from_rgba(0.3, 0.3, 0.9, 0.8));
    }

    fn draw_info(&self, frame: &mut Frame, size: Size) {
        let stats = self.data.statistics();

        // Network stats (text rendering requires font configuration)
        let _info_text = format!(
            "Cells: {}  |  Connections: {}  |  Avg D: {:.3}",
            stats.num_cells, stats.num_connections, stats.avg_resonance
        );

        // Legend (bottom right)
        let legend_x = size.width - 80.0;
        let legend_y = size.height - 50.0;

        // Psi legend
        let psi_rect = Path::rectangle(Point::new(legend_x, legend_y), Size::new(12.0, 12.0));
        frame.fill(&psi_rect, Color::from_rgba(0.9, 0.3, 0.3, 0.8));

        // Rho legend
        let rho_rect = Path::rectangle(Point::new(legend_x, legend_y + 15.0), Size::new(12.0, 12.0));
        frame.fill(&rho_rect, Color::from_rgba(0.3, 0.9, 0.3, 0.8));

        // Omega legend
        let omega_rect = Path::rectangle(Point::new(legend_x, legend_y + 30.0), Size::new(12.0, 12.0));
        frame.fill(&omega_rect, Color::from_rgba(0.3, 0.3, 0.9, 0.8));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_creation() {
        let cell = GabrielCellViz::new(0, 0.5, 0.6, 0.7);
        assert!((cell.resonance() - 0.21).abs() < 0.001);
    }

    #[test]
    fn test_default_network() {
        let network = GabrielNetworkData::create_default_network();
        assert_eq!(network.cells.len(), 8);

        // Each cell should have 2 neighbors (ring topology)
        for cell in &network.cells {
            assert_eq!(cell.neighbors.len(), 2);
        }
    }

    #[test]
    fn test_layout_computation() {
        let mut network = GabrielNetworkData::create_default_network();
        network.compute_layout(400.0, 400.0);

        // All cells should have positions
        for cell in &network.cells {
            assert!(cell.position.is_some());
        }
    }

    #[test]
    fn test_statistics() {
        let network = GabrielNetworkData::create_default_network();
        let stats = network.statistics();

        assert_eq!(stats.num_cells, 8);
        assert_eq!(stats.num_connections, 8); // Ring topology
    }
}

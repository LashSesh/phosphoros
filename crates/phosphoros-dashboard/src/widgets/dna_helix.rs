//! DNA Helix 3D Visualization Widget
//!
//! Renders a 3D double helix visualization based on the suite5d DNA strand data
//! using helical coordinates from the InfoGenetics module.
//!
//! Visual components:
//! - Two intertwined strand curves (phosphate-sugar backbone)
//! - Base pair connections between strands
//! - Color coding based on spectral signature (ψ, ρ, ω)
//! - Rotation animation for 3D effect

use iced::widget::canvas::{self, Canvas, Frame, Geometry, Path, Stroke, Fill};
use iced::{Element, Length, Point, Size, Color, Theme, Rectangle, Renderer};
use iced::mouse;
use std::f32::consts::PI;

/// DNA Helix visualization data
#[derive(Debug, Clone)]
pub struct DnaHelixData {
    /// Helix coordinates from InfoGenetics: Vec of strand coordinates
    /// Each strand is a Vec of (x, y, z) points
    pub helix_coords: Vec<Vec<(f64, f64, f64)>>,
    /// Spectral signature for coloring
    pub psi: f64,
    pub rho: f64,
    pub omega: f64,
    /// Rotation angle (animated)
    pub rotation: f32,
    /// Zoom level
    pub zoom: f32,
    /// Number of turns to display
    pub num_turns: usize,
    /// Show base pairs
    pub show_base_pairs: bool,
    /// Animation speed
    pub animation_speed: f32,
}

impl Default for DnaHelixData {
    fn default() -> Self {
        Self {
            helix_coords: Vec::new(),
            psi: 0.5,
            rho: 0.5,
            omega: 0.5,
            rotation: 0.0,
            zoom: 1.0,
            num_turns: 3,
            show_base_pairs: true,
            animation_speed: 0.02,
        }
    }
}

impl DnaHelixData {
    /// Create new helix data with coordinates
    pub fn new(helix_coords: Vec<Vec<(f64, f64, f64)>>) -> Self {
        Self {
            helix_coords,
            ..Default::default()
        }
    }

    /// Set spectral signature for coloring
    pub fn with_signature(mut self, psi: f64, rho: f64, omega: f64) -> Self {
        self.psi = psi;
        self.rho = rho;
        self.omega = omega;
        self
    }

    /// Update rotation for animation
    pub fn tick(&mut self) {
        self.rotation += self.animation_speed;
        if self.rotation > 2.0 * PI {
            self.rotation -= 2.0 * PI;
        }
    }

    /// Generate default helix coordinates if none provided
    pub fn generate_default_helix(&mut self, num_points: usize) {
        let num_turns = self.num_turns as f64;
        let points_per_turn = num_points / self.num_turns;

        // Generate two strands offset by PI
        let mut strand1 = Vec::new();
        let mut strand2 = Vec::new();

        for i in 0..num_points {
            let t = (i as f64 / points_per_turn as f64) * 2.0 * std::f64::consts::PI;
            let h = i as f64 / num_points as f64;

            // Strand 1
            let x1 = t.cos();
            let y1 = t.sin();
            let z1 = h;
            strand1.push((x1, y1, z1));

            // Strand 2 (offset by PI)
            let x2 = (t + std::f64::consts::PI).cos();
            let y2 = (t + std::f64::consts::PI).sin();
            let z2 = h;
            strand2.push((x2, y2, z2));
        }

        self.helix_coords = vec![strand1, strand2];
    }
}

/// DNA Helix visualization state
#[derive(Debug, Default)]
pub struct DnaHelixState {
    cache: canvas::Cache,
}

impl DnaHelixState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn request_redraw(&mut self) {
        self.cache.clear();
    }
}

/// Create the DNA helix visualization widget
pub fn dna_helix_view<'a, Message: 'a + Clone>(
    data: &'a DnaHelixData,
    state: &'a DnaHelixState,
    width: f32,
    height: f32,
) -> Element<'a, Message> {
    Canvas::new(DnaHelixCanvas::<Message> {
        data,
        state,
        _marker: std::marker::PhantomData,
    })
    .width(Length::Fixed(width))
    .height(Length::Fixed(height))
    .into()
}

struct DnaHelixCanvas<'a, Message> {
    data: &'a DnaHelixData,
    state: &'a DnaHelixState,
    _marker: std::marker::PhantomData<Message>,
}

impl<'a, Message> canvas::Program<Message> for DnaHelixCanvas<'a, Message> {
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
            self.draw_helix(frame, bounds.size());
        });

        vec![geometry]
    }
}

impl<'a, Message> DnaHelixCanvas<'a, Message> {
    fn draw_helix(&self, frame: &mut Frame, size: Size) {
        let center_x = size.width / 2.0;
        let center_y = size.height / 2.0;
        let scale = (size.width.min(size.height) / 3.0) * self.data.zoom;

        // Background
        let bg_path = Path::rectangle(Point::ORIGIN, size);
        frame.fill(&bg_path, Color::from_rgba(0.05, 0.08, 0.12, 1.0));

        // Colors based on spectral signature
        let strand1_color = Color::from_rgba(
            (0.2 + 0.6 * self.data.psi as f32).min(1.0),
            (0.3 + 0.4 * self.data.rho as f32).min(1.0),
            (0.6 + 0.3 * self.data.omega as f32).min(1.0),
            0.9,
        );

        let strand2_color = Color::from_rgba(
            (0.6 + 0.3 * self.data.omega as f32).min(1.0),
            (0.2 + 0.5 * self.data.psi as f32).min(1.0),
            (0.4 + 0.4 * self.data.rho as f32).min(1.0),
            0.9,
        );

        let base_pair_color = Color::from_rgba(0.4, 0.5, 0.6, 0.5);

        // If we have helix coordinates, use them
        if self.data.helix_coords.len() >= 2 {
            let strand1 = &self.data.helix_coords[0];
            let strand2 = &self.data.helix_coords[1];

            // Draw strands
            self.draw_strand(frame, strand1, center_x, center_y, scale, strand1_color);
            self.draw_strand(frame, strand2, center_x, center_y, scale, strand2_color);

            // Draw base pairs
            if self.data.show_base_pairs {
                self.draw_base_pairs(frame, strand1, strand2, center_x, center_y, scale, base_pair_color);
            }
        } else {
            // Generate default helix visualization
            self.draw_generated_helix(frame, center_x, center_y, scale, strand1_color, strand2_color, base_pair_color);
        }

        // Draw title
        // frame.fill_text(canvas::Text {
        //     content: "DNA Helix".to_string(),
        //     position: Point::new(10.0, 20.0),
        //     size: 14.0.into(),
        //     color: Color::from_rgba(0.8, 0.8, 0.8, 1.0),
        //     ..Default::default()
        // });

        // Draw resonance info
        let _resonance = self.data.psi * self.data.rho * self.data.omega;
        let _info_text = format!(
            "D={:.3} (psi={:.2}, rho={:.2}, omega={:.2})",
            _resonance, self.data.psi, self.data.rho, self.data.omega
        );
        // Note: Text rendering commented out - requires font configuration
        // frame.fill_text(canvas::Text {
        //     content: _info_text,
        //     position: Point::new(10.0, size.height - 20.0),
        //     size: 12.0.into(),
        //     color: Color::from_rgba(0.6, 0.7, 0.8, 1.0),
        //     ..Default::default()
        // });
    }

    fn draw_strand(
        &self,
        frame: &mut Frame,
        coords: &[(f64, f64, f64)],
        cx: f32,
        cy: f32,
        scale: f32,
        color: Color,
    ) {
        if coords.len() < 2 {
            return;
        }

        let cos_rot = self.data.rotation.cos();
        let sin_rot = self.data.rotation.sin();

        // Draw strand as connected circles (3D effect)
        for (_i, &(x, y, z)) in coords.iter().enumerate() {
            // Apply rotation around Y axis
            let x_rot = x as f32 * cos_rot - z as f32 * sin_rot;
            let z_rot = x as f32 * sin_rot + z as f32 * cos_rot;

            // Project to 2D (simple orthographic with depth cue)
            let depth_factor = 0.5 + 0.5 * (z_rot + 1.0) / 2.0;
            let px = cx + x_rot * scale * depth_factor;
            let py = cy + (y as f32) * scale * depth_factor - z_rot * scale * 0.3;

            // Size based on depth
            let radius = 3.0 + 2.0 * depth_factor;

            // Color with depth shading
            let shaded_color = Color::from_rgba(
                color.r * depth_factor,
                color.g * depth_factor,
                color.b * depth_factor,
                color.a,
            );

            let circle = Path::circle(Point::new(px, py), radius);
            frame.fill(&circle, shaded_color);
        }

        // Draw connecting lines
        let path = Path::new(|builder| {
            let mut first = true;
            for &(x, y, z) in coords.iter() {
                let x_rot = x as f32 * cos_rot - z as f32 * sin_rot;
                let z_rot = x as f32 * sin_rot + z as f32 * cos_rot;
                let depth_factor = 0.5 + 0.5 * (z_rot + 1.0) / 2.0;
                let px = cx + x_rot * scale * depth_factor;
                let py = cy + (y as f32) * scale * depth_factor - z_rot * scale * 0.3;

                if first {
                    builder.move_to(Point::new(px, py));
                    first = false;
                } else {
                    builder.line_to(Point::new(px, py));
                }
            }
        });

        frame.stroke(&path, Stroke::default().with_color(color).with_width(1.5));
    }

    fn draw_base_pairs(
        &self,
        frame: &mut Frame,
        strand1: &[(f64, f64, f64)],
        strand2: &[(f64, f64, f64)],
        cx: f32,
        cy: f32,
        scale: f32,
        color: Color,
    ) {
        let cos_rot = self.data.rotation.cos();
        let sin_rot = self.data.rotation.sin();

        // Draw every nth base pair
        let step = (strand1.len() / 20).max(1);

        for i in (0..strand1.len().min(strand2.len())).step_by(step) {
            let (x1, y1, z1) = strand1[i];
            let (x2, y2, z2) = strand2[i];

            // Apply rotation
            let x1_rot = x1 as f32 * cos_rot - z1 as f32 * sin_rot;
            let z1_rot = x1 as f32 * sin_rot + z1 as f32 * cos_rot;
            let x2_rot = x2 as f32 * cos_rot - z2 as f32 * sin_rot;
            let z2_rot = x2 as f32 * sin_rot + z2 as f32 * cos_rot;

            let depth1 = 0.5 + 0.5 * (z1_rot + 1.0) / 2.0;
            let depth2 = 0.5 + 0.5 * (z2_rot + 1.0) / 2.0;

            let px1 = cx + x1_rot * scale * depth1;
            let py1 = cy + (y1 as f32) * scale * depth1 - z1_rot * scale * 0.3;
            let px2 = cx + x2_rot * scale * depth2;
            let py2 = cy + (y2 as f32) * scale * depth2 - z2_rot * scale * 0.3;

            let path = Path::line(Point::new(px1, py1), Point::new(px2, py2));
            frame.stroke(&path, Stroke::default().with_color(color).with_width(1.0));
        }
    }

    fn draw_generated_helix(
        &self,
        frame: &mut Frame,
        cx: f32,
        cy: f32,
        scale: f32,
        color1: Color,
        color2: Color,
        bp_color: Color,
    ) {
        let num_points = 100;
        let num_turns = self.data.num_turns as f32;
        let cos_rot = self.data.rotation.cos();
        let sin_rot = self.data.rotation.sin();

        // Generate and draw strands
        for i in 0..num_points {
            let t = (i as f32 / num_points as f32) * num_turns * 2.0 * PI;
            let h = (i as f32 / num_points as f32) * 2.0 - 1.0;

            // Strand 1
            let x1 = t.cos();
            let y1 = t.sin();
            let z1 = h;

            // Strand 2 (offset by PI)
            let x2 = (t + PI).cos();
            let y2 = (t + PI).sin();
            let z2 = h;

            // Apply rotation
            let x1_rot = x1 * cos_rot - z1 * sin_rot;
            let z1_rot = x1 * sin_rot + z1 * cos_rot;
            let x2_rot = x2 * cos_rot - z2 * sin_rot;
            let z2_rot = x2 * sin_rot + z2 * cos_rot;

            let depth1 = 0.5 + 0.5 * (z1_rot + 1.0) / 2.0;
            let depth2 = 0.5 + 0.5 * (z2_rot + 1.0) / 2.0;

            let px1 = cx + x1_rot * scale * depth1;
            let py1 = cy + y1 * scale * depth1 - z1_rot * scale * 0.5;
            let px2 = cx + x2_rot * scale * depth2;
            let py2 = cy + y2 * scale * depth2 - z2_rot * scale * 0.5;

            // Draw strand points
            let radius1 = 2.0 + 2.0 * depth1;
            let radius2 = 2.0 + 2.0 * depth2;

            let c1 = Color::from_rgba(color1.r * depth1, color1.g * depth1, color1.b * depth1, color1.a);
            let c2 = Color::from_rgba(color2.r * depth2, color2.g * depth2, color2.b * depth2, color2.a);

            frame.fill(&Path::circle(Point::new(px1, py1), radius1), c1);
            frame.fill(&Path::circle(Point::new(px2, py2), radius2), c2);

            // Draw base pair every 10 points
            if self.data.show_base_pairs && i % 10 == 0 {
                let avg_depth = (depth1 + depth2) / 2.0;
                let bp_shaded = Color::from_rgba(
                    bp_color.r * avg_depth,
                    bp_color.g * avg_depth,
                    bp_color.b * avg_depth,
                    bp_color.a,
                );
                let path = Path::line(Point::new(px1, py1), Point::new(px2, py2));
                frame.stroke(&path, Stroke::default().with_color(bp_shaded).with_width(1.0));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_helix_data_default() {
        let data = DnaHelixData::default();
        assert!(data.helix_coords.is_empty());
        assert_eq!(data.rotation, 0.0);
    }

    #[test]
    fn test_helix_data_tick() {
        let mut data = DnaHelixData::default();
        data.tick();
        assert!(data.rotation > 0.0);
    }

    #[test]
    fn test_generate_default_helix() {
        let mut data = DnaHelixData::default();
        data.generate_default_helix(100);
        assert_eq!(data.helix_coords.len(), 2);
        assert_eq!(data.helix_coords[0].len(), 100);
    }
}

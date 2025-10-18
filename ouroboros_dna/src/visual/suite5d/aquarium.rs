use rand::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct MycelCell2D {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub color: [f32; 3],
    pub opacity: f32,
}

#[derive(Debug, Clone)]
pub struct Aquarium2D {
    cells: Vec<MycelCell2D>,
    width: f32,
    height: f32,
}

impl Aquarium2D {
    pub fn new(count: usize, width: f32, height: f32) -> Self {
        let mut rng = thread_rng();
        let cells = (0..count)
            .map(|_| MycelCell2D {
                x: rng.gen_range(0.0..width.max(1.0)),
                y: rng.gen_range(0.0..height.max(1.0)),
                radius: rng.gen_range(8.0..24.0),
                color: [
                    rng.gen_range(0.4..1.0),
                    rng.gen_range(0.2..0.6),
                    rng.gen_range(0.6..1.0),
                ],
                opacity: rng.gen_range(0.6..0.95),
            })
            .collect();

        Self {
            cells,
            width,
            height,
        }
    }

    pub fn cells(&self) -> &[MycelCell2D] {
        &self.cells
    }

    pub fn bounds(&self) -> (f32, f32) {
        (self.width, self.height)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MycelCell3D {
    pub position: [f32; 3],
    pub color: [f32; 4],
}

#[derive(Debug, Clone)]
pub struct Aquarium3D {
    cells: Vec<MycelCell3D>,
}

impl Aquarium3D {
    pub fn new(count: usize, radius: f32) -> Self {
        let mut rng = thread_rng();
        let cells = (0..count)
            .map(|_| {
                let angle_a = rng.gen_range(0.0..std::f32::consts::TAU);
                let angle_b = rng.gen_range(0.0..std::f32::consts::PI);
                let r = rng.gen_range(0.3..1.0) * radius.max(1.0);
                let (sin_b, cos_b) = angle_b.sin_cos();
                let (sin_a, cos_a) = angle_a.sin_cos();
                MycelCell3D {
                    position: [r * sin_b * cos_a, r * sin_b * sin_a, r * cos_b],
                    color: [
                        rng.gen_range(0.2..1.0),
                        rng.gen_range(0.2..1.0),
                        rng.gen_range(0.2..1.0),
                        rng.gen_range(0.7..1.0),
                    ],
                }
            })
            .collect();

        Self { cells }
    }

    pub fn cells(&self) -> &[MycelCell3D] {
        &self.cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aquarium_generates_cells() {
        let aq = Aquarium2D::new(5, 800.0, 600.0);
        assert_eq!(aq.cells().len(), 5);
        let (w, h) = aq.bounds();
        assert_eq!((w, h), (800.0, 600.0));
    }

    #[test]
    fn aquarium3d_generates_positions() {
        let aq = Aquarium3D::new(10, 5.0);
        assert_eq!(aq.cells().len(), 10);
        assert!(aq
            .cells()
            .iter()
            .all(|cell| cell.position.iter().all(|coord| coord.is_finite())));
    }
}

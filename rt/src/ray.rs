use crate::{Float, Transform, Vector};

#[derive(Debug, Copy, Clone, Default)]
pub struct Ray {
    pub origin: Vector,
    pub dir: Vector,
}

impl Ray {
    pub fn new(origin: Vector, dir: Vector) -> Self {
        Self { origin, dir }
    }

    pub fn transform(self, transform: &Transform) -> Self {
        Self::new(
            transform.transform_point3a(self.origin),
            transform.transform_vector3a(self.dir),
        )
    }

    pub fn advance_by(self, t: Float) -> Self {
        Self::new(self.origin + self.dir * t, self.dir)
    }
}

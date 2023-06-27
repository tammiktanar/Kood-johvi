use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};

use glam::Mat3A;

use crate::{Float, Vector};

#[derive(Debug, Clone)]
pub struct LocalVectorFrame {
    transform: Mat3A,
    inverse: Mat3A,
}

impl Display for LocalVectorFrame {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.transform.fmt(f)
    }
}

impl LocalVectorFrame {
    pub fn new(normal: Vector) -> LocalVectorFrame {

        // let tan = normal.cross(Vector::Y).normalize();
        // let tan = normal.any_orthogonal_vector().normalize();
        // let bitan = normal.cross(tan).normalize();

        let (tan, bitan) = normal.any_orthonormal_pair();

        let transform = Mat3A::from_cols(tan, bitan, normal);
        Self {
            transform,
            inverse: transform.inverse(),
        }
    }

    pub fn to_mat(&self, vec: Vector) -> LocalVector {
        LocalVector::from(self.inverse * vec)
    }

    pub fn to_world(&self, vec: LocalVector) -> Vector {
        self.transform * *vec
    }
}

/// A normalized [Vector] transformed such, that +Z is always aligned with the surface normal.
#[derive(Debug, Clone, Copy)]
pub struct LocalVector(Vector);

impl LocalVector {
    pub fn new(x: Float, y: Float, z: Float) -> Self {
        Vector::new(x, y, z).into()
    }

    pub fn dot_normal(&self) -> Float {
        self.z
    }

    pub fn to_vector(self) -> Vector {
        self.0
    }
}

impl From<Vector> for LocalVector {
    fn from(vec: Vector) -> Self {
        Self(vec)
    }
}

impl Deref for LocalVector {
    type Target = Vector;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LocalVector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
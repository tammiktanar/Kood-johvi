use bvh::aabb::{AABB, Bounded};
use glam::Vec3;
use serde::{Deserialize, Serialize};
use smallvec::smallvec;

use crate::{Float, Vector};
use crate::object_like::hit::{BasicHit, HitGroup, Transition};
use crate::object_like::Object;
use crate::object_like::shape::ShapeInterface;
use crate::ray::Ray;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Default)]
pub struct Plane {}

impl Plane {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ShapeInterface for Plane {
    fn try_hit<'a>(&self, ray: Ray, _min_t: Float, object: &'a Object) -> Option<HitGroup<'a>> {
        if ray.dir.y == 0.0 {
            return None;
        }

        let t = ray.origin.y / -ray.dir.y;

        let far_end = if ray.dir.y < 0.0 {
            Float::INFINITY
        } else {
            Float::NEG_INFINITY
        };

        Some(smallvec!(
            BasicHit::new(t, t, ray, Transition::Enter, object),
            BasicHit::new(far_end, far_end, ray, Transition::Exit, object),
        ))
    }

    fn get_normal(&self, _og_t: Float, _og_ray: Ray) -> Vector {
        Vector::Y
    }
}

impl Bounded for Plane {
    fn aabb(&self) -> AABB {
        AABB::with_bounds(
            Vec3::splat(f32::NEG_INFINITY),
            Vec3::splat(f32::INFINITY),
        )
    }
}
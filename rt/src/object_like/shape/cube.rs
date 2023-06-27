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
pub struct Cube {}


impl ShapeInterface for Cube {
    fn try_hit<'a>(&self, ray: Ray, _min_t: Float, object: &'a Object) -> Option<HitGroup<'a>> {
        let m = 1.0 / ray.dir;

        // Clamp to prevent infinities
        let m = m.clamp(Vector::splat(-Float::MAX.sqrt()), Vector::splat(Float::MAX.sqrt()));

        let n = m * ray.origin;
        let k = m.abs();
        let t1 = -n - k;
        let t2 = -n + k;
        let t_n = t1.max_element();
        let t_f = t2.min_element();
        if t_n > t_f /* || t_f < min_t*/ {
            return None;
        }

        Some(smallvec!(
            BasicHit::new(t_n, t_n, Ray::new(t1, ray.dir), Transition::Enter, object),
            BasicHit::new(t_f, -t_f, Ray::new(-t2, -ray.dir), Transition::Exit, object),
        ))
    }

    fn get_normal(&self, og_t: Float, og_ray: Ray) -> Vector {
        let normal = step(Vector::splat(og_t), og_ray.origin);
        normal * -og_ray.dir.signum()
    }
}

fn step(edge: Vector, v: Vector) -> Vector {
    Vector::new(
        if v.x < edge.x { 0.0 } else { 1.0 },
        if v.y < edge.y { 0.0 } else { 1.0 },
        if v.z < edge.z { 0.0 } else { 1.0 },
    )
}

impl Bounded for Cube {
    fn aabb(&self) -> AABB {
        AABB::with_bounds(
            Vec3::new(-1.0, -1.0, -1.0),
            Vec3::new(1.0, 1.0, 1.0),
        )
    }
}
use bvh::aabb::{AABB, Bounded};
use glam::Vec3;
use rand::prelude::SmallRng;
use serde::{Deserialize, Serialize};
use smallvec::smallvec;

use crate::{Float, PI, Vector};
use crate::helpers::{random_hemisphere_unit_vector, random_unit_vector};
use crate::object_like::hit::{BasicHit, HitGroup, Transition};
use crate::object_like::Object;
use crate::object_like::shape::{ShapeInterface, ShapeSampler};
use crate::ray::Ray;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Default)]
pub struct Sphere {}

impl ShapeInterface for Sphere {
    fn try_hit<'a>(&self, ray: Ray, _min_t: Float, object: &'a Object) -> Option<HitGroup<'a>> {
        let b = ray.origin.dot(ray.dir);
        let qc = ray.origin - b * ray.dir;
        let h = 1.0 - qc.length_squared();
        if h < 0.0 {
            return None;
        }

        let h = h.sqrt();

        let t1 = -b - h;
        let t2 = -b + h;

        Some(smallvec!(
            BasicHit::new(t1, t1, ray, Transition::Enter, object),
            BasicHit::new(t2, t2, ray, Transition::Exit, object),
        ))
    }

    fn get_normal(&self, og_t: Float, og_ray: Ray) -> Vector {
        og_ray.advance_by(og_t).origin
    }
}

impl ShapeSampler for Sphere {
    fn sample_point(&self, rng: &mut SmallRng, origin: Vector) -> (Vector, Vector, Float) {
        if origin.length_squared() > 1.0 {
            // Outside
            let hit = random_hemisphere_unit_vector(rng, origin);
            let normal = hit;

            (hit, normal, self.sample_pdf_uniform_hemisphere(origin, hit, normal))
        } else {
            // Inside
            let hit = random_unit_vector(rng);
            let normal = hit;

            (hit, normal, self.sample_pdf_whole_sphere(origin, hit, normal))
        }
    }

    fn sample_pdf(&self, origin: Vector, hit: Vector, normal: Vector) -> Float {
        if origin.length_squared() > 1.0 {
            // Outside
            self.sample_pdf_uniform_hemisphere(origin, hit, normal)
        } else {
            // Inside
            self.sample_pdf_whole_sphere(origin, hit, normal)
        }
    }

    fn area(&self) -> Float {
        4.0 * PI
    }

    fn supports_nee(&self) -> bool {
        // false
        true
    }
}

impl Sphere {
    fn sample_pdf_whole_sphere(&self, origin: Vector, hit: Vector, normal: Vector) -> Float {
        let wi = (origin - hit).normalize();
        1.0 / (4.0 * PI * wi.dot(normal).abs())
    }

    fn sample_pdf_uniform_hemisphere(&self, origin: Vector, hit: Vector, normal: Vector) -> Float {
        let wi = (origin - hit).normalize();
        1.0 / (2.0 * PI * wi.dot(normal).abs())
    }
}

impl Bounded for Sphere {
    fn aabb(&self) -> AABB {
        AABB::with_bounds(
            Vec3::new(-1.0, -1.0, -1.0),
            Vec3::new(1.0, 1.0, 1.0),
        )
    }
}
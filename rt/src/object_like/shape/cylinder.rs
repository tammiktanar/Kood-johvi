use bvh::aabb::{AABB, Bounded};
use glam::{Vec3, Vec3Swizzles};
use serde::{Deserialize, Serialize};
use smallvec::smallvec;

use crate::{Float, Vector};
use crate::object_like::hit::{BasicHit, HitGroup, Transition};
use crate::object_like::Object;
use crate::object_like::shape::ShapeInterface;
use crate::ray::Ray;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Default)]
pub struct Cylinder {}

impl ShapeInterface for Cylinder {
    fn try_hit<'a>(&self, ray: Ray, _min_t: Float, object: &'a Object) -> Option<HitGroup<'a>> {
        let radius: Float = 1.0;
        let height: Float = 1.0;

        let k2 = 1.0 - ray.dir.y.powi(2);
        let k1 = ray.origin.dot(ray.dir) - ray.origin.y * ray.dir.y;
        let k0 = ray.origin.dot(ray.origin) - ray.origin.y.powi(2) - radius.powi(2);


        // For parallel lines
        if k2 == 0.0 {
            // The following only works for a unit cylinder
            if ray.origin.xz().length_squared() > radius.powi(2) {
                return None;
            }

            return if ray.dir.y > 0.0 {
                let t1 = (-ray.origin.y) - height;
                let t2 = (-ray.origin.y) + height;
                Some(smallvec![
                    BasicHit::new(t1, t1, Ray::new(Vector::ZERO, Vector::NEG_Y), Transition::Enter, object),
                    BasicHit::new(t2, t2, Ray::new(Vector::ZERO, Vector::Y), Transition::Exit, object),
                ])
            } else {
                let t1 = ray.origin.y - height;
                let t2 = ray.origin.y + height;
                Some(smallvec![
                    BasicHit::new(t1, t1, Ray::new(Vector::ZERO, Vector::Y), Transition::Enter, object),
                    BasicHit::new(t2, t2, Ray::new(Vector::ZERO, Vector::NEG_Y), Transition::Exit, object),
                ])
            };
        }


        let h = k1 * k1 - k2 * k0;
        if h < 0.0 {
            return None;
        }
        let h = h.sqrt();
        let _t2 = (-k1 + h) / k2;

        let hit1 = 'hit: {
            let t = (-k1 - h) / k2;
            // body
            let y = ray.origin.y + t * ray.dir.y;
            if y > -height && y < height {
                let normal = (ray.origin + ray.dir * t - Vector::new(0.0, y, 0.0)) / radius;
                break 'hit BasicHit::new(t, t, Ray::new(Vector::ZERO, normal), Transition::Enter, object);
            }

            // caps
            let t = (height * y.signum() - ray.origin.y) / ray.dir.y;
            if (k1 + k2 * t).abs() < h
            {
                let normal = Vector::new(0.0, y.signum(), 0.0);
                break 'hit BasicHit::new(t, t, Ray::new(Vector::ZERO, normal), Transition::Enter, object);
            }
            return None;
        };

        let hit2 = 'hit: {
            let t = (-k1 + h) / k2;
            // body
            let y = ray.origin.y + t * ray.dir.y;
            if y > -height && y < height {
                let normal = (ray.origin + ray.dir * t - Vector::new(0.0, y, 0.0)) / radius;
                break 'hit BasicHit::new(t, t, Ray::new(Vector::ZERO, normal), Transition::Exit, object);
            }

            // caps
            let t = (height * y.signum() - ray.origin.y) / ray.dir.y;
            if (k1 + k2 * t).abs() < h
            {
                let normal = Vector::new(0.0, y.signum(), 0.0);
                break 'hit BasicHit::new(t, t, Ray::new(Vector::ZERO, normal), Transition::Exit, object);
            }
            return None;
        };

        Some(smallvec![hit1, hit2])
    }

    fn get_normal(&self, _og_t: Float, og_ray: Ray) -> Vector {
        og_ray.dir
    }
}

impl Bounded for Cylinder {
    fn aabb(&self) -> AABB {
        AABB::with_bounds(
            Vec3::new(-1.0, -1.0, -1.0),
            Vec3::new(1.0, 1.0, 1.0),
        )
    }
}
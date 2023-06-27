use std::cmp::Ordering;

use smallvec::SmallVec;

use crate::{Float, Vector};
use crate::object_like::Object;
use crate::object_like::shape::ShapeInterface;
use crate::ray::Ray;

#[derive(Debug)]
pub struct Hit<'a> {
    pub t: Float,
    pub point: Vector,
    pub normal: Vector,
    pub transition: Transition,
    pub outer_ior: Float,
    pub object: &'a Object,

    // pub mat_frame: MatVectorFrame,
}


pub type HitGroup<'a> = SmallVec<[BasicHit<'a>; 4]>;

#[derive(Debug)]
pub struct BasicHit<'a> {
    pub t: Float,
    pub transition: Transition,
    pub flip_normal: bool,
    pub og_t: Float,
    pub og_ray: Ray,
    pub object: &'a Object,
}

impl<'a> BasicHit<'a> {
    pub fn new(t: Float, og_t: Float, og_ray: Ray, transition: Transition, object: &'a Object) -> Self {
        Self {
            t,
            transition,
            flip_normal: false,
            og_t,
            og_ray,
            object,
        }
    }

    pub fn adjust_t(&mut self, t_approx: Float, scaling_factor: Float) {
        // self.t = t_approx + self.t / scaling_factor;
        self.t = t_approx + self.t / scaling_factor;
    }

    pub fn finalize(self, ray: Ray, outer_ior: Float) -> Hit<'a> {
        let mut normal = self.object.normal_to_world(self.object.shape.get_normal(self.og_t, self.og_ray));
        if self.flip_normal {
            normal = -normal;
        }

        Hit {
            t: self.t,
            point: ray.origin + ray.dir * self.t,
            normal,
            transition: self.transition,
            outer_ior,
            object: self.object,
            // mat_frame: MatVectorFrame::new(normal),
        }
    }
}


#[derive(Debug)]
pub enum Transition {
    Enter,
    Exit,
    EnterExit,
}

impl<'a> Eq for BasicHit<'a> {}

impl<'a> PartialEq<Self> for BasicHit<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t
    }
}

impl<'a> PartialOrd<Self> for BasicHit<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.t.partial_cmp(&other.t)
    }
}

impl<'a> Ord for BasicHit<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.t.total_cmp(&other.t)
    }
}
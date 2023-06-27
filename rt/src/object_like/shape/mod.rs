use std::fmt::Debug;

use anyhow::anyhow;
use bvh::aabb::{AABB, Bounded};
use either::Either;
use glam::Vec3;
use rand::rngs::SmallRng;
use serde::{Deserialize, Serialize};

pub use cube::Cube;
pub use cylinder::Cylinder;
pub use plane::Plane;
pub use sphere::Sphere;

use crate::{Float, Transform, Vector};
use crate::object_like::hit::HitGroup;
use crate::object_like::Object;
use crate::ray::Ray;

pub mod sphere;
mod plane;
mod cube;
mod cylinder;

pub trait ShapeInterface {
    fn try_hit<'a>(&self, ray: Ray, min_t: Float, object: &'a Object) -> Option<HitGroup<'a>>;

    fn get_normal(&self, og_t: Float, og_ray: Ray) -> Vector;
}

pub trait ShapeSampler {
    /// (point, normal)
    fn sample_point(&self, rng: &mut SmallRng, origin: Vector) -> (Vector, Vector, Float);

    fn sample_pdf(&self, origin: Vector, hit: Vector, normal: Vector) -> Float;

    fn area(&self) -> Float;

    fn supports_nee(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ShapeDeserialize {
    #[serde(with = "either::serde_untagged")]
    shape: Either<String, Shape>,
}

impl ShapeDeserialize {
    pub fn get_shape(&self) -> anyhow::Result<Shape> {
        self.shape.as_ref()
            .map_right(|shape| Ok(shape.clone()))
            .right_or_else(|name| {
                match name.as_str() {
                    "sphere" => Ok(Shape::Sphere(Sphere {})),
                    "ball" => Ok(Shape::Sphere(Sphere {})),
                    "cube" => Ok(Shape::Cube(Cube {})),
                    "box" => Ok(Shape::Cube(Cube {})),
                    "plane" => Ok(Shape::Plane(Plane {})),
                    "cylinder" => Ok(Shape::Cylinder(Cylinder {})),
                    _ => Err(anyhow!("\"{name}\" is not a valid shape")),
                }
            })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Shape {
    #[serde(alias = "ball")]
    Sphere(Sphere),
    Plane(Plane),
    #[serde(alias = "box")]
    Cube(Cube),
    Cylinder(Cylinder),
}

impl ShapeSampler for Shape {
    fn sample_point(&self, rng: &mut SmallRng, origin: Vector) -> (Vector, Vector, Float) {
        match self {
            Shape::Sphere(v) => v.sample_point(rng, origin),
            _ => unimplemented!(),
        }
    }

    fn sample_pdf(&self, origin: Vector, hit: Vector, normal: Vector) -> Float {
        match self {
            Shape::Sphere(v) => v.sample_pdf(origin, hit, normal),
            _ => unimplemented!(),
        }
    }

    fn area(&self) -> Float {
        match self {
            Shape::Sphere(v) => v.area(),
            _ => unimplemented!(),
        }
    }

    fn supports_nee(&self) -> bool {
        match self {
            Shape::Sphere(v) => v.supports_nee(),
            _ => false,
        }
    }
}


impl ShapeInterface for Shape {
    fn try_hit<'a>(&self, ray: Ray, min_t: Float, object: &'a Object) -> Option<HitGroup<'a>> {
        match self {
            Shape::Sphere(v) => v.try_hit(ray, min_t, object),
            Shape::Plane(v) => v.try_hit(ray, min_t, object),
            Shape::Cube(v) => v.try_hit(ray, min_t, object),
            Shape::Cylinder(v) => v.try_hit(ray, min_t, object),
        }
    }

    fn get_normal(&self, og_t: Float, og_ray: Ray) -> Vector {
        match self {
            Shape::Sphere(v) => v.get_normal(og_t, og_ray),
            Shape::Plane(v) => v.get_normal(og_t, og_ray),
            Shape::Cube(v) => v.get_normal(og_t, og_ray),
            Shape::Cylinder(v) => v.get_normal(og_t, og_ray),
        }
    }
}

impl Bounded for Shape {
    fn aabb(&self) -> AABB {
        match self {
            Shape::Sphere(v) => v.aabb(),
            Shape::Plane(v) => v.aabb(),
            Shape::Cube(v) => v.aabb(),
            Shape::Cylinder(v) => v.aabb(),
        }
    }
}

fn _params_to_ray(params: [Float; 6]) -> Ray {
    let origin = Vector::from_slice(&params[0..3]);
    let dir = Vector::from_slice(&params[3..6]);
    Ray::new(origin, dir)
}

fn _ray_to_params(ray: Ray) -> [Float; 6] {
    let mut params: [Float; 6] = [0.0; 6];
    let (one, two) = params.split_at_mut(3);
    one.copy_from_slice(&ray.origin.to_array());
    two.copy_from_slice(&ray.dir.to_array());
    params
}

#[derive(Debug, Clone, Copy)]
pub struct OOBB {
    origin: Vec3,
    x: Vec3,
    y: Vec3,
    z: Vec3,
}

impl OOBB {
    pub fn transform(mut self, transformation: Transform) -> Self {
        self.origin = transformation.transform_point3(self.origin);
        self.x = transformation.transform_vector3(self.x);
        self.y = transformation.transform_vector3(self.y);
        self.z = transformation.transform_vector3(self.z);
        self
    }

    pub fn from_aabb(aabb: AABB) -> Self {
        let size = aabb.size();
        let origin = aabb.min;
        Self {
            origin,
            x: Vec3::new(size.x, 0.0, 0.0),
            y: Vec3::new(0.0, size.y, 0.0),
            z: Vec3::new(0.0, 0.0, size.z),
        }
    }

    pub fn to_aabb(self) -> AABB {
        let p1 = self.origin;
        let p2 = self.origin + self.x;
        let p3 = self.origin + self.y;
        let p4 = self.origin + self.z;
        let p5 = self.origin + self.x + self.y;
        let p6 = self.origin + self.x + self.z;
        let p7 = self.origin + self.y + self.z;
        let p8 = self.origin + self.x + self.y + self.z;

        let min = p1.min(p2).min(p3).min(p4).min(p5).min(p6).min(p7).min(p8);
        let max = p1.max(p2).max(p3).max(p4).max(p5).max(p6).max(p7).max(p8);

        AABB::with_bounds(min, max)
    }
}
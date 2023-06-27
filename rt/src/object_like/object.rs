use std::sync::Arc;

use anyhow::Context;
use anyhow::Result;
use bvh::aabb::{AABB, Bounded};
use bvh::bounding_hierarchy::BHShape;
use either::Either;
use glam::Mat3A;
use rand::prelude::SmallRng;
use serde::{Deserialize, Serialize};

use crate::{EPSILON, Float, Transform, Vector};
use crate::buildable::{Buildable, BuildableTransformed};
use crate::helpers::rgb_luminance;
use crate::lights::Emitter;
use crate::local_vector::{LocalVector, LocalVectorFrame};
use crate::material::{Material, MaterialBuilder, MaterialInterface};
use crate::object_like::hit::{Hit, HitGroup};
use crate::object_like::ObjectInterface;
use crate::object_like::shape::{OOBB, Shape, ShapeDeserialize, ShapeInterface, ShapeSampler};
use crate::object_like::transform::{parse_transform, TransformDeserialize};
use crate::ray::Ray;
use crate::scene::{Scene, SceneBuilder};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ObjectBuilder {
    pub shape: ShapeDeserialize,
    #[serde(default, alias = "transform")]
    pub transforms: TransformDeserialize,
    #[serde(default = "default_material", with = "either::serde_untagged")]
    pub material: Either<String, MaterialBuilder>,
}

fn default_material() -> Either<String, MaterialBuilder> {
    Either::Right(MaterialBuilder::default())
}

impl BuildableTransformed for ObjectBuilder {
    type Target = Object;

    fn build_transformed(&self, scene: &SceneBuilder, transform: Transform) -> Result<Self::Target> {
        let this_transform = parse_transform(&self.transforms);
        let mat_to_world = transform * this_transform;

        let (scale, ..) = mat_to_world.to_scale_rotation_translation();
        let uniform_a = (scale.x - scale.y).abs() < 0.00001;
        let uniform_b = (scale.y - scale.z).abs() < 0.00001;
        let uniform_scale_2 = (uniform_a && uniform_b).then_some(scale.x.powi(2));

        let material = match &self.material {
            Either::Left(name) => {
                scene.compiled_materials.get(name).cloned()
                    .with_context(|| format!("couldn't find material named: \"{name}\""))?
            }
            Either::Right(mat) => Arc::new(mat.build(scene)?),
        };

        let mat_to_local = mat_to_world.inverse();

        let mat_to_world_normal = mat_to_local.matrix3.transpose();
        let mat_to_local_normal = mat_to_world.matrix3.transpose();

        let shape = self.shape.get_shape()?;

        let obj = Object {
            shape,
            mat_to_world,
            mat_to_local,
            mat_to_world_normal,
            mat_to_local_normal,
            material,
            uniform_scale_2,
            bvh_index: 0,
        };

        Ok(obj)
    }
}

#[derive(Debug, Clone)]
pub struct Object {
    pub shape: Shape,
    pub mat_to_world: Transform,
    pub mat_to_local: Transform,
    pub mat_to_world_normal: Mat3A,
    pub mat_to_local_normal: Mat3A,
    pub material: Arc<Material>,

    pub uniform_scale_2: Option<Float>,
    bvh_index: usize,
}

impl Object {
    pub fn normal_to_world(&self, normal: Vector) -> Vector {
        (self.mat_to_world_normal * normal).normalize()
    }

    pub fn normal_to_local(&self, normal: Vector) -> Vector {
        (self.mat_to_local_normal * normal).normalize()
    }

    pub fn supports_nee(&self) -> bool {
        self.uniform_scale_2.is_some() && self.shape.supports_nee()
    }
}

impl ObjectInterface for Object {
    fn try_hit(&self, ray: Ray, min_t: Float) -> Option<HitGroup> {
        // Doing some offsetting here to hopefully reduce floating point precision loss

        let t_approx = match self.shape {
            Shape::Plane(_) => 0.0, // Special case to prevent precision loss with planes
            // _ => 0.0, // Turn off offsetting for testing
            _ => ray.dir.dot(self.mat_to_world.translation - ray.origin),
        };

        let shifted_origin = ray.origin + ray.dir * t_approx;
        let shifted_ray = Ray::new(shifted_origin, ray.dir);

        let mut transformed_ray = shifted_ray.transform(&self.mat_to_local);
        let scaling_factor = transformed_ray.dir.length();
        transformed_ray.dir /= scaling_factor;

        let hits = self.shape.try_hit(transformed_ray, min_t - t_approx, self);
        hits.map(|mut hits| {
            for hit in hits.iter_mut() {
                hit.adjust_t(t_approx, scaling_factor)
            }
            hits
        })
    }
}

impl ShapeSampler for Object {
    fn sample_point(&self, rng: &mut SmallRng, origin: Vector) -> (Vector, Vector, Float) {
        let origin = self.mat_to_local.transform_point3a(origin);
        let (mut hit, mut normal, mut pdf) = self.shape.sample_point(rng, origin);

        hit = self.mat_to_world.transform_point3a(hit);
        normal = self.normal_to_world(normal);
        pdf /= self.uniform_scale_2
            .expect("object without uniform scaling ended up as an area light");

        (hit, normal, pdf)
    }

    fn sample_pdf(&self, origin: Vector, hit: Vector, normal: Vector) -> Float {
        let origin = self.mat_to_local.transform_point3a(origin);
        let hit = self.mat_to_local.transform_point3a(hit);
        let normal = self.normal_to_local(normal);

        self.shape.sample_pdf(origin, hit, normal) / self.uniform_scale_2
            .expect("object without uniform scaling ended up as an area light")
    }

    fn area(&self) -> Float {
        self.shape.area() * self.uniform_scale_2
            .expect("objects without uniform scaling can't have their area calculated")
    }

    fn supports_nee(&self) -> bool {
        self.uniform_scale_2.is_some() && self.shape.supports_nee()
    }
}

impl Bounded for Object {
    fn aabb(&self) -> AABB {
        let local_aabb = self.shape.aabb();
        let oobb = OOBB::from_aabb(local_aabb);
        oobb.transform(self.mat_to_world).to_aabb()
    }
}

impl BHShape for Object {
    fn set_bh_node_index(&mut self, i: usize) {
        self.bvh_index = i
    }

    fn bh_node_index(&self) -> usize {
        self.bvh_index
    }
}

impl Emitter for Object {
    fn estimate_throughput(&self, rng: &mut SmallRng, scene: &Scene, hit: &Hit, frame: LocalVectorFrame, wo: LocalVector, choose_weight: Float) -> Option<Vector> {
        if std::ptr::eq(hit.object, self) {
            // No double dipping
            return None;
        }

        // the probability of picking this light
        let pdf_pick_light = choose_weight;

        // pick a random point on the chosen light source
        let (y, n_y, pdf_y) = self.sample_point(rng, hit.point);

        // ray direction
        let xy = y - hit.point;
        // normalized direction towards light source
        let r_xy = xy.length();
        let w_i_nee = xy / r_xy;
        let wi = frame.to_mat(w_i_nee);

        // cosine angle between the NEE ray and the shading normal N_x at x
        // the max is there to allow scattering only in the upper hemisphere
        let _cos_theta_x = w_i_nee.dot(hit.normal).max(0.0);

        // cosine angle between the NEE ray and the light normal n_y at y
        let cos_theta_y = -n_y.dot(w_i_nee);
        let n_y = n_y * cos_theta_y.signum(); // correct facing normal
        let _cos_theta_y = cos_theta_y.abs(); // 2-sided lights

        let pdf_nee_a = pdf_y * pdf_pick_light;
        let pdf_nee_sa = pdf_nee_a * r_xy * r_xy;
        let pdf_bsdf_sa = hit.object.material.pdf(wi, wo);

        // power heuristic
        // This is pre divided by the pdf
        let mis_nee = pdf_nee_sa / (pdf_nee_sa.powi(2) + pdf_bsdf_sa.powi(2));

        // visibility
        let ray = Ray::new(hit.point + hit.normal * EPSILON * wi.z.signum(), w_i_nee);
        let emitted = match self.cast_visibility(scene, ray, y, n_y) {
            None => return None,
            Some(v) => v,
        };

        let brdf = hit.object.material.eval(wi, wo);

        Some(mis_nee * brdf * emitted)
        // Some(mis_nee * brdf * cos_theta_x * emitted)
    }

    fn get_cached_weight(&self) -> Float {
        let emission = self.material.emission()
            .expect("called NEE weight function on material without emission");
        self.area() * rgb_luminance(emission)
    }
}

impl Object {
    fn cast_visibility(&self, scene: &Scene, ray: Ray, expected_point: Vector, _expected_normal: Vector) -> Option<Vector> {
        scene.try_hit(ray)
            .filter(|hit| std::ptr::eq(hit.object, self))
            .filter(|hit| (hit.point - expected_point).length_squared() < EPSILON)
            // .filter(|hit| hit.normal.dot(expected_normal) > 1.0 - EPSILON)
            .map(|hit| hit.object.material.emission().expect("light has no emission??"))
    }
}

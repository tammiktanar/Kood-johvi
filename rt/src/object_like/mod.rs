use std::sync::Arc;

use anyhow::Context;
use bvh::aabb::{AABB, Bounded};
use bvh::bounding_hierarchy::BHShape;
use either::Either;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

pub use object::*;

use crate::{Float, Transform};
use crate::buildable::{BuildableTransformed, PostBuildable};
use crate::lights::{Emitter, Light};
use crate::material::MaterialInterface;
use crate::object_like::csg::{Difference, DifferenceBuilder, Intersection, IntersectionBuilder, Union, UnionBuilder};
use crate::object_like::group::{ObjectGroup, ObjectGroupBuilder};
use crate::object_like::hit::HitGroup;
use crate::object_like::instancer::InstancerBuilder;
use crate::ray::Ray;
use crate::scene::SceneBuilder;

pub mod object;
pub mod csg;
pub mod shape;
pub mod hit;
pub mod group;
pub mod transform;
pub mod instancer;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ObjectDeserialize {
    #[serde(with = "either::serde_untagged")]
    object: Either<String, ObjectLikeBuilder>,
}

impl ObjectDeserialize {
    /// Either returns the inline object or fetches it from the objects map
    fn get_object<'a>(&'a self, scene: &'a SceneBuilder) -> anyhow::Result<&'a ObjectLikeBuilder> {
        let obj = match &self.object {
            Either::Left(key) => {
                scene.object_templates.get(key)
                    .context(format!("could not find an object template named \"{key}\""))?
            }
            Either::Right(obj) => obj,
        };
        Ok(obj)
    }
}

pub fn build_object_vec(objects: &[ObjectDeserialize], transform: Transform, scene: &SceneBuilder) -> anyhow::Result<Vec<ObjectLike>> {
    objects.iter()
        .map(|obj| obj.get_object(scene))
        .map_ok(|obj| obj.build_transformed(scene, transform))
        .try_collect()?
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ObjectLikeBuilder {
    Object(Box<ObjectBuilder>),
    #[serde(rename = "group")]
    ObjectGroup(Box<ObjectGroupBuilder>),
    Instancer(Box<InstancerBuilder>),
    Union(Box<UnionBuilder>),
    Difference(Box<DifferenceBuilder>),
    Intersection(Box<IntersectionBuilder>),
}

impl BuildableTransformed for ObjectLikeBuilder {
    type Target = ObjectLike;

    fn build_transformed(&self, scene: &SceneBuilder, transform: Transform) -> anyhow::Result<Self::Target> {
        let obj = match self {
            ObjectLikeBuilder::Object(v) => ObjectLike::Object(v.build_transformed(scene, transform)?.into()),
            ObjectLikeBuilder::ObjectGroup(v) => ObjectLike::ObjectGroup(v.build_transformed(scene, transform)?),
            ObjectLikeBuilder::Instancer(v) => ObjectLike::ObjectGroup(v.build_transformed(scene, transform)?),
            ObjectLikeBuilder::Union(v) => ObjectLike::Union(v.build_transformed(scene, transform)?.into()),
            ObjectLikeBuilder::Difference(v) => ObjectLike::Difference(v.build_transformed(scene, transform)?.into()),
            ObjectLikeBuilder::Intersection(v) => ObjectLike::Intersection(v.build_transformed(scene, transform)?.into()),
        };

        Ok(obj)
    }
}

// pub trait FinalizeTransforms {
//     fn finalize_transforms(&mut self, transform: Transform);
// }
//
// impl FinalizeTransforms for ObjectLikeBuilder {
//     fn finalize_transforms(&mut self, transform: Transform) {
//         match self {
//             ObjectLikeBuilder::Object(v) => v.finalize_transforms(transform),
//             ObjectLikeBuilder::ObjectGroup(v) => v.finalize_transforms(transform),
//             ObjectLikeBuilder::Union(v) => v.finalize_transforms(transform),
//             ObjectLikeBuilder::Difference(v) => v.finalize_transforms(transform),
//             ObjectLikeBuilder::Intersection(v) => v.finalize_transforms(transform),
//         }
//     }
// }

pub trait ObjectInterface {
    fn try_hit(&self, ray: Ray, min_t: Float) -> Option<HitGroup>;
}

#[derive(Debug)]
pub enum ObjectLike {
    Object(Arc<Object>),
    ObjectGroup(ObjectGroup),
    Union(Box<Union>),
    Difference(Box<Difference>),
    Intersection(Box<Intersection>),
}

impl ObjectInterface for ObjectLike {
    fn try_hit(&self, ray: Ray, min_t: Float) -> Option<HitGroup> {
        match self {
            ObjectLike::Object(v) => v.try_hit(ray, min_t),
            ObjectLike::ObjectGroup(v) => v.try_hit(ray, min_t),
            ObjectLike::Union(v) => v.try_hit(ray, min_t),
            ObjectLike::Difference(v) => v.try_hit(ray, min_t),
            ObjectLike::Intersection(v) => v.try_hit(ray, min_t),
        }
    }
}

impl Bounded for ObjectLike {
    fn aabb(&self) -> AABB {
        match self {
            ObjectLike::Object(v) => v.aabb(),
            ObjectLike::ObjectGroup(v) => v.aabb(),
            ObjectLike::Union(v) => (v).aabb(),
            ObjectLike::Difference(v) => v.aabb(),
            ObjectLike::Intersection(v) => v.aabb(),
        }
    }
}

impl BHShape for ObjectLike {
    fn set_bh_node_index(&mut self, i: usize) {
        match self {
            ObjectLike::Object(v) => {
                let obj = Arc::get_mut(v).unwrap();
                obj.set_bh_node_index(i)
            }
            ObjectLike::ObjectGroup(v) => v.set_bh_node_index(i),
            ObjectLike::Union(v) => (v).set_bh_node_index(i),
            ObjectLike::Difference(v) => v.set_bh_node_index(i),
            ObjectLike::Intersection(v) => v.set_bh_node_index(i),
        }
    }

    fn bh_node_index(&self) -> usize {
        match self {
            ObjectLike::Object(v) => v.bh_node_index(),
            ObjectLike::ObjectGroup(v) => v.bh_node_index(),
            ObjectLike::Union(v) => (v).bh_node_index(),
            ObjectLike::Difference(v) => v.bh_node_index(),
            ObjectLike::Intersection(v) => v.bh_node_index(),
        }
    }
}

impl PostBuildable for ObjectLike {
    fn post_build(&self, lights: &mut Vec<(Light, Float)>) {
        match self {
            ObjectLike::ObjectGroup(v) => v.post_build(lights),
            ObjectLike::Union(v) => v.post_build(lights),
            ObjectLike::Difference(v) => v.post_build(lights),
            ObjectLike::Intersection(v) => v.post_build(lights),
            ObjectLike::Object(obj) => {
                if obj.supports_nee() && obj.material.emission().is_some() {
                    lights.push((Light::Area(obj.clone()), obj.get_cached_weight()))
                }
            }
        }
    }
}
use anyhow::Result;
use bvh::aabb::{AABB, Bounded};
use bvh::bounding_hierarchy::BHShape;
use either::Either;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{Float, Transform};
use crate::buildable::{BuildableTransformed, PostBuildable};
use crate::helpers::flatten_objects;
use crate::lights::Light;
use crate::object_like::{ObjectDeserialize, ObjectInterface, ObjectLike};
use crate::object_like::hit::HitGroup;
use crate::object_like::transform::{parse_transform, TransformDeserialize};
use crate::ray::Ray;
use crate::scene::SceneBuilder;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectGroupBuilder {
    #[serde(default, alias = "transform")]
    transforms: TransformDeserialize,
    #[serde(alias = "object", with = "either::serde_untagged")]
    objects: Either<ObjectDeserialize, Vec<ObjectDeserialize>>,
}

impl BuildableTransformed for ObjectGroupBuilder {
    type Target = ObjectGroup;

    fn build_transformed(&self, scene: &SceneBuilder, transform: Transform) -> Result<Self::Target> {
        let this_transform = parse_transform(&self.transforms);
        let next_transform = transform * this_transform;

        let mut objects = match &self.objects {
            Either::Left(one) => {
                vec![one.get_object(scene)?.build_transformed(scene, transform)?]
            }
            Either::Right(many) => {
                many.iter()
                    .map(|obj| obj.get_object(scene)?.build_transformed(scene, next_transform))
                    .try_collect()?
            }
        };

        flatten_objects(&mut objects);

        let group = ObjectGroup {
            objects,
            bvh_index: 0,
        };

        Ok(group)
    }
}

#[derive(Debug)]
pub struct ObjectGroup {
    pub objects: Vec<ObjectLike>,
    pub(crate) bvh_index: usize,
}

impl ObjectInterface for ObjectGroup {
    fn try_hit(&self, ray: Ray, min_t: Float) -> Option<HitGroup> {
        Some(self.objects.iter()
            .filter_map(|obj| obj.try_hit(ray, min_t))
            .flatten()
            .collect())
            .filter(|v: &HitGroup| !v.is_empty())
    }
}

impl Bounded for ObjectGroup {
    fn aabb(&self) -> AABB {
        let mut result = AABB::empty();

        self.objects.iter()
            .map(|child| child.aabb())
            .for_each(|aabb| {
                result.join_mut(&aabb);
            });

        result
    }
}

impl BHShape for ObjectGroup {
    fn set_bh_node_index(&mut self, i: usize) {
        self.bvh_index = i
    }

    fn bh_node_index(&self) -> usize {
        self.bvh_index
    }
}

impl PostBuildable for ObjectGroup {
    fn post_build(&self, lights: &mut Vec<(Light, Float)>) {
        self.objects.iter()
            .for_each(|child| child.post_build(lights))
    }
}

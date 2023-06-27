use bvh::aabb::{AABB, Bounded};
use bvh::bounding_hierarchy::BHShape;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{Float, Transform};
use crate::buildable::{BuildableTransformed, PostBuildable};
use crate::helpers::flatten_objects;
use crate::lights::Light;
use crate::object_like::{ObjectDeserialize, ObjectInterface, ObjectLike};
use crate::object_like::hit::{HitGroup, Transition};
use crate::object_like::transform::{parse_transform, TransformDeserialize};
use crate::ray::Ray;
use crate::scene::SceneBuilder;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnionBuilder {
    #[serde(default, alias = "transform")]
    transforms: TransformDeserialize,
    #[serde(alias = "children")]
    objects: Vec<ObjectDeserialize>,
}

impl BuildableTransformed for UnionBuilder {
    type Target = Union;

    fn build_transformed(&self, scene: &SceneBuilder, transform: Transform) -> anyhow::Result<Self::Target> {
        let this_transform = parse_transform(&self.transforms);
        let next_transform = transform * this_transform;

        let mut objects: Vec<_> = self.objects.iter()
            .map(|obj| obj.get_object(scene)?.build_transformed(scene, next_transform))
            .try_collect()?;

        flatten_objects(&mut objects);

        let union = Union {
            objects,
            bvh_index: 0,
        };

        Ok(union)
    }
}

#[derive(Debug)]
pub struct Union {
    objects: Vec<ObjectLike>,
    bvh_index: usize,
}

impl ObjectInterface for Union {
    fn try_hit(&self, ray: Ray, min_t: Float) -> Option<HitGroup> {
        self.objects.iter()
            .map(|obj| obj.try_hit(ray, min_t))
            .reduce(union)
            .unwrap_or(None)
    }
}

impl PostBuildable for Union {
    fn post_build(&self, lights: &mut Vec<(Light, Float)>) {
        self.objects.iter().for_each(|obj| obj.post_build(lights))
    }
}

impl Bounded for Union {
    fn aabb(&self) -> AABB {
        self.objects.iter()
            .map(|obj| obj.aabb())
            .reduce(|a, b| a.join(&b))
            .unwrap_or(AABB::empty())
    }
}

impl BHShape for Union {
    fn set_bh_node_index(&mut self, i: usize) {
        self.bvh_index = i
    }

    fn bh_node_index(&self) -> usize {
        self.bvh_index
    }
}


fn union<'a>(a_hits: Option<HitGroup<'a>>, b_hits: Option<HitGroup<'a>>) -> Option<HitGroup<'a>> {
    if a_hits.is_none() {
        return b_hits;
    }

    if b_hits.is_none() {
        return a_hits;
    }

    let a_iter = a_hits.unwrap().into_iter().map(|v| (v, true));
    let b_iter = b_hits.unwrap().into_iter().map(|v| (v, false));

    let mut a_count = 0;
    let mut b_count = 0;

    let res = a_iter.merge_by(b_iter, |(x, _), (y, _)| x.t <= y.t)
        .filter(|(hit, is_a)| {
            match hit.transition {
                Transition::Enter => {
                    let res = !(a_count > 0 || b_count > 0); // Not inside anything

                    if *is_a {
                        a_count += 1;
                    } else {
                        b_count += 1;
                    }

                    res
                }
                Transition::Exit => {
                    if *is_a {
                        a_count -= 1;
                    } else {
                        b_count -= 1;
                    }

                    !(a_count > 0 || b_count > 0) // Now not inside anything
                }
                Transition::EnterExit => {
                    if *is_a {
                        b_count <= 0 // Not inside b
                    } else {
                        a_count <= 0 // Not inside a
                    }
                }
            }
        })
        .map(|(hit, _)| hit)
        .collect();

    Some(res)
}
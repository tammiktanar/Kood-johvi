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
pub struct DifferenceBuilder {
    #[serde(default, alias = "transform")]
    transforms: TransformDeserialize,
    #[serde(alias = "children")]
    objects: Vec<ObjectDeserialize>,
}

impl BuildableTransformed for DifferenceBuilder {
    type Target = Difference;

    fn build_transformed(&self, scene: &SceneBuilder, transform: Transform) -> anyhow::Result<Self::Target> {
        let this_transform = parse_transform(&self.transforms);
        let next_transform = transform * this_transform;

        let mut objects: Vec<_> = self.objects.iter()
            .map(|obj| obj.get_object(scene)?.build_transformed(scene, next_transform))
            .try_collect()?;

        flatten_objects(&mut objects);

        let union = Difference {
            objects,
            bvh_index: 0,
        };

        Ok(union)
    }
}

#[derive(Debug)]
pub struct Difference {
    objects: Vec<ObjectLike>,
    bvh_index: usize,
}

impl ObjectInterface for Difference {
    fn try_hit(&self, ray: Ray, min_t: Float) -> Option<HitGroup> {
        self.objects.iter()
            .map(|obj| obj.try_hit(ray, min_t))
            .reduce(difference)
            .unwrap_or(None)
    }
}

impl PostBuildable for Difference {
    fn post_build(&self, lights: &mut Vec<(Light, Float)>) {
        self.objects.iter().for_each(|obj| obj.post_build(lights))
    }
}

impl Bounded for Difference {
    fn aabb(&self) -> AABB {
        self.objects.first()
            .map(|obj| obj.aabb())
            .unwrap_or(AABB::empty())
    }
}

impl BHShape for Difference {
    fn set_bh_node_index(&mut self, i: usize) {
        self.bvh_index = i
    }

    fn bh_node_index(&self) -> usize {
        self.bvh_index
    }
}


fn difference<'a>(a_hits: Option<HitGroup<'a>>, b_hits: Option<HitGroup<'a>>) -> Option<HitGroup<'a>> {
    if a_hits.is_none() || b_hits.is_none() {
        return a_hits;
    }

    let a_iter = a_hits.unwrap().into_iter().map(|v| (v, true));
    let b_iter = b_hits.unwrap().into_iter().map(|v| (v, false));

    let mut a_count = 0;
    let mut b_count = 0;

    let res: HitGroup = a_iter.merge_by(b_iter, |(x, _), (y, _)| x.t <= y.t)
        .filter(|(hit, is_a)| {
            // println!("union - t: {}, is_a: {}, ac: {} bc: {}, shape: {:?}", hit.t, is_a, a_count, b_count, hit.object.shape);
            if *is_a {
                // A is...
                match hit.transition {
                    Transition::Enter => {
                        a_count += 1;
                    }
                    Transition::Exit => {
                        a_count -= 1;
                    }
                    Transition::EnterExit => {}
                }
                b_count <= 0 // Not in b
            } else {
                // B is...
                match hit.transition {
                    Transition::Enter => {
                        b_count += 1;
                    }
                    Transition::Exit => {
                        b_count -= 1;
                    }
                    Transition::EnterExit => {}
                }
                a_count > 0 // In a
            }
        })
        .map(|(mut hit, is_a)| {
            // Reverse b transitions
            if is_a {
                hit
            } else {
                match hit.transition {
                    Transition::Enter => {
                        hit.transition = Transition::Exit;
                        hit.flip_normal = !hit.flip_normal;
                        hit
                    }
                    Transition::Exit => {
                        hit.transition = Transition::Enter;
                        hit.flip_normal = !hit.flip_normal;
                        hit
                    }
                    Transition::EnterExit => {
                        hit
                    }
                }
            }
        })
        .collect();

    Some(res)
}
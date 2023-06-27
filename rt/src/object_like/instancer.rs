use anyhow::Result;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::buildable::BuildableTransformed;
use crate::helpers::flatten_objects;
use crate::object_like::group::ObjectGroup;
use crate::object_like::ObjectDeserialize;
use crate::object_like::transform::{parse_transform, TransformDeserialize};
use crate::scene::SceneBuilder;
use crate::Transform;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstancerBuilder {
    object: ObjectDeserialize,
    #[serde(default)]
    transform_all: TransformDeserialize,
    #[serde(default, alias = "transform")]
    transforms: Vec<TransformDeserialize>,
}

impl BuildableTransformed for InstancerBuilder {
    type Target = ObjectGroup;

    fn build_transformed(&self, scene: &SceneBuilder, transform: Transform) -> Result<Self::Target> {
        let transform = transform * parse_transform(&self.transform_all);

        let mut objects = self.transforms.iter()
            .map(parse_transform)
            .map(|this_transform| transform * this_transform)
            .map(|next_transform| self.object.get_object(scene)?.build_transformed(scene, next_transform))
            .try_collect()?;

        flatten_objects(&mut objects);

        let group = ObjectGroup {
            objects,
            bvh_index: 0,
        };

        Ok(group)
    }
}

use anyhow::Result;

use crate::{Float, Transform};
use crate::lights::Light;
use crate::scene::SceneBuilder;

pub(crate) trait Buildable {
    type Target;

    fn build(&self, scene: &SceneBuilder) -> Result<Self::Target>;
}

pub(crate) trait BuildableTransformed {
    type Target;

    fn build_transformed(&self, scene: &SceneBuilder, transform: Transform) -> Result<Self::Target>;
}

impl<T> Buildable for T
where T: BuildableTransformed {
    type Target = T::Target;

    fn build(&self, scene: &SceneBuilder) -> Result<Self::Target> {
        self.build_transformed(scene, Transform::IDENTITY)
    }
}

pub(crate) trait PostBuildable {
    fn post_build(&self, lights: &mut Vec<(Light, Float)>);
}
use rand::prelude::SmallRng;
use serde::{Deserialize, Serialize};

pub use diffuse::*;
pub use glass::*;
pub use glossy::*;
pub use metal::*;
pub use mirror::*;
pub use phong::*;

use crate::{Float, Vector};
use crate::buildable::Buildable;
use crate::local_vector::LocalVector;
use crate::scene::SceneBuilder;

mod glossy;
mod glass;
mod metal;
mod math;
mod diffuse;
mod mirror;
mod phong;

pub trait MaterialInterface {
    fn eval(&self, wi: LocalVector, wo: LocalVector) -> Vector;

    fn sample(&self, rng: &mut SmallRng, wo: LocalVector, specular_bounce: &mut bool) -> (LocalVector, Vector);

    fn pdf(&self, wi: LocalVector, wo: LocalVector) -> Float;

    fn emission(&self) -> Option<Vector>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MaterialBuilder {
    Glossy(Glossy),
    Glass(Glass),
    Metal(Metal),
    Mirror(Mirror),
    Diffuse(Diffuse),
    #[serde(skip)]
    Phong(Phong),
}

impl Default for MaterialBuilder {
    fn default() -> Self {
        Self::Diffuse(Diffuse::default())
    }
}

impl Buildable for MaterialBuilder {
    type Target = Material;

    fn build(&self, _scene: &SceneBuilder) -> anyhow::Result<Self::Target> {
        let mat = match self {
            MaterialBuilder::Glossy(v) => Material::Glossy(v.clone()),
            MaterialBuilder::Glass(v) => Material::Glass(v.clone()),
            MaterialBuilder::Metal(v) => Material::Metal(v.clone()),
            MaterialBuilder::Mirror(v) => Material::Mirror(v.clone()),
            MaterialBuilder::Diffuse(v) => Material::Diffuse(v.clone()),
            MaterialBuilder::Phong(v) => Material::Phong(v.clone()),
        };

        Ok(mat)
    }
}

#[derive(Debug)]
pub enum Material {
    Glossy(Glossy),
    Glass(Glass),
    Metal(Metal),
    Mirror(Mirror),
    Diffuse(Diffuse),
    Phong(Phong),
}

impl MaterialInterface for Material {
    fn eval(&self, wi: LocalVector, wo: LocalVector) -> Vector {
        match self {
            Material::Glossy(v) => v.eval(wi, wo),
            Material::Glass(v) => v.eval(wi, wo),
            Material::Metal(v) => v.eval(wi, wo),
            Material::Mirror(v) => v.eval(wi, wo),
            Material::Diffuse(v) => v.eval(wi, wo),
            Material::Phong(v) => v.eval(wi, wo),
        }
    }

    fn sample(&self, rng: &mut SmallRng, wo: LocalVector, specular_bounce: &mut bool) -> (LocalVector, Vector) {
        match self {
            Material::Glossy(v) => v.sample(rng, wo, specular_bounce),
            Material::Glass(v) => v.sample(rng, wo, specular_bounce),
            Material::Metal(v) => v.sample(rng, wo, specular_bounce),
            Material::Mirror(v) => v.sample(rng, wo, specular_bounce),
            Material::Diffuse(v) => v.sample(rng, wo, specular_bounce),
            Material::Phong(v) => v.sample(rng, wo, specular_bounce),
        }
    }

    fn pdf(&self, wi: LocalVector, wo: LocalVector) -> Float {
        match self {
            Material::Glossy(v) => v.pdf(wi, wo),
            Material::Glass(v) => v.pdf(wi, wo),
            Material::Metal(v) => v.pdf(wi, wo),
            Material::Mirror(v) => v.pdf(wi, wo),
            Material::Diffuse(v) => v.pdf(wi, wo),
            Material::Phong(v) => v.pdf(wi, wo),
        }
    }

    fn emission(&self) -> Option<Vector> {
        match self {
            Material::Glossy(v) => v.emission(),
            Material::Glass(v) => v.emission(),
            Material::Metal(v) => v.emission(),
            Material::Mirror(v) => v.emission(),
            Material::Diffuse(v) => v.emission(),
            Material::Phong(v) => v.emission(),
        }
    }
}

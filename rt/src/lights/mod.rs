use std::sync::Arc;

use anyhow::Result;
use rand::rngs::SmallRng;
use serde::{Deserialize, Serialize};

use directional::*;
use point::*;

use crate::{Float, Vector};
use crate::buildable::Buildable;
use crate::local_vector::{LocalVector, LocalVectorFrame};
use crate::object_like::hit::Hit;
use crate::object_like::Object;
use crate::scene::{Scene, SceneBuilder};

pub mod directional;
pub mod point;

pub trait Emitter {
    /// Returns (point, normal, pdf)
    // fn sample_point(&self, rng: &mut SmallRng, scene: &Scene, hit: &Hit) -> (Vector, Vector, Float);
    //
    // fn cast_visibility(&self, scene: &Scene, ray: Ray, expected_hit: Vector) -> Option<Vector>;

    fn estimate_throughput(&self,
                           rng: &mut SmallRng,
                           scene: &Scene,
                           hit: &Hit,
                           frame: LocalVectorFrame,
                           wo: LocalVector,
                           choose_weight: Float,
    ) -> Option<Vector>;

    fn get_cached_weight(&self) -> Float;

    /// Default implementation for non-dirac delta emitters
    fn is_dirac_delta(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum LightBuilder {
    Directional(Directional),
    Point(Point),
}

impl Buildable for LightBuilder {
    type Target = Light;

    fn build(&self, _scene: &SceneBuilder) -> Result<Light> {
        let light = match self {
            LightBuilder::Directional(v) => Light::Directional(v.clone()),
            LightBuilder::Point(v) => Light::Point(v.clone()),
        };
        Ok(light)
    }
}

#[derive(Debug, Clone)]
pub enum Light {
    Directional(Directional),
    Point(Point),
    Area(Arc<Object>),
}

impl Emitter for Light {
    // fn sample_point(&self, rng: &mut SmallRng, scene: &Scene, hit: &Hit) -> (Vector, Vector, Float) {
    //     match self {
    //         Light::Directional(v) => v.sample_point(rng, scene, hit),
    //         Light::Point(v) => v.sample_point(rng, scene, hit),
    //         Light::Area(_) => unimplemented!(),
    //     }
    // }

    // fn cast_visibility(&self, scene: &Scene, ray: Ray, expected_hit: Vector) -> Option<Vector> {
    //     match self {
    //         Light::Directional(v) => v.cast_visibility(scene, ray, expected_hit),
    //         Light::Point(v) => v.cast_visibility(scene, ray, expected_hit),
    //         Light::Area(_) => unimplemented!(),
    //     }
    // }

    fn estimate_throughput(&self, rng: &mut SmallRng, scene: &Scene, hit: &Hit, frame: LocalVectorFrame, wo: LocalVector, choose_weight: Float) -> Option<Vector> {
        match self {
            Light::Directional(v) => v.estimate_throughput(rng, scene, hit, frame, wo, choose_weight),
            Light::Point(v) => v.estimate_throughput(rng, scene, hit, frame, wo, choose_weight),
            Light::Area(v) => v.estimate_throughput(rng, scene, hit, frame, wo, choose_weight),
        }
    }

    fn get_cached_weight(&self) -> Float {
        match self {
            Light::Directional(v) => v.get_cached_weight(),
            Light::Point(v) => v.get_cached_weight(),
            Light::Area(v) => v.get_cached_weight(),
        }
    }
}
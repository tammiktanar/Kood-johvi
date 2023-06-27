use rand::prelude::SmallRng;
use serde::{Deserialize, Serialize};

use crate::{EPSILON, Float, Vector};
use crate::helpers::rgb_luminance;
use crate::lights::Emitter;
use crate::local_vector::{LocalVector, LocalVectorFrame};
use crate::material::MaterialInterface;
use crate::object_like::hit::Hit;
use crate::ray::Ray;
use crate::scene::Scene;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Directional {
    #[serde(deserialize_with = "crate::validators::de_vec3_gte_0")]
    emission: Vector,
    #[serde(deserialize_with = "crate::validators::de_vec3_normalized")]
    direction: Vector,
}

impl Emitter for Directional {
    fn estimate_throughput(&self, _rng: &mut SmallRng, scene: &Scene, hit: &Hit, frame: LocalVectorFrame, wo: LocalVector, choose_weight: Float) -> Option<Vector> {
        let light_direction = -self.direction;

        let wi = frame.to_mat(light_direction);

        let ray = Ray::new(hit.point + hit.normal * EPSILON * wi.z.signum(), light_direction);

        self.cast_visibility(scene, ray)
            .map(|emission| {
                let pdf_pick_light = 1.0 / scene.lights.len() as Float;
                let brdf = hit.object.material.eval(wi, wo);
                emission * brdf / (pdf_pick_light * choose_weight)
            })
    }

    fn get_cached_weight(&self) -> Float {
        rgb_luminance(self.emission)
    }

    fn is_dirac_delta(&self) -> bool {
        true
    }
}

impl Directional {
    fn cast_visibility(&self, scene: &Scene, ray: Ray) -> Option<Vector> {
        scene.try_hit(ray).is_none().then_some(self.emission)
    }
}
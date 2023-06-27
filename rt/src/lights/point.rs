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
pub struct Point {
    #[serde(deserialize_with = "crate::validators::de_vec3_gte_0")]
    emission: Vector,
    position: Vector,
}

impl Emitter for Point {
    fn estimate_throughput(&self, _rng: &mut SmallRng, scene: &Scene, hit: &Hit, frame: LocalVectorFrame, wo: LocalVector, choose_weight: Float) -> Option<Vector> {
        let to_light = self.position - hit.point;
        let to_light_dir = to_light.normalize();

        let wi = frame.to_mat(to_light_dir);

        let ray = Ray::new(hit.point + hit.normal * EPSILON * wi.z.signum(), to_light_dir);

        self.cast_visibility(scene, ray, to_light, hit)
            .map(|emission| {
                let pdf_pick_light = 1.0 / scene.lights.len() as Float;
                let brdf = hit.object.material.eval(wi, wo);
                emission * brdf / (pdf_pick_light * to_light.length_squared() * choose_weight)
            })
    }

    fn get_cached_weight(&self) -> Float {
        rgb_luminance(self.emission)
    }

    fn is_dirac_delta(&self) -> bool {
        true
    }
}

impl Point {
    fn cast_visibility(&self, scene: &Scene, ray: Ray, to_light: Vector, _hit: &Hit) -> Option<Vector> {
        match scene.try_hit(ray) {
            None => Some(self.emission),
            Some(hit) => {
                (hit.t.powi(2) > to_light.length_squared()).then_some(self.emission)
            }
        }
    }
}

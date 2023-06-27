use rand::prelude::SmallRng;
use serde::{Deserialize, Serialize};

use crate::{Float, Vector};
use crate::local_vector::LocalVector;
use crate::material::MaterialInterface;
use crate::material::math::microfacet::{eval_specular_ggx, importance_sample_ggx_d_double_sided, importance_sample_ggx_d_pdf};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Metal {
    #[serde(deserialize_with = "crate::validators::de_emission")]
    emission: Option<Vector>,
    color: Vector,
    #[serde(deserialize_with = "crate::validators::de_float_one_clamped")]
    roughness: Float,
}

impl Default for Metal {
    fn default() -> Self {
        Self {
            emission: None,
            color: Vector::splat(0.8),
            roughness: 0.1,
        }
    }
}

impl MaterialInterface for Metal {
    fn eval(&self, wi: LocalVector, wo: LocalVector) -> Vector {
        eval_specular_ggx(wi, wo, self.color, self.roughness)
    }

    fn sample(&self, rng: &mut SmallRng, wo: LocalVector, specular_bounce: &mut bool) -> (LocalVector, Vector) {
        *specular_bounce = self.roughness <= 0.0;
        importance_sample_ggx_d_double_sided(rng, wo, self.color, self.roughness)
    }

    fn pdf(&self, wi: LocalVector, wo: LocalVector) -> Float {
        importance_sample_ggx_d_pdf(wi, wo, self.roughness)
    }

    // fn sample(&self, rng: &mut SmallRng, wo: MatVector) -> (MatVector, Vector) {
    //     let wi = random_cos_hemisphere_unit_matvector(rng);
    //     (wi, self.eval(wi, wo) / self.pdf(wi, wo))
    // }

    // fn pdf(&self, wi: MatVector, wo: MatVector) -> Float {
    //     wi.dot_normal() / PI
    // }

    // fn sample(&self, rng: &mut SmallRng, wo: MatVector) -> (MatVector, Vector) {
    //     let wi = random_hemisphere_unit_vector(rng, Vector::Z).into();
    //     (wi, self.eval(wi, wo) / self.pdf(wi, wo) * wi.dot_normal())
    // }
    //
    // fn pdf(&self, wi: MatVector, wo: MatVector) -> Float {
    //     (2.0 * PI).recip()
    // }

    fn emission(&self) -> Option<Vector> {
        self.emission
    }
}
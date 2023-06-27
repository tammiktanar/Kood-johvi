use rand::prelude::SmallRng;
use serde::{Deserialize, Serialize};

use crate::{Float, PI, Vector};
use crate::helpers::random_cos_hemisphere_unit_matvector;
use crate::local_vector::LocalVector;
use crate::material::MaterialInterface;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Diffuse {
    #[serde(deserialize_with = "crate::validators::de_emission")]
    pub emission: Option<Vector>,
    #[serde(deserialize_with = "crate::validators::de_vec3_one_clamped")]
    pub color: Vector,
}

impl Default for Diffuse {
    fn default() -> Self {
        Self {
            emission: None,
            color: Vector::splat(0.5),
        }
    }
}

impl MaterialInterface for Diffuse {
    fn eval(&self, wi: LocalVector, wo: LocalVector) -> Vector {
        if wi.dot_normal() * wo.dot_normal() <= 0.0 {
            return Vector::ZERO;
        }

        self.color / PI * wi.dot_normal().abs()
    }

    fn sample(&self, rng: &mut SmallRng, wo: LocalVector, _specular_bounce: &mut bool) -> (LocalVector, Vector) {
        let wi = *random_cos_hemisphere_unit_matvector(rng) * wo.z.signum();
        let throughput = self.color;
        (wi.into(), throughput)
    }

    fn pdf(&self, wi: LocalVector, wo: LocalVector) -> Float {
        if wi.dot_normal() * wo.dot_normal() <= 0.0 {
            return 0.0;
        }
        wi.dot_normal().abs() / PI
    }

    // fn sample(&self, rng: &mut SmallRng, wo: MatVector) -> (MatVector, Vector) {
    //     let wi = random_hemisphere_unit_vector(rng, Vector::Z).into();
    //     // (wi, self.eval(wi, wo) / self.pdf(wi, wo) * wi.dot_normal())
    //     (wi, self.albedo / PI)
    // }
    //
    // fn pdf(&self, wi: MatVector, wo: MatVector) -> Float {
    //     (2.0 * PI).recip()
    // }

    fn emission(&self) -> Option<Vector> {
        self.emission
    }
}
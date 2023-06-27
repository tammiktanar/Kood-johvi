use rand::prelude::SmallRng;
use serde::{Deserialize, Serialize};

use crate::{Float, Vector};
use crate::local_vector::LocalVector;
use crate::material::MaterialInterface;
use crate::material::math::fresnel::reflect_mat;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Mirror {
    #[serde(deserialize_with = "crate::validators::de_emission")]
    emission: Option<Vector>,
    #[serde(deserialize_with = "crate::validators::de_vec3_one_clamped")]
    color: Vector,
}

impl Default for Mirror {
    fn default() -> Self {
        Self {
            emission: None,
            color: Vector::ONE,
        }
    }
}

impl MaterialInterface for Mirror {
    fn eval(&self, _wi: LocalVector, _wo: LocalVector) -> Vector {
        // if wi.x == -wo.x && wi.y == -wo.y && wi.z == wo.z {
        //     self.albedo
        // } else {
        Vector::splat(0.0)
        // }
    }

    fn sample(&self, _rng: &mut SmallRng, wo: LocalVector, specular_bounce: &mut bool) -> (LocalVector, Vector) {
        *specular_bounce = true;
        let wi = reflect_mat(wo);
        let throughput = self.color;
        (wi, throughput)
    }

    fn pdf(&self, _wi: LocalVector, _wo: LocalVector) -> Float {
        // if wi.x == -wo.x && wi.y == -wo.y && wi.z == wo.z {
        //     1.0
        // } else {
        0.0
        // }
    }

    fn emission(&self) -> Option<Vector> {
        self.emission
    }
}
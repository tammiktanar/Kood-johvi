use std::mem;

use rand::prelude::SmallRng;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{Float, Vector};
use crate::local_vector::LocalVector;
use crate::material::MaterialInterface;
use crate::material::math::fresnel::{fresnel, reflect_mat, refract};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Glass {
    #[serde(deserialize_with = "crate::validators::de_emission")]
    emission: Option<Vector>,
    #[serde(deserialize_with = "crate::validators::de_vec3_one_clamped")]
    color: Vector,
    #[serde(deserialize_with = "crate::validators::de_float_gte_1")]
    ior: Float,
}

impl Default for Glass {
    fn default() -> Self {
        Self {
            emission: None,
            color: Vector::ONE,
            ior: 1.5,
        }
    }
}

impl MaterialInterface for Glass {
    fn eval(&self, _wi: LocalVector, _wo: LocalVector) -> Vector {
        Vector::splat(0.0)
    }

    fn sample(&self, rng: &mut SmallRng, mut wo: LocalVector, specular_bounce: &mut bool) -> (LocalVector, Vector) {
        *specular_bounce = true;

        let wm = Vector::Z;
        let m_dot_o = wo.dot(wm);

        let outside = wo.z >= 0.0;

        let mut etai = 1.0;
        let mut etat = self.ior;

        if outside {} else {
            wo.z = -wo.z;
            mem::swap(&mut etai, &mut etat);
        }

        // let f0 = calc_f0(etai, etat);
        let f = fresnel(m_dot_o, etai, etat);

        // let ior =

        let mut wi = if rng.gen_bool(f as f64) {
            // let mut wi = if false {
            reflect_mat(wo)
        } else {
            refract(wo, etai / etat).unwrap_or_else(|| reflect_mat(wo))
        };

        let throughput = if wi.z < 0.0 {
            self.color
        } else {
            Vector::splat(1.0)
        };

        if !outside {
            wi.z = -wi.z;
        }

        (wi, throughput)
    }

    fn pdf(&self, _wi: LocalVector, _wo: LocalVector) -> Float {
        0.0
    }

    fn emission(&self) -> Option<Vector> {
        self.emission
    }
}
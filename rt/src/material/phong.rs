use glam::Quat;
use rand::prelude::SmallRng;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{Float, PI, Vector};
use crate::local_vector::LocalVector;
use crate::material::MaterialInterface;
use crate::material::math::fresnel::reflect;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Phong {
    #[serde(deserialize_with = "crate::validators::de_emission")]
    emission: Option<Vector>,
    #[serde(deserialize_with = "crate::validators::de_vec3_one_clamped")]
    color: Vector,
    #[serde(deserialize_with = "crate::validators::de_vec3_gte_0")]
    specular: Vector,
    #[serde(deserialize_with = "crate::validators::de_float_one_clamped")]
    shininess: Float,
}

impl Default for Phong {
    fn default() -> Self {
        Self {
            emission: None,
            color: Vector::splat(0.5),
            specular: Vector::splat(0.16),
            shininess: 10.0,
        }
    }
}

impl MaterialInterface for Phong {
    fn eval(&self, wi: LocalVector, wo: LocalVector) -> Vector {
        if wi.dot_normal() * wo.dot_normal() <= 0.0 {
            return Vector::ZERO;
        }

        let r = reflect(*wi, Vector::Z);
        let spec = self.specular * (phong_normalization_term(self.shininess) * r.dot(*wo).max(0.0).powf(self.shininess) * wi.dot_normal());
        let diffuse = (Vector::ONE - self.specular) * (self.color / PI) * wi.dot_normal();

        diffuse + spec
    }

    fn sample(&self, rng: &mut SmallRng, wo: LocalVector, _specular_bounce: &mut bool) -> (LocalVector, Vector) {
        let (wi, spec) = sample_phong_specular(rng, wo, self.shininess, self.specular);
        let diffuse = self.color / PI;

        let throughput = diffuse + spec;

        (wi, throughput)
    }

    fn pdf(&self, wi: LocalVector, wo: LocalVector) -> Float {
        let wm = match (*wi + *wo).try_normalize() {
            None => return 0.0,
            Some(v) => LocalVector::from(v),
        };

        if wi.dot_normal() * wo.dot_normal() <= 0.0 {
            return 0.0;
        }

        let spec_pdf = phong_normalization_term(self.shininess) * wm.dot_normal().powf(self.shininess);
        let _diffuse_pdf = wi.dot_normal() / PI;

        spec_pdf
    }

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


fn phong_normalization_term(shininess: Float) -> Float {
    (1.0 + shininess) * (2.0 * PI).recip()
}

fn sample_phong_half_vec(rng: &mut SmallRng, _wo: LocalVector, shininess: Float) -> (LocalVector, Float) {
    let u0: Float = rng.gen();
    let u1: Float = rng.gen();

    let cos_theta = (1.0 - u0).powf(1.0 / (1.0 + shininess));
    let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

    let phi = 2.0 * PI * u1;

    let pdf = phong_normalization_term(shininess) * cos_theta.powf(shininess);

    let wm = LocalVector::new(phi.cos() * sin_theta, phi.sin() * sin_theta, cos_theta);
    assert!(wm.is_normalized());

    (wm, pdf)
}

fn sample_phong_specular(rng: &mut SmallRng, wo: LocalVector, shininess: Float, f0: Vector) -> (LocalVector, Vector) {
    let (l_phong, _) = sample_phong_half_vec(rng, wo, shininess);

    // Sampled l_phong is in "lobe space" - where Phong lobe is centered around +Z axis
    // We need to rotate it in direction of perfect reflection
    let lobe_direction = reflect(*wo, Vector::Z);
    let rot = Quat::from_rotation_arc(Vector::Z.into(), lobe_direction.into());
    let wi = LocalVector::from(rot * *l_phong);

    // Calculate the weight of the sample
    // let Rlocal = reflect(wi, Vector::Z);
    let n_dot_l = wi.dot_normal().max(0.00001);
    let weight = (f0 * n_dot_l).max(Vector::splat(0.0));

    (wi, weight)
}

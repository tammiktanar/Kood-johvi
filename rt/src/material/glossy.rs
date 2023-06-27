use rand::prelude::SmallRng;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{Float, PI, Vector};
use crate::helpers::{lerp, random_cos_hemisphere_unit_matvector};
use crate::local_vector::LocalVector;
use crate::material::MaterialInterface;
use crate::material::math::fresnel::{fresnel_schlick, fresnel_schlick_single};
use crate::material::math::microfacet::{eval_specular_ggx, importance_sample_ggx_d_double_sided, importance_sample_ggx_d_pdf};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Glossy {
    #[serde(deserialize_with = "crate::validators::de_emission")]
    emission: Option<Vector>,
    #[serde(deserialize_with = "crate::validators::de_vec3_one_clamped")]
    color: Vector,
    #[serde(deserialize_with = "crate::validators::de_float_one_clamped")]
    roughness: Float,
    #[serde(deserialize_with = "crate::validators::de_float_one_clamped")]
    reflectance: Float,
}

impl Default for Glossy {
    fn default() -> Self {
        Self {
            emission: None,
            color: Vector::splat(0.6),
            roughness: 0.1,
            reflectance: 0.8,
        }
    }
}

impl MaterialInterface for Glossy {
    fn eval(&self, wi: LocalVector, wo: LocalVector) -> Vector {
        if wi.dot_normal() * wo.dot_normal() <= 0.0 {
            return Vector::ZERO;
        }

        let f0 = Vector::splat(0.16 * self.reflectance.powi(2));

        let specular = eval_specular_ggx(wi, wo, f0, self.roughness);

        let diffuse = (1.0 - fresnel_schlick(wi.dot_normal(), f0)) * (1.0 - fresnel_schlick(wo.dot_normal(), f0)) * (self.color / PI) * wi.dot_normal();
        let diffuse = diffuse * 1.05;

        let res = specular + diffuse;

        if !res.is_finite() {
            eprintln!("specular: {}, diffuse: {}, wi: {}, wo: {}", specular, diffuse, *wi, *wo);
        }

        res
    }

    fn sample(&self, rng: &mut SmallRng, wo: LocalVector, specular_bounce: &mut bool) -> (LocalVector, Vector) {
        // let a = self.roughness.powi(2);
        // let wm_spec = ggx_vndf(wo, a, rng.gen(), rng.gen());
        // let wi_spec = reflect(*wo, wm_spec).into();
        // let wi_diff = random_cos_hemisphere_unit_matvector(rng);
        //
        // let eval_spec = self.eval(wi_spec, wo);
        // let eval_diff = self.eval(wi_diff, wo);
        //
        // let complex_pdf_spec = rgb_luminance(eval_spec);
        // let complex_pdf_diff = rgb_luminance(eval_diff);
        //
        // let simple_pdf_spec = importance_sample_ggx_d_pdf(wi_spec, wo, self.roughness);
        // let simple_pdf_diff = wi_diff.dot_normal() / PI;
        //
        // let weight_spec = if wi_spec.dot_normal() > 0.0 { complex_pdf_spec / simple_pdf_spec } else { 0.0 };
        // let weight_diff = complex_pdf_diff / simple_pdf_diff;
        // let weight_average = (weight_spec + weight_diff) / 2.0;
        //
        // let p = weight_spec / (weight_spec + weight_diff);
        // if !(0.0..=1.0).contains(&p) {
        //     panic!("invalid p: {p}, weight_spec: {weight_spec}, weight_diff: {weight_diff}, simple_pdf_spec: {simple_pdf_spec}, wi_dot: {}\n", wi_spec.dot_normal())
        // }
        // let (wi_good, throughput_good) = if rng.gen_bool(p as f64) {
        //     (wi_spec, eval_spec * (weight_average / weight_spec))
        // } else {
        //     (wi_diff, eval_diff * (weight_average / weight_diff))
        // };
        //
        // if !throughput_good.is_finite() {
        //     println!("non finite throughput!  eval_diff: {eval_diff}, weight_average: {weight_average}, weight_diff: {weight_diff}");
        // }
        //
        // return (wi_good, throughput_good);


        let f0_f32 = 0.16 * self.reflectance.powi(2);
        let f0 = Vector::splat(f0_f32);

        let f_wo_diff = 1.0 - fresnel_schlick_single(wo.dot_normal().abs(), f0_f32);

        let p = 1.0 / (1.0 + f_wo_diff * 1.05);
        if rng.gen_bool(p as f64) {
            *specular_bounce = self.roughness <= 0.0;
            // Sample VNDF
            let (wi, throughput) = importance_sample_ggx_d_double_sided(rng, wo, f0, self.roughness);
            let throughput = throughput;
            (wi, throughput / p)
        } else {
            // Sample lambertian
            let wi = random_cos_hemisphere_unit_matvector(rng);
            let f_wi_diff = 1.0 - fresnel_schlick_single(wi.dot_normal(), f0_f32);
            let throughput = f_wi_diff * f_wo_diff * self.color * 1.05;
            // let throughput = f_wo_diff * self.color;
            (wi, throughput / (1.0 - p))
        }
    }

    fn pdf(&self, wi: LocalVector, wo: LocalVector) -> Float {
        if wo.dot_normal() < 0.0 {
            return 0.0;
        }

        let spec_pdf = importance_sample_ggx_d_pdf(wi, wo, self.roughness);
        let diff_pdf = wi.dot_normal() / PI;
        // lerp(diff_pdf, spec_pdf, wo.dot_normal())
        lerp(diff_pdf, spec_pdf, 0.5)
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
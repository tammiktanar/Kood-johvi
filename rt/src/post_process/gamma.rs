use serde::{Deserialize, Serialize};

use crate::camera::Render;
use crate::post_process::PostProcessEffect;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct GammaCorrection {
    factor: f32,
}

impl Default for GammaCorrection {
    fn default() -> Self {
        Self {
            factor: 2.2,
        }
    }
}

impl PostProcessEffect for GammaCorrection {
    fn process(&self, render: &mut Render) {
        let gamma_pow = 1.0 / self.factor;
        render.img.pixels_mut().for_each(|px| {
            px[0] = px[0].powf(gamma_pow);
            px[1] = px[1].powf(gamma_pow);
            px[2] = px[2].powf(gamma_pow);
        })
    }
}
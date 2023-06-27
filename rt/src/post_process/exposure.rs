use serde::{Deserialize, Serialize};

use crate::camera::Render;
use crate::post_process::PostProcessEffect;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Exposure {
    factor: f32,
}

impl Default for Exposure {
    fn default() -> Self {
        Self {
            factor: 1.0,
        }
    }
}

impl PostProcessEffect for Exposure {
    fn process(&self, render: &mut Render) {
        render.img.pixels_mut().for_each(|px| {
            px[0] *= self.factor;
            px[1] *= self.factor;
            px[2] *= self.factor;
        })
    }
}
use serde::{Deserialize, Serialize};

use crate::camera::Render;
use crate::post_process::exposure::Exposure;
use crate::post_process::gamma::GammaCorrection;
use crate::post_process::tone_map::Aces;

mod tone_map;
mod gamma;
mod exposure;
mod white_balance;

pub trait PostProcessEffect {
    fn process(&self, render: &mut Render);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PostProcess {
    ToneMap(Aces),
    #[serde(alias = "gamma")]
    GammaCorrection(GammaCorrection),
    Exposure(Exposure),
}

impl PostProcessEffect for PostProcess {
    fn process(&self, render: &mut Render) {
        match self {
            PostProcess::ToneMap(v) => v.process(render),
            PostProcess::GammaCorrection(v) => v.process(render),
            PostProcess::Exposure(v) => v.process(render),
        }
    }
}

pub fn default_post_process() -> Vec<PostProcess> {
    vec![
        PostProcess::ToneMap(Aces::default()),
        PostProcess::GammaCorrection(GammaCorrection::default()),
    ]
}
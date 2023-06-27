use glam::{Mat3, Vec3};
use serde::{Deserialize, Serialize};

use crate::camera::Render;
use crate::post_process::PostProcessEffect;
use crate::Vector;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Aces {}

impl PostProcessEffect for Aces {
    fn process(&self, render: &mut Render) {
        render.img.pixels_mut().for_each(|px| {
            let new = aces_hill(px.0.into());
            px.0 = new.to_array()
        })
    }
}

const ACES_INPUT_MAT: Mat3 = transpose(Mat3::from_cols_array_2d(&[
    [0.59719, 0.35458, 0.04823],
    [0.07600, 0.90834, 0.01566],
    [0.02840, 0.13383, 0.83777],
]));

const ACES_OUTPUT_MAT: Mat3 = transpose(Mat3::from_cols_array_2d(&[
    [1.60475, -0.53108, -0.07367],
    [-0.10208, 1.10813, -0.00605],
    [-0.00327, -0.07276, 1.07602],
]));

fn rrt_and_odt_fit(v: Vector) -> Vector
{
    let a = v * (v + 0.0245786) - 0.000090537;
    let b = v * (0.983729 * v + 0.432_951) + 0.238081;
    a / b
}

fn aces_hill(color: Vector) -> Vector
{
    let color = ACES_INPUT_MAT * color;

    // Apply RRT and ODT
    let color = rrt_and_odt_fit(color);

    let color = ACES_OUTPUT_MAT * color;

    // Clamp to [0, 1]
    color.clamp(Vector::ZERO, Vector::ONE)
}

fn _aces_narkowicz(x: Vector) -> Vector
{
    let a = 2.51;
    let b = 0.03;
    let c = 2.43;
    let d = 0.59;
    let e = 0.14;
    ((x * (a * x + b)) / (x * (c * x + d) + e)).clamp(Vector::ZERO, Vector::ONE)
}

// Copied this from glam to make it a const fn
const fn transpose(mat: Mat3) -> Mat3 {
    Mat3 {
        x_axis: Vec3::new(mat.x_axis.x, mat.y_axis.x, mat.z_axis.x),
        y_axis: Vec3::new(mat.x_axis.y, mat.y_axis.y, mat.z_axis.y),
        z_axis: Vec3::new(mat.x_axis.z, mat.y_axis.z, mat.z_axis.z),
    }
}
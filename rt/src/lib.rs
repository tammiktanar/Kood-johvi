pub mod camera;
pub mod helpers;
pub mod object_like;
pub mod ray;
pub mod scene;
pub mod trace;
pub mod lights;
mod validators;
mod local_vector;
mod material;
mod buildable;
pub mod gui;
mod post_process;

const EPSILON: Float = 0.00001;
const PI: Float = std::f32::consts::PI;

type Float = f32;
type Vector = glam::Vec3A;
type Transform = glam::Affine3A;
type Rotor = glam::Quat;
#![allow(dead_code)]


use rand::Rng;
use rand::rngs::SmallRng;
use rand_distr::StandardNormal;
use thread_priority::ThreadPriority;

use crate::{Float, PI, Vector};
use crate::local_vector::LocalVector;
use crate::object_like::ObjectLike;

pub mod reservoir;
pub mod vec_or_splat;
pub mod interpolate;
pub mod square_cache;

pub fn smoothstep(low: Float, high: Float, t: Float) -> Float {
    let t = (t - low) / (high - low);
    let t = t.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

pub fn rgb_luminance(color: Vector) -> Float {
    const COEFFICIENT: Vector = Vector::new(0.2126, 0.7152, 0.0722);
    COEFFICIENT.dot(color)
}

pub fn rgb_lightness(color: Vector) -> Float {
    let y = rgb_luminance(color);
    if y <= (216.0 / 24389.0) {
        y * (24389.0 / 27.0)
    } else {
        y.powf(1.0 / 3.0) * 116.0 - 16.0
    }
}

pub fn random_unit_vector(rng: &mut SmallRng) -> Vector {
    let x = rng.sample(StandardNormal);
    let y = rng.sample(StandardNormal);
    let z = rng.sample(StandardNormal);

    // vec * 0.99999
    Vector::new(x, y, z).try_normalize().unwrap_or_else(|| random_unit_vector(rng))
}

// pub fn random_unit_vector(rng: &mut SmallRng) -> Vec3 {
//     let theta = rng.gen_range(0.0..=2.0*PI);
//     let z: Float = rng.gen_range(0.0..=1.0);
//     let d = (1.0-z.powi(2)).sqrt();
//     let x = d * theta.cos();
//     let y = d * theta.sin();
//     Vec3::new(x, y, z)
// }

pub fn random_hemisphere_unit_vector(rng: &mut SmallRng, normal: Vector) -> Vector {
    let vec = random_unit_vector(rng);
    if vec.dot(normal) > 0.0 {
        vec
    } else {
        -vec
    }
}

pub fn random_cos_hemisphere_unit_vector(rng: &mut SmallRng, normal: Vector) -> Vector {
    (normal + random_unit_vector(rng)).normalize()
}

pub fn random_cos_hemisphere_unit_matvector(rng: &mut SmallRng) -> LocalVector {
    let u1: Float = rng.gen();
    let u2: Float = rng.gen();

    let x = (u1).sqrt() * (2.0 * PI * u2).cos();
    let y = (u1).sqrt() * (2.0 * PI * u2).sin();
    let z = (1.0 - u1).sqrt();
    // let pdf = z / PI;
    LocalVector::new(x, y, z)
}

pub fn set_process_priority() {
    fn f() {
        thread_priority::set_current_thread_priority(ThreadPriority::Min)
            .expect("couldn't set thread priority")
    }

    rayon::ThreadPoolBuilder::new()
        .start_handler(|_| f())
        .build_global()
        .expect("couldn't set rayon global handler");

    // f()
}

pub fn lerp(a: Float, b: Float, t: Float) -> Float {
    a + (b - a) * t
}

pub fn spherical_to_cartesian(theta: Float, phi: Float) -> Vector {
    let tsin = theta.sin();
    let x = tsin * phi.cos();
    let y = tsin * phi.sin();
    let z = theta.cos();
    Vector::new(x, y, z)
}

pub fn orthonormal_pair(n: Vector) -> (Vector, Vector) {
    let mut tan = n.cross(Vector::Y);
    if tan == Vector::ZERO {
        tan = Vector::X
    }
    tan = tan.normalize();
    let bitan = tan.cross(n);
    (tan, bitan)
}


pub fn flatten_objects(objects: &mut Vec<ObjectLike>) {
    let mut i = 0;
    while i < objects.len() {
        let obj = &objects[i];
        if matches!(obj, ObjectLike::ObjectGroup(_)) {
            let mut obj = match objects.swap_remove(i) {
                ObjectLike::ObjectGroup(v) => v,
                _ => unreachable!(),
            };
            objects.append(&mut obj.objects);
        } else {
            i += 1
        }
    }
}
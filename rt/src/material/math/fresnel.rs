use glam::BVec3A;

use crate::{Float, Vector};
use crate::local_vector::LocalVector;

pub fn fresnel_schlick(v_h: Float, f0: Vector) -> Vector {
    // Schlick
    // let n1: Float = 1.0;
    // let n2 = 1.42;
    // let f0 = ((n1 - n2) / (n1 + n2)).powi(2);
    f0 + (1.0 - f0) * (1.0 - v_h).powi(5)
}

pub fn fresnel_schlick_single(v_h: Float, f0: Float) -> Float {
    f0 + (1.0 - f0) * (1.0 - v_h).powi(5)
}

pub fn calc_f0(n1: Float, n2: Float) -> Float {
    ((n1 - n2) / (n1 + n2)).powi(2)
}

pub fn fresnel(cosi: Float, etai: Float, etat: Float) -> Float {
    // Compute sini using Snell's law
    let sint = etai / etat * (1.0 - cosi * cosi).max(0.0).sqrt();
    // Total internal reflection
    if sint >= 1.0 {
        1.0
    } else {
        let cost = (1.0 - sint * sint).max(0.0).sqrt();
        let cosi = cosi.abs();
        let rs = ((etat * cosi) - (etai * cost)) / ((etat * cosi) + (etai * cost));
        let rp = ((etai * cosi) - (etat * cost)) / ((etai * cosi) + (etat * cost));
        (rs * rs + rp * rp) / 2.0
    }
}

pub fn diffuse_fresnel_correction_factor(n: Vector, n2: Vector) -> Vector
{
    let tir = into_bool_array(n.cmplt(Vector::splat(1.0)));

    const A1: Float = 554.33;
    const B1: Float = 380.7;
    let invdenum = mix(Vector::splat(1.0), (n2 * n2 * A1 - n * B1).recip(), tir);

    const A2: Float = 0.192_115_6;
    const B2: Float = 298.25;
    const C2: Float = 261.38;
    const D2: Float = 138.43;
    let mut num = n * mix(Vector::splat(A2), n * B2 - C2 * n2 + D2, tir);

    const A3: Float = 0.807_884_4;
    const B3: Float = -1.67;
    num += mix(Vector::splat(A3), Vector::splat(B3), tir);

    num * invdenum
}

fn into_bool_array(v: BVec3A) -> [bool; 3] {
    let bitmask = v.bitmask();
    [(bitmask & 1) != 0, (bitmask & 2) != 0, (bitmask & 4) != 0]
}


fn mix(a: Vector, b: Vector, t: [bool; 3]) -> Vector {
    Vector::new(
        if t[0] { b.x } else { a.x },
        if t[1] { b.y } else { a.y },
        if t[2] { b.z } else { a.z },
    )
}

pub fn refract(i: LocalVector, inv_eta: Float) -> Option<LocalVector> {
    let cosi = i.dot_normal();
    let cost2 = 1.0 - inv_eta * inv_eta * (1.0 - cosi * cosi);
    if cost2 > 0.0 {
        Some(LocalVector::from(inv_eta * -*i + (inv_eta * cosi - cost2.abs().sqrt()) * Vector::Z))
    } else {
        None
    }
}

pub fn reflect(vec: Vector, normal: Vector) -> Vector {
    let projected = vec.project_onto_normalized(normal);
    -vec + projected * 2.0
}

pub fn reflect_mat(vec: LocalVector) -> LocalVector {
    LocalVector::new(-vec.x, -vec.y, vec.z)
}

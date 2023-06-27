use rand::prelude::SmallRng;
use rand::Rng;

use crate::{Float, PI, Vector};
use crate::helpers::lerp;
use crate::local_vector::LocalVector;
use crate::material::math::fresnel::{fresnel_schlick, reflect};

pub fn ggx_d(n_dot_h: Float, alpha2: Float) -> Float {
    if alpha2 == 0.0 {
        return 0.0;
    }

    let b = (alpha2 - 1.0) * n_dot_h * n_dot_h + 1.0;
    alpha2 / (PI * b * b)
}

/// Samples a microfacet half vector
pub fn ggx_vndf(wo: LocalVector, alpha: Float, u1: Float, u2: Float) -> Vector
{
    // -- Stretch the view vector so we are sampling as though
    // -- roughness==1
    let v = Vector::new(wo.x * alpha, wo.y * alpha, wo.z).normalize();

    // -- Build an orthonormal basis with v, t1, and t2
    let lensq = v.x * v.x + v.y * v.y;
    let t1 = if lensq > 0.0 {
        Vector::new(-v.y, v.x, 0.0) / lensq.sqrt()
    } else {
        Vector::new(1.0, 0.0, 0.0)
    };
    let t2 = v.cross(t1);

    // -- Choose a point on a disk with each half of the disk weighted
    // -- proportionally to its projection onto direction v
    // let a = 1.0 / (1.0 + v.z);
    // let r = u1.sqrt();
    // let phi = if u2 < a { (u2 / a) * PI } else { PI + (u2 - a) / (1.0 - a) * PI };
    // let p1 = r * phi.cos();
    // let p2 = r * phi.sin() * (if u2 < a { 1.0 } else { v.z });
    let r = u1.sqrt();
    let phi = 2.0 * PI * u2;
    let p1 = r * phi.cos();
    let p2 = r * phi.sin();
    let s = 0.5 * (1.0 + v.z);
    let p2 = lerp((1.0 - p1 * p1).sqrt(), p2, s);


    // -- Calculate the normal in this stretched tangent space
    let n = p1 * t1 + p2 * t2 + (1.0 - p1 * p1 - p2 * p2).max(0.0).sqrt() * v;

    // println!("p1: {:10}, t1: {:12}, p2: {:10}, t2: {:12}, x: {:10}, v: {:12}", p1, t1.to_string(), p2, t2.to_string(), 1.0 - p1 * p1 - p2 * p2, v.to_string());

    // -- unstretch and normalize the normal
    Vector::new(alpha * n.x, alpha * n.y, n.z.max(0.0)).normalize()
}

pub fn importance_sample_ggx_d_double_sided(rng: &mut SmallRng, mut wo: LocalVector, f0: Vector, roughness: Float) -> (LocalVector, Vector) {
    let sign = wo.dot_normal().signum();
    wo.z *= sign;

    let (mut wi, throughput) = importance_sample_ggx_d(rng, wo, f0, roughness);
    wi.z *= sign;

    (wi, throughput)
}

pub fn importance_sample_ggx_d(rng: &mut SmallRng, wo: LocalVector, f0: Vector, roughness: Float) -> (LocalVector, Vector)
{
    debug_assert!(wo.dot_normal() > 0.0);

    let a = roughness * roughness;
    let a2 = a * a;

    let wm = if a == 0.0 {
        Vector::Z
    } else {
        let u1: Float = rng.gen();
        let u2: Float = rng.gen();
        ggx_vndf(wo, a, u1, u2)
    };

    let wi = LocalVector::from(reflect(*wo, wm));

    let throughput = if wi.dot_normal() > 0.0 {
        let f = fresnel_schlick(wi.dot(wm), f0);
        let g2_over_g1 = smith_ggx_g2_over_g1(a2, wi.dot_normal(), wo.dot_normal());

        f * g2_over_g1
    } else {
        Vector::ZERO
    };

    // if !throughput.is_finite() {
    //     eprintln!("wi: {}, wo: {}, throughput: {}, F: {}, G2/G1: {}", *wi, *wo, throughput, f, g2_over_g1);
    // }

    (wi, throughput)
}

pub fn importance_sample_ggx_d_pdf(wi: LocalVector, wo: LocalVector, roughness: Float) -> Float {
    let alpha = roughness * roughness;
    let alpha2 = alpha * alpha;
    let middle = match (*wi + *wo).try_normalize() {
        None => return 0.0,
        Some(v) => LocalVector::from(v),
    };

    if wi.dot_normal() * wo.dot_normal() <= 0.0 {
        return 0.0;
    }

    let g1 = smith_ggx_g1(wi.dot_normal(), alpha2);
    let d = ggx_d(middle.dot_normal(), alpha2);

    (d * g1) / (4.0 * wi.dot_normal().abs())
}

pub fn eval_specular_ggx(wi: LocalVector, wo: LocalVector, f0: Vector, roughness: Float) -> Vector {
    let middle = match (*wi + *wo).try_normalize() {
        None => return Vector::ZERO,
        Some(v) => LocalVector::from(v),
    };

    let n_dot_o = wo.dot_normal();
    let n_dot_i = wi.dot_normal();
    let n_dot_m = middle.dot_normal();
    let m_dot_i = wi.dot(*middle);

    if wi.dot_normal() * wo.dot_normal() <= 0.0 {
        return Vector::ZERO;
    }

    let alpha = roughness * roughness;
    let alpha2 = alpha * alpha;

    let f = fresnel_schlick(m_dot_i, f0);

    let d = ggx_d(n_dot_m, alpha2);

    let g2 = smith_ggx_g2_predivided(n_dot_o, n_dot_i, alpha2);


    // if !res.is_finite() {
    //     eprintln!("a2: {}, f: {}  d: {}  g2: {}, n_dot_i: {}, n_dot_o: {}, n_dot_m: {}", alpha2, f, d, g2, n_dot_i, n_dot_o, n_dot_m);
    // }

    f * (g2 * d) * n_dot_i
}

pub fn smith_ggx_g1(a2: Float, n_dot_2: Float) -> Float
{
    2.0 / ((((a2 * (1.0 - n_dot_2)) + n_dot_2) / n_dot_2).sqrt() + 1.0)
}


pub fn smith_ggx_g2_predivided(n_dot_v: Float, n_dot_l: Float, a2: Float) -> Float
{
    let denom_a = n_dot_v * (a2 + n_dot_l * (n_dot_l - a2 * n_dot_l)).sqrt();
    let denom_b = n_dot_l * (a2 + n_dot_v * (n_dot_v - a2 * n_dot_v)).sqrt();

    0.5 / (denom_a + denom_b)
}

pub fn smith_ggx_g2_over_g1(alpha2: Float, n_dot_l: Float, n_dot_v: Float) -> Float {
    let g1_v = smith_ggx_g1(alpha2, n_dot_v * n_dot_v);
    let g1_l = smith_ggx_g1(alpha2, n_dot_l * n_dot_l);
    g1_l / (g1_v + g1_l - g1_v * g1_l)
}

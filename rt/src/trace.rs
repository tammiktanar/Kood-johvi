use std::sync::RwLock;

use itertools::Itertools;
use ordered_float::OrderedFloat;
use rand::Rng;
use rand::rngs::SmallRng;

use crate::{EPSILON, Float, Vector};
use crate::camera::Camera;
use crate::lights::Emitter;
use crate::local_vector::LocalVectorFrame;
use crate::material::MaterialInterface;
use crate::object_like::hit::Hit;
use crate::object_like::ObjectInterface;
use crate::object_like::shape::ShapeSampler;
use crate::ray::Ray;
use crate::scene::Scene;

pub fn trace(rng: &mut SmallRng, mut ray: Ray, scene: &Scene, camera: &Camera) -> Vector {
    let mut bounces = 0;
    let mut throughput = Vector::ONE;
    let mut radiance = Vector::ZERO;

    let mut specular_bounce = true;
    let mut pdf_bsdf_sa: Float = 0.0;
    // let mut prev_ray = ray;

    // Choose whether we're sampling all lights or just the dirac delta ones
    let lights = if camera.indirect_only {
        &scene.dirac_lights
    } else {
        &scene.lights
    };

    loop {
        if bounces >= camera.bounces {
            if let Some(scalar) = scene.debug {
                static WARNED: RwLock<bool> = RwLock::new(false);
                if !*WARNED.read().unwrap() {
                    *WARNED.write().unwrap() = true;
                    eprintln!("Reached maximum bounce! These pixels will be made blue.  Throughput: {throughput}");
                }
                return Vector::new(0.0, 0.0, scalar);
            }
            break;
        }

        // <><><><><><> FIND HIT <><><><><><>
        let hit = match scene.try_hit(ray) {
            Some(v) => v,
            None => {
                // We hit the sky
                radiance += throughput * scene.sky;
                break;
            }
        };
        let obj = hit.object;

        // Failsafe for cases where inifinite shapes like planes can have an exit hit at inifinity
        if hit.t == Float::INFINITY {
            break;
        }


        // <><><><><><> EMISSIVE DIRECT HIT <><><><><><>
        if let Some(emission) = obj.material.emission() {
            // Object is emissive
            // Only apply it if coming from a specular bounce or we're on the first bounce
            let mis_bsdf = if camera.indirect_only || specular_bounce || !obj.supports_nee() {
                1.0
            } else {
                // 0.0
                let pdf_nee_a = obj.sample_pdf(ray.origin, hit.point, hit.normal) / lights.len() as Float;
                let pdf_nee_sa = pdf_nee_a * hit.t.powi(2);
                pdf_bsdf_sa.powi(2) / (pdf_bsdf_sa.powi(2) + pdf_nee_sa.powi(2))
            };
            radiance += throughput * emission * mis_bsdf;
        }
        specular_bounce = false;


        // <><><><><><> BRDF SAMPLING <><><><><><>

        // Transform ray direction to tangent space
        let frame = LocalVectorFrame::new(hit.normal);
        let wo = frame.to_mat(-ray.dir);
        let (wi, mat_throughput) = obj.material.sample(rng, wo, &mut specular_bounce);
        pdf_bsdf_sa = obj.material.pdf(wi, wo);


        let light_direction = frame.to_world(wi);

        let light_ray = Ray::new(
            hit.point + hit.normal * EPSILON * wi.z.signum(),
            // hit.point,
            light_direction,
        );

        // --- Applying mat_throughput is deferred until end ---

        // println!("ray: {} {} => hit:{} {} => light: {} {}", ray.origin, ray.dir, hit.point, hit.normal, light_ray.origin, light_direction);


        // if wo.dot_normal() <= 0.0 || wi.dot_normal() <= 0.0 {
        //     // println!("prev: {} {} => ray: {} {} => hit: {} {}, wi: {}, wo: {}, shape: {:?}",
        //     //          prev_ray.origin, prev_ray.dir, ray.origin, ray.dir,
        //     //          hit.point, hit.normal, *wi, *wo, obj.shape
        //     // );
        //
        //     let offset = 1;
        //     if bounces <= offset {
        //         return Vector::new(10000.0, 0.0, 0.0);
        //     } else if bounces == offset + 1 {
        //         return Vector::new(0.0, 10000.0, 0.0);
        //     } else {
        //         return Vector::new(0.0, 0.0, 10000.0);
        //     }
        // }
        // prev_ray = ray;


        // <><><><><><> DIRECT LIGHTING <><><><><><>

        // Do NEE direct lighting
        // pick a random light source at random
        if !lights.is_empty() {

            // Pick light
            let p: Float = rng.gen();
            let mut running_total = 0.0;
            let (light, choose_weight) = lights.iter()
                .find(|(_, weight)| {
                    running_total += weight;
                    p < running_total
                })
                .unwrap_or_else(|| lights.last().unwrap());

            if !specular_bounce || light.is_dirac_delta() {
                let emission = light.estimate_throughput(rng, scene, &hit, frame, wo, *choose_weight);
                // println!("light_emission: {:?}", emission);

                if let Some(emission) = emission {
                    radiance += emission * throughput;
                }

                if !radiance.is_finite() || radiance.is_negative_bitmask() > 0 {
                    // return Vector::new(1000000.0, 0.0, 0.0);
                }
            }
        }


        // <><><><><><> FINALIZE <><><><><><>
        throughput *= mat_throughput;

        throughput = throughput.min(Vector::splat(1.0));

        if !throughput.is_finite() || radiance.is_negative_bitmask() > 0 {
            if let Some(scalar) = scene.debug {
                static WARNED: RwLock<bool> = RwLock::new(false);
                if !*WARNED.read().unwrap() {
                    *WARNED.write().unwrap() = true;
                    eprintln!("ERROR: Something went wrong while ray tracing. The pixels where the error happened will be made red.");
                }

                radiance = Vector::new(scalar, 0.0, 0.0);
            } else {
                radiance = Vector::splat(0.0);
            }
            break;
        }

        // Russian roulette
        if bounces > 3 {
            let p = throughput.max_element().min(0.95);
            assert!((0.0..=1.0).contains(&p), "p: {}, throughput: {}, tint: {}, dot: {}", p, throughput, mat_throughput, light_direction.dot(hit.normal));
            if rng.gen_bool(p as f64) {
                throughput *= p.recip();
            } else {
                break;
            }
        } else if throughput == Vector::ZERO {
            // break;
        }

        ray = light_ray;
        bounces += 1;
    }

    if let Some(clamp) = camera.clamping {
        let len = radiance.length();
        if len > clamp {
            radiance = (radiance / len) * clamp;
        }
    }

    radiance
}

impl Scene {
    pub(crate) fn try_hit(&self, ray: Ray) -> Option<Hit> {
        let bvh_ray = bvh::ray::Ray::new(ray.origin.into(), ray.dir.into());

        let bvh_hit_candidates = if !self.bvh.nodes.is_empty() {
            self.bvh.traverse(&bvh_ray, &self.objects)
        } else {
            vec![]
        };

        let infinite_candidates = self.infinite_objects.iter();

        bvh_hit_candidates.into_iter().chain(infinite_candidates)
            .filter_map(|obj| obj.try_hit(ray, 0.0))
            .flatten()
            .sorted_unstable_by_key(|v| OrderedFloat(v.t))
            // .inspect(|candidate| eprintln!("t: {}, shape: {:?}", candidate.t, candidate.object.shape))
            .find(|v| v.t > 0.0)
            .map(|v| v.finalize(ray, 1.0))
    }
}

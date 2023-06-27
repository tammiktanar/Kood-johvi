use std::cmp::Reverse;
use std::collections::HashMap;
use std::mem;
use std::sync::Arc;
use std::sync::mpsc::Sender;

use anyhow::Result;
use bvh::aabb::Bounded;
use bvh::bvh::BVH;
use either::Either;
use indexmap::IndexMap;
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};

use crate::{Float, Vector};
use crate::buildable::{Buildable, PostBuildable};
use crate::camera::{Camera, CameraBuilder, Render};
use crate::gui::LockedSharedImage;
use crate::helpers::flatten_objects;
use crate::lights::{Emitter, Light, LightBuilder};
use crate::material::{Material, MaterialBuilder};
use crate::object_like::{ObjectLike, ObjectLikeBuilder};
use crate::post_process::{PostProcess, PostProcessEffect};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SceneBuilder {
    pub name: Option<String>,

    pub cameras: IndexMap<String, CameraBuilder>,

    pub objects: Vec<ObjectLikeBuilder>,

    #[serde(default, alias = "templates")]
    pub object_templates: HashMap<String, ObjectLikeBuilder>,

    #[serde(default)]
    pub lights: Vec<LightBuilder>,

    #[serde(default, deserialize_with = "crate::validators::de_vec3_gte_0")]
    pub sky: Vector,

    #[serde(default)]
    pub materials: HashMap<String, MaterialBuilder>,

    #[serde(alias = "post_processing", default = "crate::post_process::default_post_process")]
    pub post_process: Vec<PostProcess>,

    #[serde(default = "air", deserialize_with = "crate::validators::de_float_gte_1")]
    pub ior: Float,

    #[serde(skip)]
    pub compiled_materials: HashMap<String, Arc<Material>>,

    #[serde(skip)]
    finalized: bool,

    // Just some debug stuff
    #[serde(default, with = "either::serde_untagged_optional", rename = "visualize_brdf")]
    visualize_brdf: Option<Either<(usize, String), (usize, MaterialBuilder)>>,
    #[serde(default)]
    debug: Option<Float>,
}

fn air() -> Float {
    1.00029
}

impl SceneBuilder {
    pub fn build(&mut self) -> Result<Scene> {
        if !self.finalized {
            // self.visualize_brdf()?;

            self.compiled_materials = self.materials.iter()
                .map(|(k, v)| v.build(self).map(|mat| (k.clone(), Arc::new(mat))))
                .collect::<Result<_>>()?;
        }

        let cameras = self.cameras.iter()
            .map(|(name, v)|
                v.build(self)
                    .map(|cam| (name.clone(), cam))
            ).collect::<Result<_>>()?;

        let lights: Vec<_> = self.lights.iter()
            .map(|v| v.build(self)
                .map(|light| {
                    let weight = light.get_cached_weight();
                    (light, weight)
                })
            )
            .collect::<Result<_>>()?;


        let mut objects: Vec<ObjectLike> = self.objects.iter()
            .map(|v| v.build(self))
            .collect::<Result<_>>()?;

        // Flatten groups
        flatten_objects(&mut objects);

        // Separate out infinite objects like planes and also remove objects that can't possibly be hit
        let (infinite_objects, mut objects): (Vec<_>, Vec<_>) = objects.into_iter()
            .filter(|obj| {
                let aabb = obj.aabb();
                /*aabb.min.x.is_nan() || */!aabb.is_empty()
            })
            .partition(|obj| obj.aabb().min.x.is_nan());

        let bvh = if objects.is_empty() {
            BVH { nodes: vec![] }
        } else {
            BVH::build(&mut objects)
        };

        let mut scene = Scene {
            name: self.name.clone(),
            cameras,
            objects,
            infinite_objects,
            dirac_lights: lights.clone(),
            lights,
            post_process: self.post_process.clone(),
            sky: self.sky,
            debug: self.debug,
            bvh,
        };

        scene.post_build(); // This only sets up area lights for now

        Ok(scene)
    }
}

pub struct Scene {
    pub name: Option<String>,
    pub cameras: Vec<(String, Camera)>,
    pub objects: Vec<ObjectLike>,
    pub infinite_objects: Vec<ObjectLike>,
    pub lights: Vec<(Light, Float)>,
    pub dirac_lights: Vec<(Light, Float)>,
    pub post_process: Vec<PostProcess>,
    pub sky: Vector,
    pub debug: Option<Float>,

    pub bvh: BVH,
}

impl Scene {
    pub fn render_all(mut self, tx: Option<Sender<(LockedSharedImage, Sender<()>)>>) -> impl Iterator<Item=Render> {
        let cameras = mem::take(&mut self.cameras);

        cameras.into_iter()
            .enumerate()
            .inspect(|(i, (name, _))| {
                eprintln!("Rendering {i}: {name}")
            })
            .map(|(_, cam)| cam)
            .map(move |(name, cam)| {
                let mut render = cam.render(name, &self, tx.clone());
                self.post_process.iter()
                    .for_each(|pp| pp.process(&mut render));
                render
            })
    }

    /// Just a cringe AF function, don't mind me
    pub fn post_build(&mut self) {
        // Add area lights
        for obj in self.objects.iter() {
            obj.post_build(&mut self.lights)
        }

        // Set up light weights
        let summed_weights = self.lights.iter()
            .map(|(_, weight)| weight)
            .sum::<Float>();

        self.lights.iter_mut()
            .for_each(|(_, weight)| *weight /= summed_weights);

        // Now the dirac delta lights
        let summed_weights = self.dirac_lights.iter()
            .map(|(_, weight)| weight)
            .sum::<Float>();

        self.dirac_lights.iter_mut()
            .for_each(|(_, weight)| *weight /= summed_weights);

        self.lights.sort_unstable_by_key(|(_, weight)| Reverse(OrderedFloat(*weight)));
    }

    pub fn apply_post_process(&self, render: &mut Render) {
        self.post_process.iter()
            .for_each(|pp| pp.process(render))
    }
}


// This is just old code for debugging

// fn weight_to_color(weight: Float) -> Vector {
//     // let weight = weight.recip() / PI;
//     Vector::new(
//         smoothstep(1.0, 0.0, weight),
//         smoothstep(0.0, 1.0, weight) * smoothstep(2.0, 1.0, weight),
//         smoothstep(1.0, 2.0, weight),
//     )
// }

// impl SceneBuilder {
//     pub fn visualize_brdf(&mut self) -> Result<()> {
//         let (&samples, material_builder) = match &self.visualize_brdf {
//             Some(Either::Left((s, name))) => {
//                 let mat = self.materials.get(name).cloned()
//                     .with_context(|| format!("couldn't find material named: \"{name}\""))?;
//                 (s, mat)
//             }
//             Some(Either::Right((s, mat))) => (s, mat.clone()),
//             None => return Ok(()),
//         };
//
//         let material = material_builder.build(self)?;
//
//         // let mut rng = SmallRng::from_entropy();
//         let mut rng = SmallRng::from_entropy();
//
//         let wo = LocalVector::from(Vector::new(-1.0, 0.0, 1.0).normalize());
//
//         // Make a ball for each sample
//         (0..samples)
//             .into_iter()
//             .map(|_| material.sample(&mut rng, wo, &mut true))
//             .filter(|(_wi, throughput)| *throughput != Vector::ZERO)
//             // .map(|(wi, throughput)| (wi.xzy(), throughput.to_array().into_iter().sum::<Float>() / 3.0))
//             // .map(|(wi, _)| (wi.xzy(), material.eval(wi, wo).to_array().into_iter().sum::<Float>() / 3.0))
//             .map(|(wi, _)| (wi.xzy(), material.pdf(wi, wo)))
//             // .map(|(wi, _)| (wi.xzy(), (material.eval(wi, wo) / material.pdf(wi, wo)).to_array().into_iter().sum::<Float>() / 3.0))
//             .map(|(dir, weight)| {
//                 assert!(weight >= 0.0);
//                 ObjectLikeBuilder::Object(ObjectBuilder {
//                     shape: Shape::Sphere(Sphere::default()),
//                     transforms: Some(Either::Left(TransformParams {
//                         translate: dir * weight,
//                         scale: Either::Left(0.05),
//                         ..Default::default()
//                     })),
//                     material: Either::Right(MaterialBuilder::Diffuse(Diffuse {
//                         emission: None,
//                         color: weight_to_color(weight),
//                     })),
//                 }.into())
//             })
//             .for_each(|object| self.objects.push(object));
//
//         // Make a small pink ball to show hit point
//         self.objects.push(ObjectLikeBuilder::Object(ObjectBuilder {
//             shape: Shape::Sphere(Sphere::default()),
//             transforms: Some(Either::Left(TransformParams {
//                 scale: Either::Left(0.05),
//                 ..Default::default()
//             })),
//             material: Either::Right(MaterialBuilder::Diffuse(Diffuse {
//                 emission: None,
//                 color: Vector::new(1.0, 0.078, 0.576),
//             })),
//         }.into()));
//
//         // Make a base ball
//         self.objects.push(ObjectLikeBuilder::Object(ObjectBuilder {
//             shape: Shape::Sphere(Sphere::default()),
//             transforms: Some(Either::Left(TransformParams {
//                 translate: [0.0, -1.0, 0.0].into(),
//                 // scale: [1.0, 0.5, 1.0].into(),
//                 // rotate: [0.0, 0.0, 45.0].into(),
//                 ..Default::default()
//             })),
//             material: Either::Right(material_builder),
//         }.into()));
//
//         Ok(())
//     }
// }
use std::ops::Neg;
use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::Sender;
use std::time::{Duration, Instant};

use anyhow::Result;
use image::{Rgb32FImage, RgbaImage};
use rand::{Rng, SeedableRng, thread_rng};
use rand::rngs::SmallRng;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{Float, PI, Vector};
use crate::buildable::Buildable;
use crate::gui::{LockedSharedImage, SharedImage};
use crate::ray::Ray;
use crate::scene::{Scene, SceneBuilder};
use crate::trace::trace;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct CameraBuilder {
    pub pos: Vector,
    pub look_at: Vector,
    pub up: Vector,
    #[serde(deserialize_with = "crate::validators::de_u32_gt_0")]
    pub width: u32,
    #[serde(deserialize_with = "crate::validators::de_u32_gt_0")]
    pub height: u32,
    #[serde(deserialize_with = "crate::validators::de_float_fov")]
    pub fov: Float,
    #[serde(alias = "focus_distance", deserialize_with = "crate::validators::de_float_gt_0")]
    pub depth: Float,
    #[serde(deserialize_with = "crate::validators::de_float_gte_0")]
    pub aperature: Float,

    #[serde(deserialize_with = "crate::validators::de_u32_gt_0")]
    pub samples: u32,
    pub bounces: u32,

    #[serde(rename = "indirect_only")]
    indirect_only: bool,
    clamping: Option<Float>,
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self {
            pos: Vector::new(0.0, 5.0, 10.0),
            look_at: Vector::ZERO,
            up: Vector::Y,
            width: 1280,
            height: 720,
            fov: 60.0,
            depth: 1.0,
            aperature: 0.0,

            samples: 50,
            bounces: 50,

            indirect_only: false,
            clamping: None,
        }
    }
}

impl Buildable for CameraBuilder {
    type Target = Camera;

    fn build(&self, _scene: &SceneBuilder) -> Result<Camera> {
        let direction = (self.look_at - self.pos)
            .try_normalize()
            .unwrap_or(Vector::X);

        let up = self.up.try_normalize().unwrap_or(Vector::Y);

        // to rad
        // let fov = params.fov / 180.0 * PI;
        // let dist = params.height as Float / (fov / 2.0).tan();

        let depth = direction * self.depth;
        let mut u_norm = direction.cross(up);
        if u_norm == Vector::ZERO {
            u_norm = Vector::X
        }
        u_norm = u_norm.normalize();
        let v_norm = direction.cross(u_norm).neg();

        let fov = self.fov / 180.0 * PI;
        let mul = self.depth * (fov / 2.0).tan();
        let u_focal = 2.0 * u_norm * mul / self.height as Float;
        let v_focal = -2.0 * v_norm * mul / self.height as Float;

        let cam = Camera {
            pos: self.pos,
            width: self.width,
            height: self.height,

            samples: self.samples,
            bounces: self.bounces,

            depth_vec: depth,
            aperature: self.aperature,
            u_focal,
            v_focal,
            u_norm,
            v_norm,

            indirect_only: self.indirect_only,
            clamping: self.clamping,
        };

        Ok(cam)
    }
}

#[derive(Debug, Clone)]
pub struct Camera {
    pos: Vector,
    width: u32,
    height: u32,

    samples: u32,
    pub bounces: u32,

    depth_vec: Vector,
    aperature: Float,
    u_focal: Vector,
    v_focal: Vector,
    u_norm: Vector,
    v_norm: Vector,

    pub indirect_only: bool,
    pub clamping: Option<Float>,
}

pub struct Render {
    pub name: String,
    pub img: Rgb32FImage,
}

impl Camera {
    pub fn render(&self, name: String, scene: &Scene, tx: Option<Sender<(LockedSharedImage, Sender<()>)>>) -> Render {
        let mut img = Rgb32FImage::new(self.width, self.height);
        let img_sdr = RgbaImage::new(self.width, self.height);

        let shared_img = tx.map(|tx| {
            let shared = Arc::new(Mutex::new(SharedImage::new(img_sdr.clone())));

            let (ready_tx, ready_rx) = mpsc::channel();
            tx.send((shared.clone(), ready_tx))
                .expect("error while sending shared image");

            ready_rx.recv().expect("something went wrong while receiving ready signal");

            (shared, ready_rx)
        });


        let start = Instant::now();

        let mut img_raw = img.into_raw();
        let mut img_sdr_raw = img_sdr.into_raw();

        let mut samples_per_chunk = 1;
        let mut sample = 1;

        while sample <= self.samples {
            let mul_old = sample as Float / (sample + samples_per_chunk) as Float;
            let mul_new = samples_per_chunk as Float / (sample + samples_per_chunk) as Float;

            let chunk_start = Instant::now();

            img_raw.par_chunks_exact_mut(3)
                .zip(img_sdr_raw.par_chunks_exact_mut(4))
                .enumerate()
                .map(|(i, rgb)| (i as u32, rgb))
                .map(|(i, rgb)| (i % self.width, i / self.width, rgb))
                .for_each_init(
                    || SmallRng::from_rng(thread_rng()).unwrap(),
                    |rng, (x, y, (rgb, rgb_sdr))| {
                        rgb[0] *= mul_old;
                        rgb[1] *= mul_old;
                        rgb[2] *= mul_old;

                        for _ in 0..samples_per_chunk {
                            let ray = self.ray_to(rng, x, y);
                            let res = trace(rng, ray, scene, self);

                            rgb[0] += res.x * mul_new;
                            rgb[1] += res.y * mul_new;
                            rgb[2] += res.z * mul_new;
                        }

                        rgb_sdr[0] = (rgb[0].powf(1.0 / 2.2).clamp(0.0, 1.0) * 255.0) as u8;
                        rgb_sdr[1] = (rgb[1].powf(1.0 / 2.2).clamp(0.0, 1.0) * 255.0) as u8;
                        rgb_sdr[2] = (rgb[2].powf(1.0 / 2.2).clamp(0.0, 1.0) * 255.0) as u8;
                    });

            sample += samples_per_chunk;
            let samples_remaining = self.samples + 1 - sample;
            eprint!("\rSamples remaining: {samples_remaining:<8}");

            // Dynamically adjust how many samples we get before sending over the image to show on screen
            const DESIRED_TIME: Duration = Duration::from_millis(16);
            let chunk_time = chunk_start.elapsed();
            let chunk_adjustment = DESIRED_TIME.as_secs_f32() / chunk_time.as_secs_f32();
            samples_per_chunk = (samples_per_chunk as f32 * chunk_adjustment) as u32;
            samples_per_chunk = samples_per_chunk.min(samples_remaining).max(1);

            if let Some((shared_img, stop_rx)) = &shared_img {
                shared_img.lock().unwrap().update_image(&img_sdr_raw);
                if let Ok(()) = stop_rx.try_recv() {
                    eprint!("\nStopping early!");
                    break;
                }
            }
        }

        img = Rgb32FImage::from_raw(self.width, self.height, img_raw).unwrap();

        eprintln!("\nFinished render in: {:?}", start.elapsed());

        Render {
            name,
            img,
        }
    }

    fn ray_to(&self, rng: &mut SmallRng, x: u32, y: u32) -> Ray {
        let x = x as Float - (self.width as Float + 1.0) / 2.0;
        let y = y as Float - (self.height as Float + 1.0) / 2.0;

        // Depth of field stuff
        let u1: Float = rng.gen();
        let u2: Float = rng.gen();
        let dof_u = (u1).sqrt() * (2.0 * PI * u2).cos();
        let dof_v = (u1).sqrt() * (2.0 * PI * u2).sin();

        let origin_jitter = self.u_norm * self.aperature * dof_u +
            self.v_norm * self.aperature * dof_v;
        let origin = self.pos + origin_jitter;


        let dir_jitter = self.u_focal * rng.gen_range(-0.5..0.5) + self.v_focal * rng.gen_range(-0.5..=0.5);
        let direction = (self.depth_vec + self.u_focal * x + self.v_focal * y + dir_jitter - origin_jitter).normalize();

        Ray {
            origin,
            dir: direction,
        }
    }
}

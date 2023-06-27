use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender, TryRecvError};

use eframe::{App, CreationContext, Frame, NativeOptions, run_native};
use eframe::egui::{CentralPanel, Color32, containers, Context, Style, TextureId, Vec2};
use eframe::egui_wgpu::{RenderState, WgpuConfiguration};
use eframe::wgpu;
use eframe::wgpu::{PowerPreference, PresentMode};
use image::RgbaImage;

pub type LockedSharedImage = Arc<Mutex<SharedImage>>;

pub struct SharedImage {
    image: RgbaImage,
    sample: usize,
    eframe_ctx: Option<Context>,
}

impl SharedImage {
    pub fn new(image: RgbaImage) -> Self {
        Self {
            image,
            sample: 0,
            eframe_ctx: None,
        }
    }

    pub fn update_image(&mut self, image: &[u8]) {
        if let Some(ctx) = &self.eframe_ctx {
            ctx.request_repaint()
        }

        self.image.clone_from_slice(image);
        self.sample += 1;
    }
}

pub struct GUI {
    /// Channel to receive a shared image,
    /// and a sender to let the renderer know we're ready to display it.
    rx: Receiver<(LockedSharedImage, Sender<()>)>,

    texture_data: Option<TextureData>,

}

struct TextureData {
    img_lock: LockedSharedImage,
    texture: wgpu::Texture,
    texture_id: TextureId,
    texture_size: wgpu::Extent3d,
    size_vec2: Vec2,
    control_tx: Sender<()>,
    sample: usize,
}

impl TextureData {
    fn new(ctx: &Context, state: &RenderState, img_lock: LockedSharedImage, control_tx: Sender<()>) -> Self {
        let mut img = img_lock.lock().unwrap();

        img.eframe_ctx = Some(ctx.clone());

        let texture_size = wgpu::Extent3d {
            width: img.image.width(),
            height: img.image.height(),
            depth_or_array_layers: 1,
        };

        drop(img);

        let texture = state.device.create_texture(&wgpu::TextureDescriptor {
            size: texture_size,
            ..TEX_DESC
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let texture_id = state.renderer.write().register_native_texture(&state.device, &view, Default::default());

        let size_vec2 = Vec2::new(texture_size.width as f32, texture_size.height as f32);

        Self {
            img_lock,
            texture,
            texture_id,
            texture_size,
            size_vec2,
            control_tx,
            sample: 0,
        }
    }

    fn update_texture(&mut self, state: &RenderState) {
        let img = &self.img_lock.lock().unwrap();

        if self.sample == img.sample {
            return;
        } else {
            self.sample += 1;
        }

        // let (x, y) = (200, 100);
        // let i = (self.texture_size.width as usize * y + x) * 4;
        // let slc: &[u8] = &img.image;
        // let slc = &slc[i..i+16];
        // eprintln!("{:?}", slc);


        state.queue.write_texture(
            // Tells wgpu where to copy the pixel data
            wgpu::ImageCopyTexture {
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            // The actual pixel data
            &img.image,
            // The layout of the texture
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * self.texture_size.width),
                rows_per_image: std::num::NonZeroU32::new(self.texture_size.height),
            },
            self.texture_size,
        );
    }
}

const TEX_DESC: wgpu::TextureDescriptor = wgpu::TextureDescriptor {
    label: None,
    size: wgpu::Extent3d {
        width: 0,
        height: 0,
        depth_or_array_layers: 0,
    },
    mip_level_count: 1,
    sample_count: 1,
    dimension: wgpu::TextureDimension::D2,
    format: wgpu::TextureFormat::Rgba8UnormSrgb,
    usage: wgpu::TextureUsages::TEXTURE_BINDING.union(wgpu::TextureUsages::COPY_DST),
};

impl GUI {
    pub fn new(_cc: &CreationContext, rx: Receiver<(LockedSharedImage, Sender<()>)>) -> Self {
        Self {
            rx,
            texture_data: None,
        }
    }
}


pub fn start_gui(rx: Receiver<(LockedSharedImage, Sender<()>)>) {
    let options = NativeOptions {
        resizable: false,
        initial_window_size: Some(Vec2::splat(200.0)),
        wgpu_options: WgpuConfiguration {
            power_preference: PowerPreference::HighPerformance,
            present_mode: PresentMode::AutoVsync,
            ..Default::default()
        },
        ..Default::default()
    };


    run_native("Jank Tracer", options, Box::new(|cc| {
        Box::new(GUI::new(cc, rx))
    }));
}

impl App for GUI {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        // let signal = None;

        match self.rx.try_recv() {
            Ok((img_lock, ready_tx)) => {
                let state = frame.wgpu_render_state().unwrap();
                let data = TextureData::new(ctx, state, img_lock, ready_tx.clone());

                frame.set_window_size(data.size_vec2);

                self.texture_data.replace(data);

                // Signal that we're ready to receive data
                ready_tx.send(()).expect("something went wrong while sending ready signal");
                // let signal
            }
            Err(TryRecvError::Disconnected) => frame.close(),
            _ => {}
        }

        let state = frame.wgpu_render_state().unwrap();
        CentralPanel::default().frame(containers::Frame::default()).show(ctx, |ui| {
            if let Some(data) = &mut self.texture_data {
                data.update_texture(state);
                ui.image(data.texture_id, data.size_vec2);
            }
        });

        let frame = containers::Frame::central_panel(&Style::default())
            .fill(Color32::TRANSPARENT);

        CentralPanel::default().frame(frame).show(ctx, |ui| {
            if let Some(data) = &self.texture_data {
                let response = ui.button("Double click to stop early");

                if response.double_clicked() {
                    data.control_tx.send(()).ok();
                    self.texture_data.take();
                    ctx.request_repaint();
                };
            } else {
                ui.centered_and_justified(|ui| {
                    ui.heading("Waiting for rendering to start...")
                });
                ctx.request_repaint();
            }
        });
        ctx.request_repaint();
    }
}

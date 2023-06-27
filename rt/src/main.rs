use std::{panic, thread};
use std::fs::File;
use std::path::Path;
use std::sync::mpsc;

use anyhow::{Context, Result};
use image::{ColorType, DynamicImage, EncodableLayout, ImageEncoder};
use image::codecs::openexr::OpenExrEncoder;
use image::codecs::pnm::{PnmEncoder, PnmSubtype, SampleEncoding};
use ron::extensions::Extensions;

use jank_tracer::camera::Render;
use jank_tracer::gui::start_gui;
use jank_tracer::helpers::set_process_priority;
use jank_tracer::scene::SceneBuilder;

use crate::args::{ARGS, OutputFormat};

mod args;

fn main() -> Result<()> {
    set_process_priority();


    let scene_file = File::open(&ARGS.scene)
        .context("failed to open scene file")?;

    let options = ron::options::Options::default()
        .with_default_extension(Extensions::IMPLICIT_SOME)
        .with_default_extension(Extensions::UNWRAP_NEWTYPES)
        .with_default_extension(Extensions::UNWRAP_VARIANT_NEWTYPES);

    let mut scene: SceneBuilder = options.from_reader(scene_file)
        .context("failed to decode scene")?;
    let scene = scene.build()?;

    if let Some(path) = &ARGS.post_process {
        // Do post processing and exit
        let mut render = import_render(path)?;
        scene.apply_post_process(&mut render);
        export(&render.name.clone(), render)?;
        return Ok(());
    }

    let scene_name = scene.name.clone()
        .unwrap_or("render".to_string());

    let (tx, rx) = if !ARGS.no_preview {
        let (tx, rx) = mpsc::channel();
        (Some(tx), Some(rx))
    } else {
        (None, None)
    };


    let join = thread::spawn(|| {
        scene.render_all(tx)
            .map(move |render| (format!("{}-{}", scene_name, render.name), render))
            .map(|(file_name, render)| export(&file_name, render))
            .filter_map(|res| res.err())
            .for_each(|err| eprintln!("{err}"));
    });

    if ARGS.no_preview {
        match join.join() {
            Ok(v) => v,
            Err(err) => panic::resume_unwind(err),
        }
    } else {
        start_gui(rx.unwrap())
    };

    Ok(())
}

fn export(file_name: &str, rend: Render) -> Result<()> {
    for v in rend.img.iter().copied() {
        (v.is_finite()).then_some(())
            .with_context(|| format!("rendered image \"{file_name}\" included invalid value: {v}"))?;
    }

    let mut output = File::create(format!("{}.{}", file_name, ARGS.output_format))
        .context("couldn't open file")?;
    // let output = std::io::stdout();

    match ARGS.output_format {
        OutputFormat::Png => {
            let img = DynamicImage::ImageRgb32F(rend.img).into_rgb8();
            img.write_to(
                &mut output,
                image::ImageOutputFormat::Png,
            ).context("failed to encode")?;
        }
        OutputFormat::Ppm => {
            let img = DynamicImage::ImageRgb32F(rend.img).into_rgb8();
            let (width, height) = img.dimensions();
            PnmEncoder::new(output)
                .with_subtype(PnmSubtype::Pixmap(SampleEncoding::Ascii))
                .encode(
                    img.into_flat_samples().as_slice(),
                    width,
                    height,
                    ColorType::Rgb8,
                ).context("failed to encode")?;
        }
        OutputFormat::Exr => {
            OpenExrEncoder::new(output)
                .write_image(
                    rend.img.as_bytes(),
                    rend.img.width(),
                    rend.img.height(),
                    ColorType::Rgb32F,
                ).context("failed to encode")?;
        }
    }

    Ok(())
}

fn import_render(path: &Path) -> Result<Render> {
    let dyn_image = image::open(path)?;
    let image = dyn_image.into_rgb32f();

    let mut name = path.file_stem()
        .expect("how did image::open() open this when it's not a file?")
        .to_string_lossy()
        .into_owned();

    name.push_str("-pp");

    Ok(Render {
        name,
        img: image,
    })
}
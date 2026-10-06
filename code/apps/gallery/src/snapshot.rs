use iced::advanced::renderer::Headless;
use iced::advanced::renderer::Style;
use iced::theme::Base;
use iced::{Font, Pixels, Size};
use iced_futures::backend::default::Executor;
use iced_renderer::Renderer;
use iced_runtime::user_interface::{Cache, UserInterface};

const LOGICAL_SIZE: Size<f32> = Size {
    width: 1100.0,
    height: 720.0,
};

#[derive(Clone, Copy)]
struct SnapshotScale {
    name: &'static str,
    factor: f32,
}

impl SnapshotScale {
    const ALL: [Self; 2] = [
        Self {
            name: "100",
            factor: 1.0,
        },
        Self {
            name: "150",
            factor: 1.5,
        },
    ];

    fn physical_size(self) -> Size<u32> {
        Size::new(
            (LOGICAL_SIZE.width * self.factor) as u32,
            (LOGICAL_SIZE.height * self.factor) as u32,
        )
    }
}

fn io_error(error: std::io::Error) -> iced::Error {
    iced::Error::WindowCreationFailed(Box::new(error))
}

pub(crate) fn compare() -> iced::Result {
    let mut failed = false;
    for scale in SnapshotScale::ALL {
        let current_dir = std::path::Path::new("visual-baselines/.current").join(scale.name);
        render_to(&current_dir, scale)?;

        for page in Page::ALL {
            let baseline = std::path::Path::new("visual-baselines")
                .join(scale.name)
                .join(format!("{}.png", page.file_name()));
            let current = current_dir.join(format!("{}.png", page.file_name()));
            let result = compare_png(&baseline, &current).map_err(io_error)?;
            println!(
                "{} @ {}%: {} differing pixels ({:.4}% max channel error)",
                page.file_name(),
                scale.name,
                result.different_pixels,
                result.max_channel_error
            );
            failed |= result.different_pixels > 0;
        }
    }
    if failed {
        return Err(io_error(std::io::Error::other(
            "visual snapshot comparison failed",
        )));
    }
    Ok(())
}

pub(crate) fn run() -> iced::Result {
    for scale in SnapshotScale::ALL {
        let output_dir = std::path::Path::new("visual-baselines").join(scale.name);
        render_to(&output_dir, scale)?;
    }
    Ok(())
}

fn render_to(output_dir: &std::path::Path, scale: SnapshotScale) -> iced::Result {
    let executor = Executor::new().map_err(io_error)?;
    executor.block_on(async {
        let mut renderer = Renderer::new(Font::DEFAULT, Pixels(16.0), Some("tiny-skia"))
            .await
            .ok_or(iced::Error::ExecutorCreationFailed(std::io::Error::other(
                "tiny-skia headless renderer is unavailable",
            )))?;

        std::fs::create_dir_all(output_dir).map_err(io_error)?;

        for page in Page::ALL {
            let mut gallery = Gallery::new().0;
            gallery.page = page.into();
            gallery.page_anim = iced::animation::Animation::new(1.0);

            let mut interface =
                UserInterface::build(gallery.view(), LOGICAL_SIZE, Cache::new(), &mut renderer);
            interface.draw(
                &mut renderer,
                &gallery.theme,
                &Style {
                    text_color: gallery.theme.base().text_color,
                },
                iced::mouse::Cursor::Unavailable,
            );

            let pixels = renderer.screenshot(
                scale.physical_size(),
                scale.factor,
                gallery.theme.base().background_color,
            );
            let path = output_dir.join(format!("{}.png", page.file_name()));
            write_png(&path, scale.physical_size(), &pixels).map_err(io_error)?;
        }

        Ok(())
    })
}

struct Difference {
    different_pixels: usize,
    max_channel_error: f32,
}

fn compare_png(
    baseline_path: &std::path::Path,
    current_path: &std::path::Path,
) -> std::io::Result<Difference> {
    let baseline = read_png(baseline_path)?;
    let current = read_png(current_path)?;
    if baseline.0 != current.0 || baseline.1 != current.1 {
        return Err(std::io::Error::other(format!(
            "PNG dimensions differ: baseline {}x{}, current {}x{}",
            baseline.0, baseline.1, current.0, current.1
        )));
    }

    let mut different_pixels = 0;
    let mut max_channel = 0u8;
    for (baseline_pixel, current_pixel) in baseline.2.chunks_exact(4).zip(current.2.chunks_exact(4))
    {
        let pixel_diff = baseline_pixel
            .iter()
            .zip(current_pixel)
            .map(|(a, b)| a.abs_diff(*b))
            .max()
            .unwrap_or(0);
        if pixel_diff > 0 {
            different_pixels += 1;
        }
        max_channel = max_channel.max(pixel_diff);
    }

    Ok(Difference {
        different_pixels,
        max_channel_error: f32::from(max_channel) / 255.0 * 100.0,
    })
}

fn read_png(path: &std::path::Path) -> std::io::Result<(u32, u32, Vec<u8>)> {
    let file = std::fs::File::open(path)?;
    let decoder = png::Decoder::new(std::io::BufReader::new(file));
    let mut reader = decoder.read_info().map_err(std::io::Error::other)?;
    let mut pixels =
        vec![
            0;
            reader
                .output_buffer_size()
                .ok_or_else(|| std::io::Error::other("PNG output buffer is unavailable"))?
        ];
    let frame = reader
        .next_frame(&mut pixels)
        .map_err(std::io::Error::other)?;
    pixels.truncate(frame.buffer_size());
    if frame.color_type != png::ColorType::Rgba || frame.bit_depth != png::BitDepth::Eight {
        return Err(std::io::Error::other("PNG must be 8-bit RGBA"));
    }
    Ok((frame.width, frame.height, pixels))
}

fn write_png(path: &std::path::Path, size: Size<u32>, pixels: &[u8]) -> std::io::Result<()> {
    let file = std::fs::File::create(path)?;
    let mut encoder = png::Encoder::new(file, size.width, size.height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().map_err(std::io::Error::other)?;
    writer
        .write_image_data(pixels)
        .map_err(std::io::Error::other)?;
    Ok(())
}

#[derive(Clone, Copy)]
enum Page {
    Overview,
    Tokens,
    Shapes,
    Components,
    Overlays,
    About,
}

impl Page {
    const ALL: [Self; 6] = [
        Self::Overview,
        Self::Tokens,
        Self::Shapes,
        Self::Components,
        Self::Overlays,
        Self::About,
    ];

    fn file_name(self) -> &'static str {
        match self {
            Self::Overview => "overview",
            Self::Tokens => "tokens",
            Self::Shapes => "shapes",
            Self::Components => "components",
            Self::Overlays => "overlays",
            Self::About => "about",
        }
    }
}

use super::Gallery;

impl From<Page> for super::Page {
    fn from(page: Page) -> Self {
        match page {
            Page::Overview => Self::Overview,
            Page::Tokens => Self::Tokens,
            Page::Shapes => Self::Shapes,
            Page::Components => Self::Components,
            Page::Overlays => Self::Overlays,
            Page::About => Self::About,
        }
    }
}

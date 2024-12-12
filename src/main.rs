#![warn(warnings)]

mod cotation;
mod graph;
mod journal;

use clap::Parser;
use cotation::Cotation;
use journal::Journal;

type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Parser)]
/// Draw various graph from climbing journal.
///
/// Climbing journal format:
///
/// ```yaml
/// session date in YYYY-MM-DD format:
///     Wall name:
///         - bloc difficulty
/// ```
///
/// For example:
///
/// ```yaml
/// 2024-01-01:
///     Au coin:
///         - b5
///         - b6
///
/// 2024-01-02:
///     'Plug & Play':
///         - b1
///     La tornade:
///         - b2
/// ```
struct Opt {
    #[clap(long, short)]
    /// Path to saves the image, by default print the image in terminal.
    output: Option<std::path::PathBuf>,
    /// Path to your climbing journal.
    filename: std::path::PathBuf,
}

fn main() -> Result {
    let opt = Opt::parse();
    let file = std::fs::File::open(opt.filename)?;
    let journal: Journal = serde_yaml_ng::from_reader(&file)?;

    let mut image = [0u8; graph::LEN];
    {
        use plotters::drawing::IntoDrawingArea as _;

        let root = plotters::backend::BitMapBackend::with_buffer(&mut image, graph::SIZE)
            .into_drawing_area();
        root.fill(&plotters::style::WHITE)?;

        let roots = root.split_evenly((2, 2));

        graph::score::draw(&journal, &roots[0])?;
        graph::pie::draw(&journal, &roots[1])?;
        graph::wall::draw(&journal, &roots[2])?;
        graph::kind::draw(&journal, &roots[3])?;

        root.present()?;
    }

    if let Some(output) = opt.output {
        image::save_buffer(
            &output,
            &image,
            graph::SIZE.0,
            graph::SIZE.1,
            image::ColorType::Rgb8,
        )?;
    } else {
        print(&image)?;
    }

    Ok(())
}

fn print(image: &[u8]) -> Result {
    use base64::Engine as _;
    use image::ImageEncoder as _;
    use std::io::Write as _;

    let mut stdout = std::io::stdout();
    let mut png = Vec::new();

    image::codecs::png::PngEncoder::new(&mut png).write_image(
        image,
        graph::SIZE.0,
        graph::SIZE.1,
        image::ExtendedColorType::Rgb8,
    )?;

    writeln!(
        stdout,
        "\x1b]1337;File=inline=1;size={};width={}px;height={}px;preserveAspectRatio=1:{}\x07",
        image.len(),
        graph::SIZE.0,
        graph::SIZE.1,
        base64::engine::general_purpose::STANDARD.encode(png),
    )?;

    stdout.flush()?;

    Ok(())
}

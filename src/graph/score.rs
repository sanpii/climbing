use chrono::Datelike as _;
use plotters::style::Color as _;
use plotters::style::IntoFont as _;

pub fn draw(
    journal: &crate::Journal,
    root: &plotters::drawing::DrawingArea<plotters::backend::BitMapBackend, plotters::coord::Shift>,
) -> crate::Result {
    let mut x = journal.iter().map(|(x, _)| *x);
    let y = journal.iter().map(|(_, y)| y.score() as f32);

    let mut chart = plotters::chart::ChartBuilder::on(root)
        .caption("Score / jour", ("sans-serif", 30).into_font())
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(10)
        .build_cartesian_2d(bound_x(x.clone()), bound_y(y.clone()))?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .y_desc("Score (n * 2^difficulté)")
        .y_label_formatter(&|y| format!("{y:0}"))
        .draw()?;

    chart.draw_series(journal.iter().map(|(x, y)| {
        let style = y
            .color()
            .as_ref()
            .map(plotters::style::RGBAColor::filled)
            .unwrap_or_else(|| plotters::style::ShapeStyle {
                color: plotters::style::BLACK.mix(1.),
                filled: false,
                stroke_width: 1,
            });
        plotters::prelude::Circle::new((*x, y.score() as f32), 3, style)
    }))?;

    chart.draw_series(plotters::series::LineSeries::new(
        x.clone().zip(y.clone()),
        plotters::style::BLACK,
    ))?;

    let regression = Affine::regression(
        x.clone().map(|x| x.num_days_from_ce() as f32).collect(),
        y.collect(),
    );

    let x0 = x.next().unwrap();
    let x1 = x.last().unwrap();

    let points = vec![
        (x0, regression.y(x0.num_days_from_ce() as f32)),
        (x1, regression.y(x1.num_days_from_ce() as f32)),
    ];

    chart.draw_series(plotters::series::DottedLineSeries::new(points, 0, 3, |c| {
        plotters::element::Pixel::new(c, plotters::style::colors::full_palette::GREY)
    }))?;

    Ok(())
}

fn bound_x<I: Iterator<Item = chrono::NaiveDate> + Clone>(
    data: I,
) -> std::ops::Range<chrono::NaiveDate> {
    let min = data.clone().min().unwrap_or_default();
    let max = data.max().unwrap_or_default();

    min..max
}

fn bound_y<I: Iterator<Item = f32> + Clone>(data: I) -> std::ops::Range<f32> {
    let max = data.fold(f32::MIN, |acc, x| acc.max(x));

    0f32..max
}

#[derive(Debug)]
struct Affine {
    a: f32,
    b: f32,
}

impl Affine {
    fn y(&self, x: f32) -> f32 {
        self.a * x + self.b
    }

    fn regression(x: Vec<f32>, y: Vec<f32>) -> Self {
        let a = Self::a(&x, &y);

        Self {
            a,
            b: Self::b(a, &x, &y),
        }
    }

    fn a(x: &[f32], y: &[f32]) -> f32 {
        Self::variance(x, y) / Self::variance(x, x)
    }

    fn variance(x: &[f32], y: &[f32]) -> f32 {
        let mean_x = Self::mean(x);
        let mean_y = Self::mean(y);

        x.iter()
            .map(|x| x - mean_x)
            .zip(y.iter().map(|y| y - mean_y))
            .map(|(x, y)| x * y)
            .sum()
    }

    fn b(a: f32, x: &[f32], y: &[f32]) -> f32 {
        Self::mean(y) - a * Self::mean(x)
    }

    fn mean(x: &[f32]) -> f32 {
        x.iter().sum::<f32>() / x.len() as f32
    }
}

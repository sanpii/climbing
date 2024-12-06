use plotters::style::Color as _;
use plotters::style::IntoFont as _;

pub fn draw(
    journal: &crate::Journal,
    root: &plotters::drawing::DrawingArea<plotters::backend::BitMapBackend, plotters::coord::Shift>,
) -> crate::Result {
    let mut chart = plotters::chart::ChartBuilder::on(root)
        .caption("Score / jour", ("sans-serif", 30).into_font())
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(10)
        .build_cartesian_2d(
            bound_x(journal.iter().map(|(x, _)| *x)),
            bound_y(journal.iter().map(|(_, y)| y.score())),
        )?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .y_desc("Score (n * difficulté)")
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
        journal.iter().map(|(x, y)| (*x, y.score() as f32)),
        plotters::style::BLACK,
    ))?;

    Ok(())
}

fn bound_x<I: Iterator<Item = chrono::NaiveDate> + Clone>(
    data: I,
) -> std::ops::Range<chrono::NaiveDate> {
    let min = data.clone().min().unwrap_or_default();
    let max = data.max().unwrap_or_default();

    min..max
}

fn bound_y<I: Iterator<Item = u32> + Clone>(data: I) -> std::ops::Range<f32> {
    let max = data.max().unwrap_or_default() as f32;

    0f32..max
}

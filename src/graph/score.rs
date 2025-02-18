use chrono::Datelike as _;
use plotters::style::Color as _;
use plotters::style::IntoFont as _;

pub fn draw(
    journal: &crate::Journal,
    root: &plotters::drawing::DrawingArea<plotters::backend::BitMapBackend, plotters::coord::Shift>,
) -> crate::Result {
    let mut x = journal.iter().map(|(x, _)| *x);
    let y1 = journal.iter().map(|(_, y)| y.len() as f32);
    let y2 = journal.iter().map(|(_, y)| y.score() as f32);

    let mut chart = plotters::chart::ChartBuilder::on(root)
        .caption("Score / jour", ("sans-serif", 30).into_font())
        .x_label_area_size(35)
        .y_label_area_size(40)
        .right_y_label_area_size(40)
        .margin(10)
        .build_cartesian_2d(bound_x(x.clone()), bound_y(y1.clone()))?
        .set_secondary_coord(bound_x(x.clone()), bound_y(y2.clone()));

    chart
        .configure_mesh()
        .disable_x_mesh()
        .y_desc("nb")
        .light_line_style(plotters::style::TRANSPARENT)
        .y_label_formatter(&|y| format!("{y:0}"))
        .draw()?;

    chart
        .configure_secondary_axes()
        .y_desc("Score (n * 2^difficulté)")
        .y_label_formatter(&|y| format!("{y:0}"))
        .draw()?;

    let histogram = plotters::series::Histogram::vertical(&chart)
        .style(plotters::style::BLACK.mix(0.2).filled())
        .data(x.clone().zip(y1.clone()));
    chart.draw_series(histogram)?;

    chart.draw_secondary_series(journal.iter().map(|(x, y)| {
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

    chart.draw_secondary_series(plotters::series::LineSeries::new(
        x.clone().zip(y2.clone()),
        plotters::style::BLACK,
    ))?;

    let (a, b) = linregress(
        x.clone().map(|x| x.num_days_from_ce() as f64).collect(),
        y2.map(Into::into).collect(),
    )?;
    let x0 = x.next().unwrap();
    let x1 = x.last().unwrap();

    let points = vec![
        (x0, a * x0.num_days_from_ce() as f32 + b),
        (x1, a * x1.num_days_from_ce() as f32 + b),
    ];

    chart.draw_secondary_series(plotters::series::DottedLineSeries::new(points, 0, 3, |c| {
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

fn linregress(x: Vec<f64>, y: Vec<f64>) -> crate::Result<(f32, f32)> {
    let data = linregress::RegressionDataBuilder::new().build_from(vec![("Y", y), ("X", x)])?;
    let model = linregress::FormulaRegressionBuilder::new()
        .data(&data)
        .formula("Y ~ X")
        .fit_without_statistics()?;

    Ok((model[1] as f32, model[0] as f32))
}

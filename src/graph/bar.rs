use plotters::style::IntoTextStyle as _;
use std::collections::BTreeMap;

const MARGINS: (u32, u32) = (100, 20);

pub type Data<T> = BTreeMap<T, BTreeMap<crate::journal::Difficulty, i32>>;

pub fn draw<T: ToString>(
    title: &str,
    data: Data<T>,
    root: &plotters::drawing::DrawingArea<plotters::backend::BitMapBackend, plotters::coord::Shift>,
) -> crate::Result {
    let font_size = 13.;
    let text_style = ("sans-serif", font_size, &plotters::style::BLACK).into_text_style(root);
    let max = data.values().map(|x| x.values().sum()).max().unwrap_or(0);

    let mut pos = caption(title, root)?;
    pos.0 = 10;

    for (wall, stat) in data {
        root.draw_text(&wall.to_string(), &text_style, pos)?;

        let mut x = 100i32;
        let size = (super::SIZE.0 / 2 - MARGINS.0 - MARGINS.1) / max as u32;

        for (difficulty, nb) in stat {
            let upper = (x, pos.1);
            x += size as i32 * nb;
            let lower = (x, pos.1 + font_size as i32);

            let bar = plotters::element::Rectangle::new(
                [upper, lower],
                plotters::style::ShapeStyle::from(difficulty),
            );
            root.draw(&bar)?;
        }

        pos.1 += (font_size * 1.4) as i32;
    }

    axis(pos, max, root)?;

    Ok(())
}

fn caption(
    title: &str,
    root: &plotters::drawing::DrawingArea<plotters::backend::BitMapBackend, plotters::coord::Shift>,
) -> crate::Result<(i32, i32)> {
    use plotters::style::IntoFont as _;

    let (origin_dx, origin_dy) = root.get_base_pixel();
    let drawing_area = root.titled(title, ("sans-serif", 30).into_font())?;
    let (current_dx, current_dy) = drawing_area.get_base_pixel();

    Ok((current_dx - origin_dx, current_dy - origin_dy))
}

fn axis(
    pos: (i32, i32),
    max: i32,
    root: &plotters::drawing::DrawingArea<plotters::backend::BitMapBackend, plotters::coord::Shift>,
) -> crate::Result {
    let range = root.get_pixel_range();

    let upper = (MARGINS.0 as i32, pos.1);
    let lower = (range.0.end - MARGINS.1 as i32, pos.1);

    let axis = plotters::element::Rectangle::new([upper, lower], plotters::style::BLACK);
    root.draw(&axis)?;

    let step = (range.0.end - MARGINS.0 as i32 - MARGINS.1 as i32) / max;

    for x in 0..=max {
        let upper = (MARGINS.0 as i32 + step * x, pos.1);
        let mut lower = (MARGINS.0 as i32 + step * x, pos.1 + 5);

        if x % 5 == 0 {
            lower.1 += 5;
        }

        if x % 10 == 0 {
            lower.1 += 5;
        }

        let tic = plotters::element::Rectangle::new([upper, lower], plotters::style::BLACK);

        root.draw(&tic)?;
    }

    Ok(())
}

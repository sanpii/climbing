use plotters::style::IntoFont as _;
use std::collections::BTreeMap;

impl From<&crate::Journal> for BTreeMap<crate::Cotation, f64> {
    fn from(journal: &crate::Journal) -> Self {
        let mut values = BTreeMap::new();

        for entries in journal.values() {
            for difficulties in entries.values() {
                for difficulty in difficulties.iter() {
                    if let Some(x) = values.get_mut(difficulty) {
                        *x += 1.;
                    } else {
                        values.insert(*difficulty, 1.);
                    }
                }
            }
        }

        values
    }
}

pub fn draw(
    journal: &crate::Journal,
    root: &plotters::drawing::DrawingArea<plotters::backend::BitMapBackend, plotters::coord::Shift>,
) -> crate::Result {
    let values = BTreeMap::<crate::Cotation, f64>::from(journal);
    let total = values.values().sum::<f64>();

    root.titled(
        &format!("Total ({total:.0})"),
        ("sans-serif", 30).into_font(),
    )?;

    let range = root.get_pixel_range();
    let center = (
        range.0.start + (range.0.end - range.0.start) / 2,
        range.1.start + (range.1.end - range.1.start) / 2,
    );

    let sizes = values.values().copied().collect::<Vec<_>>();
    let labels = values
        .iter()
        .map(|(k, v)| format!("{k} ({v})"))
        .collect::<Vec<_>>();
    let colors = values.keys().map(Into::into).collect::<Vec<_>>();

    let pie = plotters::prelude::Pie::new(&center, &200., &sizes, &colors, &labels);

    root.draw(&pie)?;

    Ok(())
}

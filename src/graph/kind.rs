pub fn draw(
    journal: &crate::Journal,
    root: &plotters::drawing::DrawingArea<plotters::backend::BitMapBackend, plotters::coord::Shift>,
) -> crate::Result {
    let mut data = super::bar::Data::new();

    for entries in journal.values() {
        for (wall, entry) in entries.iter() {
            let kind = wall.kind();
            let x = data
                .entry(kind)
                .or_insert_with(std::collections::BTreeMap::new);

            for difficulty in entry.iter() {
                x.entry(*difficulty).and_modify(|x| *x += 1).or_insert(1);
            }
        }
    }

    super::bar::draw("Voix / catégories", data, root)
}

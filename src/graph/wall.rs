pub fn draw(
    journal: &crate::Journal,
    root: &plotters::drawing::DrawingArea<plotters::backend::BitMapBackend, plotters::coord::Shift>,
) -> crate::Result {
    let mut data = super::bar::Data::new();
    let mut max = 0;

    for entries in journal.values() {
        for (wall, entry) in entries.iter() {
            if !data.contains_key(wall) {
                data.insert(*wall, std::collections::BTreeMap::new());
            }

            let x = data.get_mut(wall).unwrap();
            for difficulty in entry.iter() {
                x.entry(*difficulty).and_modify(|x| *x += 1).or_insert(1);
            }
            max = usize::max(max, x.values().sum::<i32>() as usize);
        }
    }

    super::bar::draw("Voix / mur", data, root)
}

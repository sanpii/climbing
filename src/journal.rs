use std::collections::BTreeMap;

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Journal(BTreeMap<chrono::NaiveDate, Entries>);

impl std::ops::Deref for Journal {
    type Target = BTreeMap<chrono::NaiveDate, Entries>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Journal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone, Debug, Default, serde::Deserialize)]
pub struct Entries(BTreeMap<Wall, Entry>);

impl Entries {
    pub fn score(&self) -> u32 {
        self.0.values().map(Entry::score).sum()
    }

    pub fn count(&self) -> usize {
        self.0.values().map(|x| x.len()).sum()
    }

    pub fn color(&self) -> Option<plotters::style::RGBAColor> {
        self.0
            .values()
            .flat_map(|v| v.0.clone())
            .max()
            .map(Into::into)
    }
}

impl std::ops::Deref for Entries {
    type Target = BTreeMap<Wall, Entry>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Entries {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize)]
pub struct Entry(Vec<crate::Cotation>);

impl Entry {
    pub fn score(&self) -> u32 {
        self.0.iter().map(crate::Cotation::score).sum()
    }
}

impl std::ops::Deref for Entry {
    type Target = Vec<crate::Cotation>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Entry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Wall {
    #[serde(rename = "Au coin")]
    AuCoin,
    Arrow,
    #[serde(rename = "Le diamant")]
    LeDiamant,
    Origami,
    Hexaordinaire,
    #[serde(rename = "La tornade")]
    LaTornade,
    Pixels,

    Rhino,
    R2D2,
    #[serde(rename = "Plug & Play")]
    PlugPlay,
    Sushi,
    #[serde(rename = "Klein Klein")]
    KleinKlein,
    #[serde(rename = "War Space")]
    WarSpace,

    #[serde(rename = "Da Da Da")]
    DaDaDa,
    Sparrow,
    #[serde(rename = "Toux PI")]
    TouxPi,
    #[serde(rename = "White side")]
    WhiteSide,
    Bluetooth,

    #[serde(rename = "Woody Wood")]
    WoodyWood,
    Pecker,

    Poutre,
    #[serde(rename = "La Mother")]
    LaMother,

    Togepi,
    Pythagore,
    #[serde(rename = "Arrach'toi d'là")]
    ArrachToiDLa,

    #[serde(rename = "Gaffe à tes genoux")]
    GaffeATesGenoux,
    #[serde(rename = "Le Cul du Chien")]
    LeCulDuChien,
    #[serde(rename = "Bigoût")]
    Bigout,
    Sunny,
    #[serde(rename = "Océan")]
    Ocean,

    _Last,
}

impl Wall {
    pub const fn nb() -> usize {
        Self::_Last as usize
    }

    pub fn kind(&self) -> Kind {
        use Wall::*;

        match self {
            DaDaDa | WoodyWood | Pecker | Poutre => Kind::Dalle,
            Togepi | Pythagore | ArrachToiDLa => Kind::Enfant,
            Sparrow | WhiteSide | Bluetooth => Kind::Reta,
            Rhino | R2D2 | WarSpace | KleinKlein | Sushi | PlugPlay => Kind::Haut,
            AuCoin | Pixels | GaffeATesGenoux | Ocean => Kind::Dierdre,
            TouxPi | Arrow | LeDiamant | Origami | Hexaordinaire | LaTornade | LeCulDuChien
            | Bigout | Sunny | LaMother => Kind::Dever,

            _Last => unreachable!(),
        }
    }
}

impl std::fmt::Display for Wall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = serde_yaml_ng::to_string(self).unwrap();

        f.write_str(s.trim_end_matches('\n'))
    }
}

impl PartialOrd for Wall {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Wall {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.to_string().cmp(&other.to_string())
    }
}

#[derive(
    Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, serde::Deserialize, serde::Serialize,
)]
pub enum Kind {
    Dalle,
    Dever,
    Dierdre,
    Enfant,
    Haut,
    Reta,

    _Last,
}

impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = serde_yaml_ng::to_string(self).unwrap();

        f.write_str(s.trim_end_matches('\n'))
    }
}

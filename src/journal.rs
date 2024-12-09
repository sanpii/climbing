use plotters::style::Color as _;
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
pub struct Entry(Vec<Difficulty>);

impl Entry {
    pub fn score(&self) -> u32 {
        self.0.iter().map(Difficulty::score).sum()
    }
}

impl std::ops::Deref for Entry {
    type Target = Vec<Difficulty>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Entry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[repr(u32)]
pub enum Difficulty {
    B1 = 1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
    B8,
    B9,
    B10,
    B11,
    B12,
    B13,
    B14,
}

impl Difficulty {
    pub fn score(&self) -> u32 {
        2_u32.pow(*self as u32)
    }

    pub fn nb(&self) -> usize {
        Self::B14 as usize
    }
}

impl From<Difficulty> for plotters::style::RGBColor {
    fn from(value: Difficulty) -> Self {
        use plotters::style::colors::full_palette::*;
        use Difficulty::*;

        match value {
            B1 => YELLOW,
            B2 => YELLOW_700,
            B3 => ORANGE,
            B4 => ORANGE_700,
            B5 => BLUE,
            B6 => BLUE_700,
            B7 => RED,
            B8 => RED_700,
            B9 => GREY_200,
            B10 => GREY_500,
            B11 => GREY_700,
            B12 => BLACK,
            B13 => GREEN,
            B14 => GREEN_700,
        }
    }
}

impl From<&Difficulty> for plotters::style::RGBColor {
    fn from(value: &Difficulty) -> Self {
        (*value).into()
    }
}

impl From<Difficulty> for plotters::style::RGBAColor {
    fn from(value: Difficulty) -> Self {
        plotters::style::RGBColor::from(value).mix(1.)
    }
}

impl From<Difficulty> for plotters::style::ShapeStyle {
    fn from(value: Difficulty) -> Self {
        plotters::style::RGBAColor::from(value).filled()
    }
}

impl std::fmt::Display for Difficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(
    Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[repr(usize)]
pub enum Wall {
    #[serde(rename = "Au coin")]
    AuCoin = 0,
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
            WoodyWood | Pecker => Kind::Dalle,
            Togepi | Pythagore | ArrachToiDLa => Kind::Enfant,
            DaDaDa | Sparrow | WhiteSide | Bluetooth => Kind::Reta,
            Rhino | R2D2 | WarSpace | KleinKlein | Sushi | PlugPlay => Kind::Haut,
            AuCoin | Pixels | GaffeATesGenoux | Ocean => Kind::Dierdre,
            TouxPi | Arrow | LeDiamant | Origami | Hexaordinaire | LaTornade | LeCulDuChien
            | Bigout | Sunny | LaMother => Kind::Dever,

            _Last => unreachable!(),
        }
    }
}

impl From<usize> for Wall {
    fn from(value: usize) -> Self {
        if value >= Self::nb() {
            unreachable!();
        }

        unsafe { std::mem::transmute::<usize, Wall>(value) }
    }
}

impl std::fmt::Display for Wall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = serde_yaml_ng::to_string(self).unwrap();

        f.write_str(s.trim_end_matches('\n'))
    }
}

#[derive(
    Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, serde::Deserialize, serde::Serialize,
)]
#[repr(usize)]
pub enum Kind {
    Dalle = 0,
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

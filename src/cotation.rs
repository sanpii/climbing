use plotters::style::Color as _;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize)]
#[serde(untagged)]
pub enum Cotation {
    BlockOut(BlockOut),
}

impl Cotation {
    pub fn score(&self) -> u32 {
        use Cotation::*;

        match self {
            BlockOut(bo) => bo.score(),
        }
    }

    pub fn nb(&self) -> usize {
        14
    }
}

impl From<Cotation> for plotters::style::RGBColor {
    fn from(value: Cotation) -> Self {
        use Cotation::*;

        match value {
            BlockOut(bo) => bo.into(),
        }
    }
}

impl From<&Cotation> for plotters::style::RGBColor {
    fn from(value: &Cotation) -> Self {
        (*value).into()
    }
}

impl From<Cotation> for plotters::style::RGBAColor {
    fn from(value: Cotation) -> Self {
        plotters::style::RGBColor::from(value).mix(1.)
    }
}

impl From<Cotation> for plotters::style::ShapeStyle {
    fn from(value: Cotation) -> Self {
        plotters::style::RGBAColor::from(value).filled()
    }
}

impl std::fmt::Display for Cotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Cotation::*;

        match self {
            BlockOut(bo) => write!(f, "{bo}"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[repr(u32)]
pub enum BlockOut {
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

impl BlockOut {
    pub fn score(&self) -> u32 {
        2_u32.pow(*self as u32)
    }
}

impl From<BlockOut> for plotters::style::RGBColor {
    fn from(value: BlockOut) -> Self {
        use BlockOut::*;
        use plotters::style::colors::full_palette::*;

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

impl std::fmt::Display for BlockOut {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

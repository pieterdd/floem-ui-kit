use std::fmt::Display;

use floem::peniko::Color;
use strum::EnumIter;

#[derive(Debug, Clone, Copy, EnumIter, Eq, PartialEq, Hash)]
pub enum AccentColor {
    Magenta,
    Green,
    Cyan,
}

pub enum PrimaryFillColorVariant {
    DefaultGrayscale,
    DefaultColored,
    Hovered,
    Pressed,
    Disabled,
}

pub enum BorderColorVariant {
    DefaultGrayscale,
    DefaultColored,
    HoveredGrayscale,
    HoveredColored,
    FocusedColored,
    FocusedGrayscale,
    Disabled,
}

impl AccentColor {
    pub fn root_view_background(&self) -> Color {
        match self {
            Self::Magenta => Color::rgb(0.12, 0.11, 0.13),
            Self::Green => Color::rgb(0.11, 0.11, 0.12),
            Self::Cyan => Color::rgb(0.11, 0.12, 0.12),
        }
    }

    pub fn primary_fill_color(&self, variant: PrimaryFillColorVariant) -> Color {
        match variant {
            PrimaryFillColorVariant::DefaultColored => match self {
                Self::Magenta => Color::rgb(0.27, 0.02, 0.15),
                Self::Green => Color::rgb(0.07, 0.33, 0.15),
                Self::Cyan => Color::rgb(0.09, 0.24, 0.21),
            },
            PrimaryFillColorVariant::DefaultGrayscale => Color::BLACK.with_alpha_factor(0.3),
            PrimaryFillColorVariant::Hovered => match self {
                Self::Magenta => Color::rgb(0.29, 0.04, 0.17),
                Self::Green => Color::rgb(0.09, 0.35, 0.17),
                Self::Cyan => Color::rgb(0.11, 0.26, 0.23),
            },
            PrimaryFillColorVariant::Pressed => match self {
                Self::Magenta => Color::rgb(0.25, 0.00, 0.13),
                Self::Green => Color::rgb(0.05, 0.31, 0.13),
                Self::Cyan => Color::rgb(0.07, 0.22, 0.19),
            },
            PrimaryFillColorVariant::Disabled => Color::rgb(0.4, 0.4, 0.4),
        }
    }

    pub fn secondary_fill_color(&self) -> Color {
        match self {
            Self::Magenta => Color::rgb(0.34, 0.09, 0.2),
            Self::Green => Color::rgb(0.16, 0.36, 0.22),
            Self::Cyan => Color::rgb(0.16, 0.31, 0.28),
        }
    }

    pub fn border_color(&self, variant: BorderColorVariant) -> Color {
        match variant {
            BorderColorVariant::DefaultGrayscale => {
                Color::rgb(0.23, 0.23, 0.23).with_alpha_factor(0.8)
            }
            BorderColorVariant::DefaultColored => {
                Color::rgb(0.23, 0.23, 0.23).with_alpha_factor(0.8)
            }
            BorderColorVariant::HoveredGrayscale => {
                Color::rgb(0.27, 0.27, 0.27).with_alpha_factor(0.95)
            }
            BorderColorVariant::HoveredColored => match self {
                Self::Magenta => Color::rgb(0.36, 0.11, 0.22),
                Self::Green => Color::rgb(0.16, 0.39, 0.21),
                Self::Cyan => Color::rgb(0.18, 0.33, 0.30),
            },
            BorderColorVariant::FocusedColored => match self {
                Self::Magenta => Color::rgb(0.40, 0.15, 0.26),
                Self::Green => Color::rgb(0.20, 0.43, 0.25),
                Self::Cyan => Color::rgb(0.22, 0.37, 0.34),
            },
            BorderColorVariant::FocusedGrayscale => Color::rgb(0.29, 0.29, 0.29),
            BorderColorVariant::Disabled => Color::rgb(0.3, 0.3, 0.3),
        }
    }

    pub fn disabled_text_color(&self) -> Color {
        Color::rgb(0.5, 0.5, 0.5)
    }
}

impl Display for AccentColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Magenta => write!(f, "Magenta"),
            Self::Green => write!(f, "Green"),
            Self::Cyan => write!(f, "Cyan"),
        }
    }
}

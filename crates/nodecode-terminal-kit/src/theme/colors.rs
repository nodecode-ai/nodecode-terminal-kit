//! Tailwind-style color family definitions for theme systems.

use crate::theme::Color;

/// A single color family with 11 shades (50-950).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorScale {
    pub c50: Color,
    pub c100: Color,
    pub c200: Color,
    pub c300: Color,
    pub c400: Color,
    pub c500: Color,
    pub c600: Color,
    pub c700: Color,
    pub c800: Color,
    pub c900: Color,
    pub c950: Color,
}

impl Default for ColorScale {
    fn default() -> Self {
        Self {
            c50: Color::Rgb {
                r: 249,
                g: 250,
                b: 251,
            },
            c100: Color::Rgb {
                r: 243,
                g: 244,
                b: 246,
            },
            c200: Color::Rgb {
                r: 229,
                g: 231,
                b: 235,
            },
            c300: Color::Rgb {
                r: 209,
                g: 213,
                b: 219,
            },
            c400: Color::Rgb {
                r: 156,
                g: 163,
                b: 175,
            },
            c500: Color::Rgb {
                r: 107,
                g: 114,
                b: 128,
            },
            c600: Color::Rgb {
                r: 75,
                g: 85,
                b: 99,
            },
            c700: Color::Rgb {
                r: 55,
                g: 65,
                b: 81,
            },
            c800: Color::Rgb {
                r: 31,
                g: 41,
                b: 55,
            },
            c900: Color::Rgb {
                r: 17,
                g: 24,
                b: 39,
            },
            c950: Color::Rgb { r: 3, g: 7, b: 18 },
        }
    }
}

/// Named color families.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorFamilies {
    pub slate: ColorScale,
    pub gray: ColorScale,
    pub zinc: ColorScale,
    pub neutral: ColorScale,
    pub stone: ColorScale,
    pub red: ColorScale,
    pub orange: ColorScale,
    pub amber: ColorScale,
    pub yellow: ColorScale,
    pub lime: ColorScale,
    pub green: ColorScale,
    pub emerald: ColorScale,
    pub teal: ColorScale,
    pub cyan: ColorScale,
    pub sky: ColorScale,
    pub blue: ColorScale,
    pub indigo: ColorScale,
    pub violet: ColorScale,
    pub purple: ColorScale,
    pub fuchsia: ColorScale,
    pub pink: ColorScale,
    pub rose: ColorScale,
}

impl Default for ColorFamilies {
    fn default() -> Self {
        let default_scale = ColorScale::default();

        Self {
            slate: default_scale,
            gray: default_scale,
            zinc: default_scale,
            neutral: default_scale,
            stone: default_scale,
            red: default_scale,
            orange: default_scale,
            amber: default_scale,
            yellow: default_scale,
            lime: default_scale,
            green: default_scale,
            emerald: default_scale,
            teal: default_scale,
            cyan: default_scale,
            sky: default_scale,
            blue: default_scale,
            indigo: default_scale,
            violet: default_scale,
            purple: default_scale,
            fuchsia: default_scale,
            pink: default_scale,
            rose: default_scale,
        }
    }
}

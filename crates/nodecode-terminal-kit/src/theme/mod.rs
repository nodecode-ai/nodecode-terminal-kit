use ratatui::style::{Color as RatatuiColor, Modifier, Style};
use ratatui::widgets::BorderType;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub mod colors;
pub mod cursor;

pub use colors::{ColorFamilies, ColorScale};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Color {
    Rgb { r: u8, g: u8, b: u8 },
    Indexed(u8),
}

impl Default for Color {
    fn default() -> Self {
        Self::Rgb {
            r: 255,
            g: 255,
            b: 255,
        }
    }
}

pub fn to_ratatui(color: Color) -> RatatuiColor {
    match color {
        Color::Rgb { r, g, b } => RatatuiColor::Rgb(r, g, b),
        Color::Indexed(idx) => RatatuiColor::Indexed(idx),
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ThemeOverride {
    pub name: Option<String>,
    #[serde(default)]
    pub colors: BTreeMap<String, Color>,
}

#[derive(Debug, Clone)]
pub struct ThemePalette {
    pub background_surface: Color,
    pub primary: Color,
    pub background_elevated: Color,
    pub foreground: Color,
    pub border_focused: Color,
    pub secondary: Color,
    pub success: Color,
    pub error: Color,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Theme {
    pub name: String,
    pub background: Color,
    pub foreground: Color,
    pub primary: Color,
    pub secondary: Color,
    pub tertiary: Color,
    pub quaternary: Color,
    pub accent1: Color,
    pub accent2: Color,
    pub accent3: Color,
    pub accent4: Color,
    pub user_input: Color,
    pub success: Color,
    pub error: Color,
    pub info: Color,
    pub selection: Color,
    pub cursor: Color,
    pub border: Color,
    pub border_subtle: Color,
    pub border_focused: Color,
    pub background_void: Color,
    pub background_canvas: Color,
    pub background_subtle: Color,
    pub background_surface: Color,
    pub background_elevated: Color,
    pub background_floating: Color,
    pub background_hover: Color,
    pub background_active: Color,
    pub background_selected: Color,
    pub background_faded: Color,
    pub background_input: Color,
    pub background_badge: Color,
    pub background_track: Color,
    pub background_thumb: Color,
    pub gray50: Color,
    pub gray100: Color,
    pub gray200: Color,
    pub gray300: Color,
    pub gray400: Color,
    pub gray500: Color,
    pub gray600: Color,
    pub gray700: Color,
    pub gray800: Color,
    pub gray900: Color,
    pub gray950: Color,
    pub border_type: BorderType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeElement {
    Base,
    Foreground,
    Primary,
    Secondary,
    Tertiary,
    Quaternary,
    Accent1,
    Accent2,
    Accent3,
    Accent4,
    UserInput,
    Success,
    Error,
    Info,
    Selection,
    Cursor,
    Border,
    BorderSubtle,
    BorderFocused,
    BackgroundVoid,
    BackgroundCanvas,
    BackgroundSubtle,
    BackgroundSurface,
    BackgroundElevated,
    BackgroundFloating,
    BackgroundHover,
    BackgroundActive,
    BackgroundSelected,
    BackgroundFaded,
    BackgroundInput,
    BackgroundBadge,
    BackgroundTrack,
    BackgroundThumb,
    Gray50,
    Gray100,
    Gray200,
    Gray300,
    Gray400,
    Gray500,
    Gray600,
    Gray700,
    Gray800,
    Gray900,
    Gray950,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeState {
    Normal,
    Hovered,
    Active,
    Disabled,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            name: "nodecode-default".to_string(),
            background: Color::Rgb {
                r: 10,
                g: 12,
                b: 14,
            },
            foreground: Color::Rgb {
                r: 230,
                g: 232,
                b: 235,
            },
            primary: Color::Rgb {
                r: 232,
                g: 236,
                b: 244,
            },
            secondary: Color::Rgb {
                r: 168,
                g: 176,
                b: 188,
            },
            tertiary: Color::Rgb {
                r: 134,
                g: 145,
                b: 160,
            },
            quaternary: Color::Rgb {
                r: 108,
                g: 121,
                b: 138,
            },
            accent1: Color::Rgb {
                r: 110,
                g: 168,
                b: 255,
            },
            accent2: Color::Rgb {
                r: 71,
                g: 208,
                b: 177,
            },
            accent3: Color::Rgb {
                r: 255,
                g: 198,
                b: 93,
            },
            accent4: Color::Rgb {
                r: 219,
                g: 132,
                b: 255,
            },
            user_input: Color::Rgb {
                r: 232,
                g: 236,
                b: 244,
            },
            success: Color::Rgb {
                r: 94,
                g: 201,
                b: 123,
            },
            error: Color::Rgb {
                r: 236,
                g: 99,
                b: 110,
            },
            info: Color::Rgb {
                r: 90,
                g: 175,
                b: 255,
            },
            selection: Color::Rgb {
                r: 60,
                g: 72,
                b: 89,
            },
            cursor: Color::Rgb {
                r: 232,
                g: 236,
                b: 244,
            },
            border: Color::Rgb {
                r: 58,
                g: 67,
                b: 80,
            },
            border_subtle: Color::Rgb {
                r: 41,
                g: 47,
                b: 56,
            },
            border_focused: Color::Rgb {
                r: 110,
                g: 168,
                b: 255,
            },
            background_void: Color::Rgb { r: 6, g: 8, b: 10 },
            background_canvas: Color::Rgb {
                r: 10,
                g: 12,
                b: 14,
            },
            background_subtle: Color::Rgb {
                r: 15,
                g: 18,
                b: 22,
            },
            background_surface: Color::Rgb {
                r: 19,
                g: 23,
                b: 28,
            },
            background_elevated: Color::Rgb {
                r: 24,
                g: 29,
                b: 35,
            },
            background_floating: Color::Rgb {
                r: 28,
                g: 34,
                b: 41,
            },
            background_hover: Color::Rgb {
                r: 31,
                g: 38,
                b: 46,
            },
            background_active: Color::Rgb {
                r: 35,
                g: 43,
                b: 53,
            },
            background_selected: Color::Rgb {
                r: 46,
                g: 55,
                b: 67,
            },
            background_faded: Color::Rgb {
                r: 12,
                g: 15,
                b: 18,
            },
            background_input: Color::Rgb {
                r: 12,
                g: 15,
                b: 18,
            },
            background_badge: Color::Rgb {
                r: 32,
                g: 38,
                b: 46,
            },
            background_track: Color::Rgb {
                r: 24,
                g: 30,
                b: 37,
            },
            background_thumb: Color::Rgb {
                r: 72,
                g: 84,
                b: 99,
            },
            gray50: Color::Rgb {
                r: 249,
                g: 250,
                b: 251,
            },
            gray100: Color::Rgb {
                r: 243,
                g: 244,
                b: 246,
            },
            gray200: Color::Rgb {
                r: 229,
                g: 231,
                b: 235,
            },
            gray300: Color::Rgb {
                r: 209,
                g: 213,
                b: 219,
            },
            gray400: Color::Rgb {
                r: 156,
                g: 163,
                b: 175,
            },
            gray500: Color::Rgb {
                r: 107,
                g: 114,
                b: 128,
            },
            gray600: Color::Rgb {
                r: 75,
                g: 85,
                b: 99,
            },
            gray700: Color::Rgb {
                r: 55,
                g: 65,
                b: 81,
            },
            gray800: Color::Rgb {
                r: 31,
                g: 41,
                b: 55,
            },
            gray900: Color::Rgb {
                r: 17,
                g: 24,
                b: 39,
            },
            gray950: Color::Rgb { r: 3, g: 7, b: 18 },
            border_type: BorderType::Plain,
        }
    }
}

impl Theme {
    pub fn apply_overrides(mut self, ov: &ThemeOverride) -> Self {
        if let Some(name) = &ov.name {
            self.name = name.clone();
        }
        for (k, v) in &ov.colors {
            match k.as_str() {
                "background" => self.background = *v,
                "foreground" => self.foreground = *v,
                "primary" => self.primary = *v,
                "secondary" => self.secondary = *v,
                "tertiary" => self.tertiary = *v,
                "quaternary" => self.quaternary = *v,
                "accent1" => self.accent1 = *v,
                "accent2" => self.accent2 = *v,
                "accent3" => self.accent3 = *v,
                "accent4" => self.accent4 = *v,
                "user-input" => self.user_input = *v,
                "success" => self.success = *v,
                "error" => self.error = *v,
                "info" => self.info = *v,
                "selection" => self.selection = *v,
                "cursor" => self.cursor = *v,
                "border" => self.border = *v,
                "border-subtle" => self.border_subtle = *v,
                "border-focused" => self.border_focused = *v,
                "background-void" => self.background_void = *v,
                "background-canvas" => self.background_canvas = *v,
                "background-subtle" => self.background_subtle = *v,
                "background-surface" => self.background_surface = *v,
                "background-elevated" => self.background_elevated = *v,
                "background-floating" => self.background_floating = *v,
                "background-hover" => self.background_hover = *v,
                "background-active" => self.background_active = *v,
                "background-selected" => self.background_selected = *v,
                "background-faded" => self.background_faded = *v,
                "background-input" => self.background_input = *v,
                "background-badge" => self.background_badge = *v,
                "background-track" => self.background_track = *v,
                "background-thumb" => self.background_thumb = *v,
                "gray50" => self.gray50 = *v,
                "gray100" => self.gray100 = *v,
                "gray200" => self.gray200 = *v,
                "gray300" => self.gray300 = *v,
                "gray400" => self.gray400 = *v,
                "gray500" => self.gray500 = *v,
                "gray600" => self.gray600 = *v,
                "gray700" => self.gray700 = *v,
                "gray800" => self.gray800 = *v,
                "gray900" => self.gray900 = *v,
                "gray950" => self.gray950 = *v,
                _ => {}
            }
        }
        self
    }

    pub fn palette(&self) -> ThemePalette {
        ThemePalette {
            background_surface: self.background_surface,
            primary: self.primary,
            background_elevated: self.background_elevated,
            foreground: self.foreground,
            border_focused: self.border_focused,
            secondary: self.secondary,
            success: self.success,
            error: self.error,
        }
    }

    #[must_use]
    pub fn style(&self, element: ThemeElement) -> Style {
        use ThemeElement::*;
        match element {
            Base => Style::default()
                .fg(to_ratatui(self.foreground))
                .bg(to_ratatui(self.background)),
            Foreground => Style::default().fg(to_ratatui(self.foreground)),
            Primary => Style::default().fg(to_ratatui(self.primary)),
            Secondary => Style::default().fg(to_ratatui(self.secondary)),
            Tertiary => Style::default().fg(to_ratatui(self.tertiary)),
            Quaternary => Style::default().fg(to_ratatui(self.quaternary)),
            Accent1 => Style::default().fg(to_ratatui(self.accent1)),
            Accent2 => Style::default().fg(to_ratatui(self.accent2)),
            Accent3 => Style::default().fg(to_ratatui(self.accent3)),
            Accent4 => Style::default().fg(to_ratatui(self.accent4)),
            UserInput => Style::default().fg(to_ratatui(self.user_input)),
            Success => Style::default().fg(to_ratatui(self.success)),
            Error => Style::default().fg(to_ratatui(self.error)),
            Info => Style::default().fg(to_ratatui(self.info)),
            Selection => Style::default().fg(to_ratatui(self.selection)),
            Cursor => Style::default().bg(to_ratatui(self.cursor)),
            Border => Style::default().fg(to_ratatui(self.border)),
            BorderSubtle => Style::default().fg(to_ratatui(self.border_subtle)),
            BorderFocused => Style::default().fg(to_ratatui(self.border_focused)),
            BackgroundVoid => Style::default().bg(to_ratatui(self.background_void)),
            BackgroundCanvas => Style::default().bg(to_ratatui(self.background_canvas)),
            BackgroundSubtle => Style::default().bg(to_ratatui(self.background_subtle)),
            BackgroundSurface => Style::default().bg(to_ratatui(self.background_surface)),
            BackgroundElevated => Style::default().bg(to_ratatui(self.background_elevated)),
            BackgroundFloating => Style::default().bg(to_ratatui(self.background_floating)),
            BackgroundHover => Style::default().bg(to_ratatui(self.background_hover)),
            BackgroundActive => Style::default().bg(to_ratatui(self.background_active)),
            BackgroundSelected => Style::default().bg(to_ratatui(self.background_selected)),
            BackgroundFaded => Style::default().bg(to_ratatui(self.background_faded)),
            BackgroundInput => Style::default().bg(to_ratatui(self.background_input)),
            BackgroundBadge => Style::default().bg(to_ratatui(self.background_badge)),
            BackgroundTrack => Style::default().bg(to_ratatui(self.background_track)),
            BackgroundThumb => Style::default().bg(to_ratatui(self.background_thumb)),
            Gray50 => Style::default().fg(to_ratatui(self.gray50)),
            Gray100 => Style::default().fg(to_ratatui(self.gray100)),
            Gray200 => Style::default().fg(to_ratatui(self.gray200)),
            Gray300 => Style::default().fg(to_ratatui(self.gray300)),
            Gray400 => Style::default().fg(to_ratatui(self.gray400)),
            Gray500 => Style::default().fg(to_ratatui(self.gray500)),
            Gray600 => Style::default().fg(to_ratatui(self.gray600)),
            Gray700 => Style::default().fg(to_ratatui(self.gray700)),
            Gray800 => Style::default().fg(to_ratatui(self.gray800)),
            Gray900 => Style::default().fg(to_ratatui(self.gray900)),
            Gray950 => Style::default().fg(to_ratatui(self.gray950)),
        }
    }

    #[must_use]
    pub fn style_state(&self, element: ThemeElement, state: ThemeState) -> Style {
        let base = self.style(element);
        match state {
            ThemeState::Normal => base,
            ThemeState::Hovered => base.bg(to_ratatui(self.background_hover)),
            ThemeState::Active => base.bg(to_ratatui(self.background_active)),
            ThemeState::Disabled => base
                .fg(to_ratatui(self.quaternary))
                .add_modifier(Modifier::DIM),
        }
    }

    #[must_use]
    pub fn base_style(&self) -> Style {
        self.style(ThemeElement::Base)
    }

    #[must_use]
    pub fn primary_style(&self) -> Style {
        self.style(ThemeElement::Primary)
    }

    #[must_use]
    pub fn secondary_style(&self) -> Style {
        self.style(ThemeElement::Secondary)
    }

    #[must_use]
    pub fn border_focused_style(&self) -> Style {
        self.style(ThemeElement::BorderFocused)
            .add_modifier(Modifier::BOLD)
    }

    #[must_use]
    pub fn agent_color(&self, agent: &str, agent_color_override: Option<(u8, u8, u8)>) -> Color {
        if let Some((r, g, b)) = agent_color_override {
            return Color::Rgb { r, g, b };
        }

        match agent {
            "exec" => self.success,
            "plan" => self.accent2,
            _ => self.info,
        }
    }
}

#[must_use]
pub fn blend_colors(foreground: &Color, background: &Color, opacity: f32) -> RatatuiColor {
    let o = opacity.clamp(0.0, 1.0);
    match (foreground, background) {
        (
            Color::Rgb {
                r: fr,
                g: fg,
                b: fb,
            },
            Color::Rgb {
                r: br,
                g: bg,
                b: bb,
            },
        ) => {
            let r = (*fr as f32 * o + *br as f32 * (1.0 - o)) as u8;
            let g = (*fg as f32 * o + *bg as f32 * (1.0 - o)) as u8;
            let b = (*fb as f32 * o + *bb as f32 * (1.0 - o)) as u8;
            RatatuiColor::Rgb(r, g, b)
        }
        _ => to_ratatui(*foreground),
    }
}

#[must_use]
pub fn blend_theme_color(foreground: &Color, background: &Color, opacity: f32) -> Color {
    let o = opacity.clamp(0.0, 1.0);
    match (foreground, background) {
        (
            Color::Rgb {
                r: fr,
                g: fg,
                b: fb,
            },
            Color::Rgb {
                r: br,
                g: bg,
                b: bb,
            },
        ) => {
            let r = (*fr as f32 * o + *br as f32 * (1.0 - o)) as u8;
            let g = (*fg as f32 * o + *bg as f32 * (1.0 - o)) as u8;
            let b = (*fb as f32 * o + *bb as f32 * (1.0 - o)) as u8;
            Color::Rgb { r, g, b }
        }
        _ => *foreground,
    }
}

#[must_use]
pub fn list_item_style(
    theme: &Theme,
    element: ThemeElement,
    selected: bool,
    hovered: bool,
) -> Style {
    if selected {
        Style::default()
            .fg(to_ratatui(theme.selection))
            .add_modifier(Modifier::BOLD)
    } else if element == ThemeElement::Base {
        let bg = if hovered {
            theme.background_hover
        } else {
            theme.background_surface
        };
        Style::default()
            .fg(to_ratatui(theme.foreground))
            .bg(to_ratatui(bg))
    } else {
        theme.style_state(
            element,
            if hovered {
                ThemeState::Hovered
            } else {
                ThemeState::Normal
            },
        )
    }
}

#[derive(Debug, Clone, Default)]
pub struct ThemeFacade {
    inner: Theme,
}

#[derive(Debug, thiserror::Error)]
#[error("theme error: {0}")]
pub struct ThemeFacadeError(pub String);

impl ThemeFacade {
    pub fn from_theme(theme: Theme) -> Self {
        Self { inner: theme }
    }

    pub fn apply_overrides(self, ov: &ThemeOverride) -> Self {
        Self {
            inner: self.inner.apply_overrides(ov),
        }
    }

    pub fn palette(&self) -> ThemePalette {
        self.inner.palette()
    }

    pub fn theme(&self) -> &Theme {
        &self.inner
    }
}

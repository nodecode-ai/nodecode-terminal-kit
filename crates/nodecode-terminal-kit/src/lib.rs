#![forbid(unsafe_code)]

//! Design-only terminal UI kit for composing TUI products.
//!
//! `nodecode-terminal-kit` contains reusable terminal UI building blocks:
//! components, layout helpers, primitives, and a unified theme model.
//!
//! # Non-goals
//! - No chat/session runtime.
//! - No networking/server logic.
//! - No product-specific orchestration.

/// Reusable UI components (dialogs, inputs, lists, overlays).
pub mod components;
/// Layout helpers and branding/text utilities.
pub mod layout;
/// Low-level primitives shared across components.
pub mod primitives;
/// Theme model, palette, styling, and color helpers.
pub mod theme;
/// Generic multi-step wizard building blocks.
pub mod wizard;

/// Common imports for consumers embedding the design kit.
pub mod prelude {
    /// Low-level prelude for design-system composition.
    pub mod core {
        pub use crate::components::{
            component, dialog_shell, dropdown, help_bar, input_box, key_hints, lines_viewport,
            list, list_items, overlay_dialog, picker, picker_dialog, search_bar, tabbed_dialog,
            tabbed_prompt_dialog, text_input,
        };
        pub use crate::layout::{branding, picker_kit, section_stack, text as layout_text};
        pub use crate::primitives::{
            geom, path, rich_text, scrollbar, shimmer, text as primitive_text,
        };
        pub use crate::theme::cursor;
        pub use crate::theme::{
            blend_colors, blend_theme_color, list_item_style, to_ratatui, Color, ColorFamilies,
            ColorScale, Theme, ThemeElement, ThemeOverride, ThemePalette, ThemeState,
        };
        pub use crate::wizard;
    }

    pub use core::*;
}

use crate::components::dialog_shell::DialogOptions;
use crate::components::{tabbed_dialog, tabbed_prompt_dialog};
use crate::layout::picker_kit::{
    key_to_picker_msg, mouse_to_picker_msg, PickerHooks, PickerMsg, PickerOptions, PickerState,
};
use crate::theme::{Theme, ThemeElement};
use ratatui::crossterm::event::{KeyEvent, MouseEvent};
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};

pub use crate::components::tabbed_prompt_dialog::SearchSpec;

pub struct SingleTabHeader<'a> {
    pub inactive_label: &'a str,
    pub counted_label: &'a str,
    pub title: &'a str,
}

pub fn default_tab_style(theme: &Theme, is_active: bool) -> Style {
    let mut style = theme.style(ThemeElement::Tertiary);
    if is_active {
        style = theme
            .style(ThemeElement::Selection)
            .add_modifier(Modifier::BOLD);
    }
    style
}

pub fn counted_tab_label(total: usize, counted_label: &str, inactive_label: &str) -> String {
    if total > 0 {
        format!(" {} ({}) ", counted_label, total)
    } else {
        format!(" {} ", inactive_label)
    }
}

pub fn render_picker_prompt_dialog<'a, Tab, Label>(
    frame: &mut ratatui::Frame,
    area: Rect,
    theme: &Theme,
    opts: DialogOptions,
    order: &[Tab],
    active: Tab,
    label: Label,
    title: &str,
    search: Option<SearchSpec<'a>>,
    body: tabbed_dialog::TabBody<'a>,
    footer_text: &str,
) where
    Tab: Copy + Eq,
    Label: FnMut(Tab) -> String,
{
    tabbed_prompt_dialog::render_tabbed_prompt_dialog(
        frame,
        area,
        theme,
        opts,
        order,
        active,
        label,
        |_, is_active, theme| default_tab_style(theme, is_active),
        title,
        search,
        body,
        footer_text,
    );
}

pub fn render_single_tab_picker_dialog<'a>(
    frame: &mut ratatui::Frame,
    area: Rect,
    theme: &Theme,
    opts: DialogOptions,
    header: SingleTabHeader<'_>,
    total: usize,
    search: Option<SearchSpec<'a>>,
    body: tabbed_dialog::TabBody<'a>,
    footer_text: &str,
) {
    let order = [()];
    let active = ();
    let label = counted_tab_label(total, header.counted_label, header.inactive_label);

    render_picker_prompt_dialog(
        frame,
        area,
        theme,
        opts,
        &order,
        active,
        move |_| label.clone(),
        header.title,
        search,
        body,
        footer_text,
    );
}

pub fn picker_key_msg<T, Custom>(
    state: &mut PickerState<T>,
    key: KeyEvent,
    opts: &PickerOptions<T>,
) -> Option<PickerMsg<Custom>>
where
    T: Send + 'static,
    Custom: Clone + Send + 'static,
{
    key_to_picker_msg(state, key, None, opts, &())
}

pub fn picker_key_msg_with<T, H, Custom>(
    state: &mut PickerState<T>,
    key: KeyEvent,
    visible_height: Option<usize>,
    opts: &PickerOptions<T>,
    hooks: &H,
) -> Option<PickerMsg<Custom>>
where
    H: PickerHooks<T, Custom>,
    T: Send + 'static,
    Custom: Clone + Send + 'static,
{
    key_to_picker_msg(state, key, visible_height, opts, hooks)
}

pub fn picker_mouse_msg<T, Custom>(
    state: &mut PickerState<T>,
    mouse: MouseEvent,
    area: Rect,
    dialog_opts: DialogOptions,
) -> Option<PickerMsg<Custom>>
where
    T: Send + 'static,
    Custom: Clone + Send + 'static,
{
    mouse_to_picker_msg(state, mouse, area, dialog_opts)
}

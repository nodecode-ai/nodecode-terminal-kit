//! Generic wizard view rendering

use super::framework::{ItemListView, WizardItem};
use super::model::{GenericWizardModel, ViewMode};
use crate::components::{dialog_shell, tabbed_dialog};
use crate::theme::{to_ratatui, Theme, ThemeElement};
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::Text;
use ratatui::widgets::{List, ListItem, Padding, Paragraph, Wrap};
use ratatui::Frame;
use unicode_width::UnicodeWidthStr;

pub const DIALOG_OPTS_HOOKS: dialog_shell::DialogOptions = dialog_shell::DialogOptions {
    width_pct: 1.0,
    height_pct: 1.0,
    max_width: 60,
    max_height: 20,
    header_rows: 3,
    footer_rows: 1,
    padding: Padding::new(1, 1, 1, 1),
};

pub fn generic_wizard_view<T, L>(
    model: &GenericWizardModel<T>,
    list_view: &L,
    frame: &mut Frame,
    area: Rect,
    theme: &Theme,
    title: &str,
    empty_label: &str,
    tab_labels: &[String],
    list_active_tab: usize,
    wizard_active_tab: usize,
) where
    T: WizardItem,
    L: ItemListView<T>,
{
    if !model.is_open {
        return;
    }

    let active_tab = match model.view_mode {
        ViewMode::List => list_active_tab,
        ViewMode::Wizard | ViewMode::Confirmation => wizard_active_tab,
    };

    match model.view_mode {
        ViewMode::List => render_list_view(
            model,
            list_view,
            frame,
            area,
            theme,
            title,
            empty_label,
            tab_labels,
            active_tab,
        ),
        ViewMode::Wizard => {
            render_wizard_view(model, frame, area, theme, title, tab_labels, active_tab)
        }
        ViewMode::Confirmation => {
            render_confirmation_view(model, frame, area, theme, tab_labels, active_tab)
        }
    }
}

pub(crate) fn render_dialog_shell(
    frame: &mut Frame,
    area: Rect,
    theme: &Theme,
    tab_labels: &[String],
    active_tab: usize,
    title_line: impl FnOnce(u16) -> String,
) -> dialog_shell::DialogLayout {
    let layout = dialog_shell::layout_centered(frame, area, theme, DIALOG_OPTS_HOOKS);

    let header_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(layout.header);

    if tab_labels.is_empty() {
        let tab_bar = Paragraph::new(Text::default())
            .style(theme.base_style())
            .wrap(Wrap { trim: false });
        frame.render_widget(tab_bar, header_chunks[0]);
    } else {
        let order: Vec<usize> = (0..tab_labels.len()).collect();
        let active_tab = active_tab.min(tab_labels.len().saturating_sub(1));
        let tab_line = tabbed_dialog::tab_bar_line_from_order(
            &order,
            active_tab,
            header_chunks[0].width,
            |tab| format!(" {} ", tab_labels[tab]),
            |_, is_active| {
                let mut style = theme.style(ThemeElement::Tertiary);
                if is_active {
                    style = theme
                        .style(ThemeElement::Selection)
                        .add_modifier(Modifier::BOLD);
                }
                style
            },
        );
        let tab_bar = Paragraph::new(Text::from(vec![tab_line]))
            .style(theme.base_style())
            .wrap(Wrap { trim: false });
        frame.render_widget(tab_bar, header_chunks[0]);
    }

    let title_text = title_line(header_chunks[2].width);
    let title_paragraph = Paragraph::new(title_text)
        .style(
            theme
                .style(ThemeElement::Primary)
                .add_modifier(Modifier::BOLD),
        )
        .wrap(Wrap { trim: false });
    frame.render_widget(title_paragraph, header_chunks[2]);

    layout
}

fn format_title_with_indicator(width: u16, base_text: &str, indicator: &str) -> String {
    let base_len = UnicodeWidthStr::width(base_text);
    let indicator_len = UnicodeWidthStr::width(indicator);
    let total_width = width as usize;
    let gap = total_width.saturating_sub(base_len + indicator_len);

    if gap < 1 {
        return format!("{} {}", base_text, indicator);
    }

    format!("{}{}{}", base_text, " ".repeat(gap), indicator)
}

fn render_footer_text(frame: &mut Frame, area: Rect, theme: &Theme, text: &str) {
    let footer = Paragraph::new(text)
        .style(theme.style(ThemeElement::Tertiary))
        .alignment(Alignment::Left);
    frame.render_widget(footer, area);
}

fn render_list_view<T, L>(
    model: &GenericWizardModel<T>,
    list_view: &L,
    frame: &mut Frame,
    area: Rect,
    theme: &Theme,
    title: &str,
    empty_label: &str,
    tab_labels: &[String],
    active_tab: usize,
) where
    T: WizardItem,
    L: ItemListView<T>,
{
    let layout = render_dialog_shell(frame, area, theme, tab_labels, active_tab, |_| {
        title.to_string()
    });

    if model.items.is_empty() {
        let empty_message = format!("No {} configured", empty_label.to_lowercase());
        crate::components::picker::render_centered_message(
            frame,
            layout.body,
            theme,
            &empty_message,
        );
    } else {
        let items: Vec<ListItem> = model
            .items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let is_selected = i == model.selected_idx;
                list_view.render_item(item, is_selected, theme)
            })
            .collect();

        let list_widget =
            List::new(items).highlight_style(Style::default().add_modifier(Modifier::BOLD));

        frame.render_stateful_widget(
            list_widget,
            layout.body,
            &mut ratatui::widgets::ListState::default().with_selected(Some(model.selected_idx)),
        );
    }

    let help_text = if model.items.is_empty() {
        let tab_hint = if tab_labels.len() > 1 {
            "  tab next"
        } else {
            ""
        };
        format!("esc close{}  n new item", tab_hint)
    } else {
        "enter edit  esc close  ↑↓ navigate  n new".to_string()
    };

    render_footer_text(frame, layout.footer, theme, &help_text);
}

fn render_wizard_view<T>(
    model: &GenericWizardModel<T>,
    frame: &mut Frame,
    area: Rect,
    theme: &Theme,
    base_title: &str,
    tab_labels: &[String],
    active_tab: usize,
) where
    T: WizardItem,
{
    let wizard = match &model.wizard {
        Some(w) => w,
        None => return,
    };

    let step_num = wizard.current_step_number();
    let step_count = wizard.step_count();
    let step_title = wizard.current_step().title();

    let layout = render_dialog_shell(frame, area, theme, tab_labels, active_tab, |width| {
        let step_indicator = format!("[{}/{}]", step_num, step_count);
        let base_text = format!("{} - {}", base_title, step_title);
        format_title_with_indicator(width, &base_text, &step_indicator)
    });

    let help_text = wizard.current_step().help_text();
    let content_height = wizard.current_step().content_height();

    let body_split_constraints = [
        Constraint::Length(content_height),
        Constraint::Length(2),
        Constraint::Min(0),
    ];

    let body_split = Layout::default()
        .direction(Direction::Vertical)
        .constraints(body_split_constraints)
        .split(layout.body);

    wizard
        .current_step()
        .render(frame, body_split[0], theme, wizard.item());

    crate::components::help_bar::render_help_bar(
        frame,
        body_split[1],
        theme,
        help_text,
        Some(ratatui::layout::Alignment::Left),
    );

    let nav_text = wizard.current_step().navigation_hint().unwrap_or_else(|| {
        if step_num == 1 {
            "enter next  esc cancel"
        } else if step_num == step_count {
            "enter save  esc cancel  ← back"
        } else {
            "enter next  esc cancel  ← back"
        }
    });

    render_footer_text(frame, layout.footer, theme, nav_text);
}

fn render_confirmation_view<T>(
    model: &GenericWizardModel<T>,
    frame: &mut Frame,
    area: Rect,
    theme: &Theme,
    tab_labels: &[String],
    active_tab: usize,
) where
    T: WizardItem,
{
    let delete_id = match &model.pending_delete {
        Some(id) => id,
        None => return,
    };

    let item_name = model
        .items
        .iter()
        .find(|item| item.id() == *delete_id)
        .map(|item| item.display_name())
        .unwrap_or_else(|| "Unknown".to_string());

    let layout = render_dialog_shell(frame, area, theme, tab_labels, active_tab, |_| {
        "Confirm Delete".to_string()
    });

    let message = format!("Delete '{}'?\n\nThis action cannot be undone.", item_name);

    let text = Paragraph::new(message)
        .style(Style::default().fg(to_ratatui(theme.foreground)))
        .alignment(Alignment::Center);

    frame.render_widget(text, layout.body);

    render_footer_text(frame, layout.footer, theme, "y confirm  n cancel");
}

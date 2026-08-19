use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::{app::App, data, theme, ui};

pub fn draw(f: &mut Frame, area: Rect, app: &App) {
    let cols =
        Layout::horizontal([Constraint::Length(40), Constraint::Min(0)]).split(area);

    let tree = data::tree();

    let items: Vec<ListItem> = tree
        .iter()
        .map(|r| {
            let indent = "  ".repeat(r.depth);
            let name_style = if r.is_file {
                Style::default().fg(Color::Gray)
            } else {
                Style::default().fg(theme::TIME).add_modifier(Modifier::BOLD)
            };
            ListItem::new(Line::from(vec![
                Span::styled(format!("{indent}{} ", r.glyph), Style::default().fg(theme::MUTED)),
                Span::styled(r.name, name_style),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(ui::block("Files", true))
        .highlight_style(Style::default().bg(theme::SELECTION).add_modifier(Modifier::BOLD));
    let mut state = ListState::default();
    state.select(Some(app.files_sel));
    f.render_stateful_widget(list, cols[0], &mut state);

    // preview panel for the selected entry
    let row = &tree[app.files_sel.min(tree.len() - 1)];
    let preview = preview_lines(row.name, row.is_file);
    f.render_widget(Paragraph::new(preview).block(ui::block(row.name, false)), cols[1]);
}

fn preview_lines(name: &str, is_file: bool) -> Vec<Line<'static>> {
    let kw = Style::default().fg(theme::HASH);
    let ty = Style::default().fg(theme::TITLE);
    let cmt = Style::default().fg(theme::MUTED);
    let txt = Style::default().fg(Color::Gray);

    match name {
        "color.rs" => vec![
            Line::styled("//! ANSI + indexed + RGB color model.", cmt),
            Line::raw(""),
            Line::from(vec![Span::styled("pub enum ", kw), Span::styled("Color {", ty)]),
            Line::styled("    Reset,", txt),
            Line::styled("    Black,", txt),
            Line::styled("    Red,", txt),
            Line::styled("    Indexed(u8),", txt),
            Line::styled("    Rgb(u8, u8, u8),", txt),
            Line::styled("}", ty),
        ],
        "README.md" => vec![
            Line::styled("# Ratatui", Style::default().fg(theme::TIME)),
            Line::raw(""),
            Line::styled("Cook up terminal user interfaces.", txt),
            Line::raw(""),
            Line::styled("- 60+ widgets on a fixed cell grid", txt),
            Line::styled("- crossterm / termion / termwiz backends", txt),
        ],
        _ if !is_file => vec![Line::styled(format!("{name}/"), cmt), Line::raw(""), Line::styled("(directory)", cmt)],
        _ => vec![Line::styled(format!("// {name}"), cmt), Line::raw(""), Line::styled("(binary or unchanged)", cmt)],
    }
}

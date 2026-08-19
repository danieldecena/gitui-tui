use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{List, ListState, Paragraph},
    Frame,
};

use crate::{app::App, data, theme, ui};

pub fn draw(f: &mut Frame, area: Rect, app: &App) {
    let cols =
        Layout::horizontal([Constraint::Length(48), Constraint::Min(0)]).split(area);

    // left: the working-tree changes that will be stashed
    let changes = data::stashing_changes();
    let items: Vec<_> = changes.iter().map(ui::file_item).collect();
    let list = List::new(items)
        .block(ui::block("Changes to stash", true))
        .highlight_style(Style::default().bg(theme::SELECTION).add_modifier(Modifier::BOLD));
    let mut state = ListState::default();
    state.select(Some(app.stashing_sel));
    f.render_stateful_widget(list, cols[0], &mut state);

    // right: stash message + options
    let right = Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).split(cols[1]);

    let message = Line::from(vec![
        Span::raw("WIP: refactor color model"),
        // a block cursor drawn as a reversed cell
        Span::styled(" ", Style::default().bg(theme::SELECTION)),
    ]);
    f.render_widget(
        Paragraph::new(message).block(ui::block("Stash message", false)),
        right[0],
    );

    let on = Style::default().fg(theme::ADDED);
    let off = Style::default().fg(theme::MUTED);
    let key = Style::default().fg(theme::KEY).add_modifier(Modifier::BOLD);
    let opts = vec![
        Line::from(vec![
            Span::styled("[i] ", key),
            Span::styled("[ ] ", off),
            Span::styled("keep staged changes in index", Style::default().fg(Color::Gray)),
        ]),
        Line::from(vec![
            Span::styled("[u] ", key),
            Span::styled("[x] ", on),
            Span::styled("include untracked files", Style::default().fg(Color::Gray)),
        ]),
        Line::raw(""),
        Line::from(vec![
            Span::styled(format!("{} files · ", changes.len()), off),
            Span::styled("will stash and reset working tree", on),
        ]),
    ];
    f.render_widget(Paragraph::new(opts).block(ui::block("Options", false)), right[1]);
}

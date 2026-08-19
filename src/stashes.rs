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
        Layout::horizontal([Constraint::Length(48), Constraint::Min(0)]).split(area);

    let stashes = data::stashes();

    // Two lines per stash: ref/time/message, then the base branch + hash.
    let items: Vec<ListItem> = stashes
        .iter()
        .map(|s| {
            ListItem::new(vec![
                Line::from(vec![
                    Span::styled(format!("{:<9}", s.reff), Style::default().fg(theme::HASH)),
                    Span::styled(format!("{:>3} ", s.time), Style::default().fg(theme::TIME)),
                    Span::raw(s.msg),
                ]),
                Line::styled(
                    format!("         on {} · {}", s.branch, s.hash),
                    Style::default().fg(theme::MUTED),
                ),
            ])
        })
        .collect();

    let list = List::new(items)
        .block(ui::block("Stashes", true))
        .highlight_style(Style::default().bg(theme::SELECTION).add_modifier(Modifier::BOLD));
    let mut state = ListState::default();
    state.select(Some(app.stashes_sel));
    f.render_stateful_widget(list, cols[0], &mut state);

    // details of the selected stash
    let s = &stashes[app.stashes_sel.min(stashes.len() - 1)];
    let mut lines = vec![
        Line::from(vec![
            Span::styled("stashed ", Style::default().fg(theme::MUTED)),
            Span::styled(format!("{} ago", s.time), Style::default().fg(theme::TIME)),
            Span::styled(" · on ", Style::default().fg(theme::MUTED)),
            Span::styled(s.branch, Style::default().fg(theme::AUTHOR)),
            Span::styled(" · ", Style::default().fg(theme::MUTED)),
            Span::styled(s.hash, Style::default().fg(theme::HASH)),
        ]),
        Line::styled(s.msg, Style::default().add_modifier(Modifier::BOLD)),
        Line::raw(""),
    ];
    for (stat, name) in &s.files {
        let color = match stat {
            'M' => theme::MODIFIED,
            '+' => theme::ADDED,
            'D' => theme::REMOVED,
            _ => Color::Gray,
        };
        lines.push(Line::from(vec![
            Span::styled(format!("{stat} "), Style::default().fg(color)),
            Span::raw(*name),
        ]));
    }
    f.render_widget(Paragraph::new(lines).block(ui::block(s.reff, false)), cols[1]);
}

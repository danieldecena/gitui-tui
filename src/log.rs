use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::{app::App, data, theme, ui};

pub fn draw(f: &mut Frame, area: Rect, app: &App) {
    let rows =
        Layout::vertical([Constraint::Min(0), Constraint::Length(8)]).split(area);

    let commits = data::commits();

    let items: Vec<ListItem> = commits
        .iter()
        .map(|c| {
            ListItem::new(Line::from(vec![
                Span::styled(format!("{:<3}", c.graph), Style::default().fg(theme::MUTED)),
                Span::styled(format!("{:<8} ", c.hash), Style::default().fg(theme::HASH)),
                Span::styled(
                    format!("{:<13}", truncate(c.author, 12)),
                    Style::default().fg(theme::AUTHOR),
                ),
                Span::styled(format!("{:>3} ", c.time), Style::default().fg(theme::TIME)),
                Span::raw(c.msg),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(ui::block("Commit", true))
        .highlight_style(Style::default().bg(theme::SELECTION).add_modifier(Modifier::BOLD));
    let mut state = ListState::default();
    state.select(Some(app.log_sel));
    f.render_stateful_widget(list, rows[0], &mut state);

    // details of the selected commit
    let c = &commits[app.log_sel.min(commits.len() - 1)];
    let details = vec![
        Line::styled(format!("commit {}0f2b1a", c.hash), Style::default().fg(theme::TITLE)),
        Line::styled(format!("Author:  {}", c.author), Style::default().fg(theme::MUTED)),
        Line::styled(format!("Date:    {} ago", c.time), Style::default().fg(theme::MUTED)),
        Line::raw(""),
        Line::styled(format!("    {}", c.msg), Style::default().add_modifier(Modifier::BOLD)),
    ];
    f.render_widget(Paragraph::new(details).block(ui::block("Details", false)), rows[1]);
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}…", &s[..max.saturating_sub(1)])
    }
}

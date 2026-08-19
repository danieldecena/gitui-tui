use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{List, ListState, Paragraph},
    Frame,
};

use crate::{
    app::App,
    data::{self, DiffKind, FileChange},
    theme, ui,
};

pub fn draw(f: &mut Frame, area: Rect, app: &App) {
    let cols =
        Layout::horizontal([Constraint::Length(46), Constraint::Min(0)]).split(area);
    let rows = Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(cols[0]);

    let unstaged = data::unstaged();
    let staged = data::staged();

    file_list(f, rows[0], "Unstaged", &unstaged, app.status_pane == 0, app.status_sel[0]);
    file_list(f, rows[1], "Staged", &staged, app.status_pane == 1, app.status_sel[1]);

    // The diff panel follows whichever pane has focus.
    let active = if app.status_pane == 0 {
        &unstaged[app.status_sel[0].min(unstaged.len() - 1)]
    } else {
        &staged[app.status_sel[1].min(staged.len() - 1)]
    };
    diff(f, cols[1], active);
}

fn file_list(
    f: &mut Frame,
    area: Rect,
    title: &str,
    files: &[FileChange],
    focused: bool,
    sel: usize,
) {
    let items: Vec<_> = files.iter().map(ui::file_item).collect();
    let list = List::new(items)
        .block(ui::block(title, focused))
        .highlight_style(Style::default().bg(theme::SELECTION).add_modifier(Modifier::BOLD));

    let mut state = ListState::default();
    // Only the focused pane shows its selection bar (gitui dims the other).
    if focused {
        state.select(Some(sel));
    }
    f.render_stateful_widget(list, area, &mut state);
}

fn diff(f: &mut Frame, area: Rect, fc: &FileChange) {
    let lines: Vec<Line> = fc
        .diff
        .iter()
        .map(|d| match d.kind {
            DiffKind::File => Line::styled(
                d.text.clone(),
                Style::default().fg(theme::MUTED).add_modifier(Modifier::BOLD),
            ),
            DiffKind::Hunk => Line::styled(d.text.clone(), Style::default().fg(theme::TIME)),
            DiffKind::Add => {
                Line::styled(format!("+{}", d.text), Style::default().fg(theme::ADDED))
            }
            DiffKind::Del => {
                Line::styled(format!("-{}", d.text), Style::default().fg(theme::REMOVED))
            }
            DiffKind::Ctx => {
                Line::styled(format!(" {}", d.text), Style::default().fg(Color::Gray))
            }
        })
        .collect();

    let name = fc.path.rsplit('/').next().unwrap_or(&fc.path);
    f.render_widget(
        Paragraph::new(lines).block(ui::block(&format!("Diff: {name}"), false)),
        area,
    );
}

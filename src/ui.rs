use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, ListItem, Paragraph, Tabs},
    Frame,
};

use crate::{app::App, data::FileChange, files, log, stashes, stashing, status, theme};

const TAB_TITLES: [&str; 5] = [
    "Status [1]",
    "Log [2]",
    "Files [3]",
    "Stashing [4]",
    "Stashes [5]",
];

/// Top-level frame: header (tabs + branch), the active tab, command bar.
pub fn draw(f: &mut Frame, app: &App) {
    let root = Layout::vertical([
        Constraint::Length(1), // header
        Constraint::Min(0),    // content
        Constraint::Length(1), // command bar
    ])
    .split(f.area());

    draw_header(f, root[0], app);

    match app.tab {
        0 => status::draw(f, root[1], app),
        1 => log::draw(f, root[1], app),
        2 => files::draw(f, root[1], app),
        3 => stashing::draw(f, root[1], app),
        _ => stashes::draw(f, root[1], app),
    }

    draw_command_bar(f, root[2], app);
}

fn draw_header(f: &mut Frame, area: Rect, app: &App) {
    let cols =
        Layout::horizontal([Constraint::Min(0), Constraint::Length(14)]).split(area);

    let tabs = Tabs::new(TAB_TITLES.iter().map(|t| Line::from(*t)).collect::<Vec<_>>())
        .select(app.tab)
        .divider("│")
        .style(Style::default().fg(theme::MUTED))
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(theme::SELECTION)
                .add_modifier(Modifier::BOLD),
        );
    f.render_widget(tabs, cols[0]);

    let branch = Line::from(vec![
        Span::styled("⎇ main", Style::default().fg(theme::HASH)),
        Span::styled(" ↑2", Style::default().fg(theme::AUTHOR)),
        Span::styled("↓0", Style::default().fg(theme::REMOVED)),
    ])
    .alignment(Alignment::Right);
    f.render_widget(Paragraph::new(branch), cols[1]);
}

fn draw_command_bar(f: &mut Frame, area: Rect, app: &App) {
    // contextual keybinds, mirroring the HTML kit's per-tab command line
    let cmds: &[(&str, &str)] = match app.tab {
        0 => &[("Tab", "toggle"), ("←→", "pane"), ("s", "stage"), ("D", "reset"), ("c", "commit")],
        1 => &[("↑↓", "move"), ("↵", "details"), ("c", "checkout"), ("P", "push"), ("F", "fetch")],
        2 => &[("↑↓", "move"), ("↵", "open"), ("f", "find")],
        3 => &[("s", "stash"), ("↑↓", "move"), ("i", "index"), ("u", "untracked")],
        _ => &[("a", "apply"), ("p", "pop"), ("D", "drop"), ("i", "inspect")],
    };

    let key = Style::default()
        .fg(Color::Black)
        .bg(theme::KEY)
        .add_modifier(Modifier::BOLD);
    let muted = Style::default().fg(theme::MUTED);

    let mut spans: Vec<Span> = Vec::new();
    for (k, a) in cmds {
        spans.push(Span::styled(format!(" {k} "), key));
        spans.push(Span::styled(format!(" {a}   "), muted));
    }
    spans.push(Span::styled(" ? ", key));
    spans.push(Span::styled(" help   ", muted));
    spans.push(Span::styled(" q ", key));
    spans.push(Span::styled(" quit", muted));

    f.render_widget(Paragraph::new(Line::from(spans)), area);
}

/// A bordered Block with a notched, bold title — the foundation of every
/// panel. Border turns yellow when the panel is focused.
pub fn block(title: &str, focused: bool) -> Block<'static> {
    let border = if focused { theme::FOCUS } else { theme::BORDER };
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .border_style(Style::default().fg(border))
        .title(Span::styled(
            format!(" {title} "),
            Style::default().fg(theme::TITLE).add_modifier(Modifier::BOLD),
        ))
}

/// A file-change list row: a colored status letter then the path.
pub fn file_item(fc: &FileChange) -> ListItem<'static> {
    let color = match fc.stat {
        'M' => theme::MODIFIED,
        '+' => theme::ADDED,
        'D' => theme::REMOVED,
        'R' => theme::RENAMED,
        _ => Color::Gray,
    };
    ListItem::new(Line::from(vec![
        Span::styled(format!("{} ", fc.stat), Style::default().fg(color)),
        Span::raw(fc.path.clone()),
    ]))
}

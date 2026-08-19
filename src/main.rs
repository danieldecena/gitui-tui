use std::io;

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;

mod app;
mod data;
mod theme;
mod ui;
// one module per tab screen
mod files;
mod log;
mod stashes;
mod stashing;
mod status;

use app::App;

fn main() -> io::Result<()> {
    // ratatui::init() enables raw mode + alternate screen and installs a panic
    // hook that restores the terminal, so a crash never leaves it garbled.
    let mut terminal = ratatui::init();
    let result = run(&mut terminal);
    ratatui::restore();
    result
}

fn run(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let mut app = App::new();
    loop {
        terminal.draw(|frame| ui::draw(frame, &app))?;

        // Blocking read — a TUI only repaints on input (or a tick, which this
        // demo doesn't need). Ignore key *release* events on platforms that
        // send them (Windows) so a keypress doesn't register twice.
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => break,
                KeyCode::Char('1') => app.tab = 0,
                KeyCode::Char('2') => app.tab = 1,
                KeyCode::Char('3') => app.tab = 2,
                KeyCode::Char('4') => app.tab = 3,
                KeyCode::Char('5') => app.tab = 4,
                KeyCode::Tab => app.tab = (app.tab + 1) % 5,
                KeyCode::BackTab => app.tab = (app.tab + 4) % 5,
                KeyCode::Down | KeyCode::Char('j') => app.move_sel(1),
                KeyCode::Up | KeyCode::Char('k') => app.move_sel(-1),
                // On the Status tab, ←/→ (or h/l) toggles focus between the
                // Unstaged and Staged panes.
                KeyCode::Left
                | KeyCode::Right
                | KeyCode::Char('h')
                | KeyCode::Char('l') => {
                    if app.tab == 0 {
                        app.status_pane ^= 1;
                    }
                }
                _ => {}
            }
        }
    }
    Ok(())
}

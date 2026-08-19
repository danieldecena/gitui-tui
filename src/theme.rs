use ratatui::style::Color;

// gitui's default theme, expressed as the named ANSI colors Ratatui ships.
// The exact on-screen shade is decided by the user's terminal palette — these
// names are the contract, the hexes in the HTML kit are just its preview.
pub const SELECTION: Color = Color::Blue; // the reversed-video selection bar
pub const HASH: Color = Color::Magenta; // commit / stash short hashes
pub const AUTHOR: Color = Color::Green; // commit authors
pub const TIME: Color = Color::Cyan; // relative times, folders, hunk headers
pub const ADDED: Color = Color::Green; // diff added lines / new files
pub const REMOVED: Color = Color::Red; // diff removed lines / deletions
pub const MODIFIED: Color = Color::Yellow; // modified-file status letter
pub const RENAMED: Color = Color::Cyan; // renamed-file status letter
pub const TITLE: Color = Color::Yellow; // block titles
pub const KEY: Color = Color::Yellow; // keybind chips in the command bar
pub const MUTED: Color = Color::DarkGray; // secondary text
pub const FOCUS: Color = Color::Yellow; // focused block border
pub const BORDER: Color = Color::DarkGray; // unfocused block border

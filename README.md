# gitui-tui

A runnable [Ratatui](https://ratatui.rs) recreation of the **gitui** design from
the Ratatui Design system — the same five tabs, blue selection bar, diff panel
and contextual keybind footer, rendered by real terminal widgets.

This is the HTML kit turned into actual code: each mockup primitive maps onto
its Ratatui widget (`Block`, `List`, `Tabs`, `Paragraph`), and the ANSI color
names (`Color::Blue`, `Color::Magenta`, …) let your terminal theme pick the
exact shades.

## Run it

```sh
cd gitui-tui
cargo run
```

You need a Rust toolchain (`rustup` / `cargo`). First build pulls `ratatui`
(and its bundled `crossterm`) from crates.io.

## Keys

| Key            | Action                                   |
| -------------- | ---------------------------------------- |
| `1`–`5`        | jump to a tab                            |
| `Tab`          | next tab (`Shift+Tab` previous)          |
| `↑`/`↓`, `j/k` | move the selection                       |
| `←`/`→`, `h/l` | Status tab: toggle Unstaged/Staged focus |
| `q` / `Esc`    | quit                                     |

## Layout

- `src/main.rs` — terminal init/restore + the input loop
- `src/app.rs` — UI state (active tab, per-view cursor) and navigation
- `src/ui.rs` — top-level frame: header tabs, command bar, shared `block()` helper
- `src/theme.rs` — the named-color theme
- `src/data.rs` — static demo fixtures (commits, changes, tree, stashes)
- `src/{status,log,files,stashing,stashes}.rs` — one draw fn per tab

The data is hardcoded. To make it a real client, replace the `data::*`
functions with calls into `git2`/libgit2 (or by shelling out to `git`), and
add key handlers in `main.rs` for the actions the command bar advertises
(`s` stage, `c` commit, `a` apply, …).

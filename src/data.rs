// Static demo content — the same fixtures the HTML kit shows, so the two
// stay recognizably the same app. In a real client these come from `git2` /
// libgit2 or shelling out to `git`.

#[derive(Clone, Copy)]
pub enum DiffKind {
    File, // "diff --git ..." header line
    Hunk, // "@@ -.. +.. @@"
    Add,  // added line (rendered with a leading '+')
    Del,  // removed line (leading '-')
    Ctx,  // unchanged context line (leading ' ')
}

pub struct DiffLine {
    pub kind: DiffKind,
    pub text: String,
}

fn dl(kind: DiffKind, text: &str) -> DiffLine {
    DiffLine {
        kind,
        text: text.to_string(),
    }
}

pub struct FileChange {
    pub stat: char, // M / + / D / R
    pub path: String,
    pub diff: Vec<DiffLine>,
}

fn fc(stat: char, path: &str, diff: Vec<DiffLine>) -> FileChange {
    FileChange {
        stat,
        path: path.to_string(),
        diff,
    }
}

pub fn unstaged() -> Vec<FileChange> {
    use DiffKind::*;
    vec![
        fc(
            'M',
            "ratatui-core/src/style/color.rs",
            vec![
                dl(File, "diff --git a/color.rs b/color.rs"),
                dl(Hunk, "@@ -212,7 +212,9 @@ impl Color {"),
                dl(Ctx, "    pub const fn from_ansi(code: u8) -> Self {"),
                dl(Ctx, "        match code {"),
                dl(Del, "            0 => Color::Black,"),
                dl(Add, "            0 => Color::Reset,"),
                dl(Add, "            1 => Color::Red,"),
                dl(Ctx, "            _ => Color::Indexed(code),"),
                dl(Ctx, "        }"),
            ],
        ),
        fc(
            'M',
            "ratatui-widgets/src/list.rs",
            vec![
                dl(File, "diff --git a/list.rs b/list.rs"),
                dl(Hunk, "@@ -88,6 +88,7 @@ impl List {"),
                dl(Ctx, "    fn render_item(&self, i: usize) {"),
                dl(Del, "        let s = &self.items[i];"),
                dl(Add, "        let s = self.items.get(i)?;"),
                dl(Add, "        // guard against stale selection"),
                dl(Ctx, "        draw(s);"),
            ],
        ),
        fc(
            '+',
            "examples/canvas_braille.rs",
            vec![
                dl(File, "diff --git a/canvas_braille.rs (new file)"),
                dl(Hunk, "@@ -0,0 +1,4 @@"),
                dl(Add, "use ratatui::widgets::canvas::Canvas;"),
                dl(Add, ""),
                dl(Add, "fn main() -> io::Result<()> {"),
                dl(Add, "    let mut terminal = ratatui::init();"),
            ],
        ),
    ]
}

pub fn staged() -> Vec<FileChange> {
    use DiffKind::*;
    vec![
        fc(
            'M',
            "Cargo.toml",
            vec![
                dl(File, "diff --git a/Cargo.toml b/Cargo.toml"),
                dl(Hunk, "@@ -3,7 +3,7 @@"),
                dl(Ctx, "[package]"),
                dl(Del, "version = \"0.30.0\""),
                dl(Add, "version = \"0.30.1\""),
                dl(Ctx, "edition = \"2021\""),
            ],
        ),
        fc(
            'R',
            "src/app.rs -> src/tui/app.rs",
            vec![
                dl(File, "similarity index 96%"),
                dl(Hunk, "rename from src/app.rs"),
                dl(Hunk, "rename to src/tui/app.rs"),
            ],
        ),
    ]
}

pub fn stashing_changes() -> Vec<FileChange> {
    vec![
        fc('M', "ratatui-core/src/style/color.rs", vec![]),
        fc('M', "ratatui-widgets/src/list.rs", vec![]),
        fc('+', "examples/canvas_braille.rs", vec![]),
        fc('M', "Cargo.toml", vec![]),
    ]
}

pub struct Commit {
    pub graph: &'static str,
    pub hash: &'static str,
    pub author: &'static str,
    pub time: &'static str,
    pub msg: &'static str,
}

pub fn commits() -> Vec<Commit> {
    vec![
        Commit { graph: "●",  hash: "a1f4c2e", author: "j. anderson", time: "2m",  msg: "list: guard against stale selection index" },
        Commit { graph: "●",  hash: "9d2b71a", author: "orhun",       time: "38m", msg: "color: add Color::from_ansi const fn" },
        Commit { graph: "●╮", hash: "4e88f01", author: "kdheepak",    time: "3h",  msg: "Merge pull request #1420 'canvas braille'" },
        Commit { graph: "│●", hash: "c07aa93", author: "pav",         time: "5h",  msg: "canvas: high-res braille marker example" },
        Commit { graph: "●╯", hash: "bb1029d", author: "orhun",       time: "6h",  msg: "release: bump workspace to 0.30.1" },
        Commit { graph: "●",  hash: "77c3e5f", author: "j. anderson", time: "1d",  msg: "table: cache column layout between frames" },
        Commit { graph: "●",  hash: "de90a14", author: "ratatui-bot", time: "1d",  msg: "ci: pin nightly toolchain 2024-06-30" },
        Commit { graph: "●",  hash: "2a5f9c8", author: "kdheepak",    time: "2d",  msg: "scrollbar: honor track symbol overrides" },
    ]
}

pub struct TreeRow {
    pub depth: usize,
    pub glyph: &'static str, // ▾ open dir, ▸ closed dir, "" file
    pub name: &'static str,
    pub is_file: bool,
}

pub fn tree() -> Vec<TreeRow> {
    vec![
        TreeRow { depth: 0, glyph: "▾", name: "ratatui-core", is_file: false },
        TreeRow { depth: 1, glyph: "▾", name: "src", is_file: false },
        TreeRow { depth: 2, glyph: "▾", name: "style", is_file: false },
        TreeRow { depth: 3, glyph: " ", name: "color.rs", is_file: true },
        TreeRow { depth: 3, glyph: " ", name: "mod.rs", is_file: true },
        TreeRow { depth: 2, glyph: " ", name: "buffer.rs", is_file: true },
        TreeRow { depth: 1, glyph: " ", name: "Cargo.toml", is_file: true },
        TreeRow { depth: 0, glyph: "▾", name: "ratatui-widgets", is_file: false },
        TreeRow { depth: 1, glyph: "▸", name: "src", is_file: false },
        TreeRow { depth: 1, glyph: " ", name: "Cargo.toml", is_file: true },
        TreeRow { depth: 0, glyph: " ", name: "Cargo.toml", is_file: true },
        TreeRow { depth: 0, glyph: " ", name: "README.md", is_file: true },
    ]
}

pub struct Stash {
    pub reff: &'static str,
    pub hash: &'static str,
    pub branch: &'static str,
    pub time: &'static str,
    pub msg: &'static str,
    pub files: Vec<(char, &'static str)>,
}

pub fn stashes() -> Vec<Stash> {
    vec![
        Stash {
            reff: "stash@{0}", hash: "f10a2b9", branch: "main", time: "2m",
            msg: "WIP: refactor color model",
            files: vec![('M', "color.rs"), ('M', "list.rs"), ('+', "canvas_braille.rs")],
        },
        Stash {
            reff: "stash@{1}", hash: "8c41de0", branch: "main", time: "4h",
            msg: "debug prints in render loop",
            files: vec![('M', "app.rs"), ('M', "draw.rs")],
        },
        Stash {
            reff: "stash@{2}", hash: "2b7f5a1", branch: "feat/popup", time: "2d",
            msg: "popup: experiment with shadow",
            files: vec![('+', "popup.rs"), ('M', "mod.rs")],
        },
    ]
}

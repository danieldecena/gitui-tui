use crate::data;

/// All UI state: which tab is active and the selected row within each tab.
/// gitui keeps an independent cursor per view, so we do too.
pub struct App {
    pub tab: usize,
    /// Status tab: 0 = Unstaged pane focused, 1 = Staged pane focused.
    pub status_pane: usize,
    /// Selected row for [unstaged, staged].
    pub status_sel: [usize; 2],
    pub log_sel: usize,
    pub files_sel: usize,
    pub stashing_sel: usize,
    pub stashes_sel: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            tab: 0,
            status_pane: 0,
            status_sel: [0, 0],
            log_sel: 0,
            files_sel: 0,
            stashing_sel: 0,
            stashes_sel: 0,
        }
    }

    /// Length of the list the cursor is currently moving through.
    fn current_len(&self) -> usize {
        match self.tab {
            0 => {
                if self.status_pane == 0 {
                    data::unstaged().len()
                } else {
                    data::staged().len()
                }
            }
            1 => data::commits().len(),
            2 => data::tree().len(),
            3 => data::stashing_changes().len(),
            4 => data::stashes().len(),
            _ => 0,
        }
    }

    fn current_sel_mut(&mut self) -> &mut usize {
        match self.tab {
            0 => &mut self.status_sel[self.status_pane],
            1 => &mut self.log_sel,
            2 => &mut self.files_sel,
            3 => &mut self.stashing_sel,
            _ => &mut self.stashes_sel,
        }
    }

    /// Move the active cursor by `delta`, wrapping at the ends (gitui wraps).
    pub fn move_sel(&mut self, delta: isize) {
        let len = self.current_len();
        if len == 0 {
            return;
        }
        let cur = *self.current_sel_mut() as isize;
        let next = (cur + delta).rem_euclid(len as isize) as usize;
        *self.current_sel_mut() = next;
    }
}

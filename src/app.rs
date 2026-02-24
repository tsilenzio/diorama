use std::time::Instant;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use tempfile::TempDir;

use crate::panels::{self, Panel};
use crate::scaffold::{self, DetectedTools};
use crate::ui::{columns_for_width, PANEL_HEIGHT};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    Grid,
    FullScreen,
}

pub struct App {
    pub mode: ViewMode,
    pub panels: Vec<Panel>,
    pub selected: usize,
    pub scroll_offset: u16,
    pub fullscreen_scroll: u16,
    pub confirm_quit: Option<Instant>,
    #[allow(dead_code)]
    pub tools: DetectedTools,
    pub _scaffold: TempDir,
}

impl App {
    pub fn new(offline: bool, panel_index: Option<usize>) -> color_eyre::Result<Self> {
        let (scaffold_dir, tools) = scaffold::setup(offline)?;
        let panels = panels::build_all(&tools);

        let (mode, selected) = match panel_index {
            Some(i) => (ViewMode::FullScreen, i),
            None => (ViewMode::Grid, 0),
        };

        Ok(Self {
            mode,
            panels,
            selected,
            scroll_offset: 0,
            fullscreen_scroll: 0,
            confirm_quit: None,
            tools,
            _scaffold: scaffold_dir,
        })
    }

    /// Returns true if the app should quit.
    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        // Any non-Esc key clears the quit confirmation
        if key.code != KeyCode::Esc {
            self.confirm_quit = None;
        }

        match key.code {
            KeyCode::Char('q') | KeyCode::Char('Q') => return true,
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => return true,

            KeyCode::Esc => {
                if self.mode == ViewMode::FullScreen {
                    self.mode = ViewMode::Grid;
                    self.confirm_quit = None;
                } else if let Some(t) = self.confirm_quit {
                    if t.elapsed().as_secs() < 2 {
                        return true;
                    }
                    self.confirm_quit = Some(Instant::now());
                } else {
                    self.confirm_quit = Some(Instant::now());
                }
            }

            KeyCode::Tab | KeyCode::Enter => match self.mode {
                ViewMode::Grid => {
                    self.mode = ViewMode::FullScreen;
                    self.fullscreen_scroll = 0;
                }
                ViewMode::FullScreen => {
                    self.mode = ViewMode::Grid;
                }
            },

            // Panel cycling in fullscreen
            KeyCode::Right | KeyCode::Char('l') => {
                if self.mode == ViewMode::FullScreen && !self.panels.is_empty() {
                    self.selected = (self.selected + 1) % self.panels.len();
                    self.fullscreen_scroll = 0;
                }
            }
            KeyCode::Left | KeyCode::Char('h') => {
                if self.mode == ViewMode::FullScreen && !self.panels.is_empty() {
                    self.selected = self
                        .selected
                        .checked_sub(1)
                        .unwrap_or(self.panels.len() - 1);
                    self.fullscreen_scroll = 0;
                }
            }

            // Scrolling
            KeyCode::Down | KeyCode::Char('j') => {
                if self.mode == ViewMode::Grid {
                    self.scroll_offset = self.scroll_offset.saturating_add(2);
                    self.clamp_scroll();
                } else {
                    self.fullscreen_scroll = self.fullscreen_scroll.saturating_add(1);
                    self.clamp_fullscreen_scroll();
                }
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if self.mode == ViewMode::Grid {
                    self.scroll_offset = self.scroll_offset.saturating_sub(2);
                } else {
                    self.fullscreen_scroll = self.fullscreen_scroll.saturating_sub(1);
                }
            }
            KeyCode::PageDown | KeyCode::Char('d')
                if key.code == KeyCode::PageDown
                    || key.modifiers.contains(KeyModifiers::CONTROL) =>
            {
                if self.mode == ViewMode::Grid {
                    let page = self.page_size();
                    self.scroll_offset = self.scroll_offset.saturating_add(page);
                    self.clamp_scroll();
                } else {
                    let page = self.page_size();
                    self.fullscreen_scroll = self.fullscreen_scroll.saturating_add(page);
                    self.clamp_fullscreen_scroll();
                }
            }
            KeyCode::PageUp | KeyCode::Char('u')
                if key.code == KeyCode::PageUp || key.modifiers.contains(KeyModifiers::CONTROL) =>
            {
                if self.mode == ViewMode::Grid {
                    let page = self.page_size();
                    self.scroll_offset = self.scroll_offset.saturating_sub(page);
                } else {
                    let page = self.page_size();
                    self.fullscreen_scroll = self.fullscreen_scroll.saturating_sub(page);
                }
            }
            KeyCode::Home | KeyCode::Char('g') => {
                if self.mode == ViewMode::Grid {
                    self.scroll_offset = 0;
                } else {
                    self.fullscreen_scroll = 0;
                }
            }
            KeyCode::End | KeyCode::Char('G') => {
                if self.mode == ViewMode::Grid {
                    self.scroll_offset = u16::MAX;
                    self.clamp_scroll();
                } else {
                    self.fullscreen_scroll = u16::MAX;
                    self.clamp_fullscreen_scroll();
                }
            }

            _ => {}
        }
        false
    }

    fn page_size(&self) -> u16 {
        let (_, term_h) = crossterm::terminal::size().unwrap_or((80, 24));
        term_h.saturating_sub(4)
    }

    fn clamp_fullscreen_scroll(&mut self) {
        if self.panels.is_empty() {
            self.fullscreen_scroll = 0;
            return;
        }
        let (_, term_h) = crossterm::terminal::size().unwrap_or((80, 24));
        let inner_h = term_h.saturating_sub(3);
        let content_lines = self.panels[self.selected].content.len() as u16;
        let max_scroll = content_lines.saturating_sub(inner_h);
        self.fullscreen_scroll = self.fullscreen_scroll.min(max_scroll);
    }

    fn clamp_scroll(&mut self) {
        let (term_w, term_h) = crossterm::terminal::size().unwrap_or((80, 24));
        let grid_h = term_h.saturating_sub(1);

        let cols = columns_for_width(term_w);
        let rows = self.panels.len().div_ceil(cols);
        let total_height = rows as u16 * PANEL_HEIGHT;
        let max_scroll = total_height.saturating_sub(grid_h);

        self.scroll_offset = self.scroll_offset.min(max_scroll);
    }
}

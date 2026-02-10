use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::panels::{self, Panel};
use crate::scaffold::{self, DetectedTools};

pub struct App {
    pub panels: Vec<Panel>,
    pub selected: usize,
    #[allow(dead_code)]
    tools: DetectedTools,
}

impl App {
    pub fn new() -> Self {
        let tools = scaffold::setup();
        let panels = panels::build_all(&tools);

        Self {
            panels,
            selected: 0,
            tools,
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Char('q') | KeyCode::Char('Q') => return true,
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => return true,
            KeyCode::Esc => return true,
            _ => {}
        }
        false
    }
}

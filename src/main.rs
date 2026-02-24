mod app;
mod panels;
mod scaffold;
mod ui;

use std::io;
use std::process;

use clap::Parser;
use color_eyre::Result;
use crossterm::{
    event::{self, Event, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::*;

use app::App;

#[derive(Parser)]
#[command(about = "Preview terminal color themes with realistic scenarios")]
struct Cli {
    /// Jump to a panel by name (case-insensitive substring match)
    #[arg(short, long)]
    panel: Option<String>,

    /// Skip tool detection, use fallback prompts
    #[arg(long)]
    offline: bool,

    /// List available panels and exit
    #[arg(short, long)]
    list: bool,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    if cli.list {
        let tools = scaffold::offline_tools();
        let panels = panels::build_all(&tools);
        for p in &panels {
            println!("{}", p.title);
        }
        return Ok(());
    }

    // Resolve panel index before entering the TUI
    let panel_index = if let Some(ref query) = cli.panel {
        let tools = scaffold::offline_tools();
        let all = panels::build_all(&tools);
        match panels::find_panel_index(&all, query) {
            Some(i) => Some(i),
            None => {
                eprintln!("no panel matching '{query}'");
                process::exit(1);
            }
        }
    } else {
        None
    };

    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;

    let mut app = App::new(cli.offline, panel_index)?;

    loop {
        terminal.draw(|f| ui::draw(f, &app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && app.handle_key(key) {
                break;
            }
        }
    }

    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

mod app;
mod panels;
mod scaffold;
mod ui;

use std::io;
use std::process;

use clap::Parser;
use color_eyre::Result;
use crossterm::{
    ExecutableCommand,
    event::{self, Event, KeyEventKind},
    terminal::{
        BeginSynchronizedUpdate, EndSynchronizedUpdate, EnterAlternateScreen, LeaveAlternateScreen,
        disable_raw_mode, enable_raw_mode,
    },
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

    /// Render without color (also honors the NO_COLOR env var)
    #[arg(long)]
    no_color: bool,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    let no_color = cli.no_color || std::env::var_os("NO_COLOR").is_some_and(|v| !v.is_empty());

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

    let mut app = App::new(cli.offline, no_color, panel_index)?;

    loop {
        // Bracket each frame in a synchronized update (DEC mode 2026) so
        // terminals that support it present the frame atomically. This stops
        // the blank clear-and-repaint that ratatui does on resize from showing
        // as a flash on fast terminals like Ghostty.
        io::stdout().execute(BeginSynchronizedUpdate)?;
        terminal.draw(|f| ui::draw(f, &app))?;
        io::stdout().execute(EndSynchronizedUpdate)?;

        if let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
            && app.handle_key(key)
        {
            break;
        }
    }

    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

use ratatui::style::Color;
use ratatui::text::Line;

use ansi_to_tui::IntoText;

use super::*;

// Build a panel straight from captured tool output. The samples are real ANSI
// dumps produced by scripts/generate-samples and embedded at compile time.
fn sample_panel(title: &str, icon: char, color: Color, ansi: &[u8]) -> Panel {
    let content = ansi
        .into_text()
        .map(|t| t.lines)
        .unwrap_or_else(|_| vec![Line::from(s("(failed to parse sample)", RED))]);
    Panel {
        title: title.into(),
        icon,
        border_color: color,
        content,
    }
}

pub(super) fn cpp_real_panel() -> Panel {
    sample_panel(
        "C/C++ (real)",
        '\u{e61d}',
        BLUE,
        include_bytes!("../../samples/cpp-cmake-error.ansi"),
    )
}

pub(super) fn ruby_real_panel() -> Panel {
    sample_panel(
        "Ruby (real)",
        '\u{e791}',
        RED,
        include_bytes!("../../samples/ruby-rspec.ansi"),
    )
}

pub(super) fn lua_real_panel() -> Panel {
    sample_panel(
        "Lua (real)",
        '\u{e620}',
        BLUE,
        include_bytes!("../../samples/lua-busted.ansi"),
    )
}

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

use crate::scaffold::DetectedTools;

mod devtools;
mod git;
mod interactive;
mod languages;
mod system;

pub(crate) const BLACK: Color = Color::Indexed(0);
pub(crate) const RED: Color = Color::Indexed(1);
pub(crate) const GREEN: Color = Color::Indexed(2);
pub(crate) const YELLOW: Color = Color::Indexed(3);
pub(crate) const BLUE: Color = Color::Indexed(4);
pub(crate) const MAGENTA: Color = Color::Indexed(5);
pub(crate) const CYAN: Color = Color::Indexed(6);
pub(crate) const WHITE: Color = Color::Indexed(7);
pub(crate) const BRIGHT_BLACK: Color = Color::Indexed(8);
pub(crate) const BRIGHT_RED: Color = Color::Indexed(9);
pub(crate) const BRIGHT_GREEN: Color = Color::Indexed(10);
pub(crate) const BRIGHT_YELLOW: Color = Color::Indexed(11);
pub(crate) const BRIGHT_BLUE: Color = Color::Indexed(12);
pub(crate) const BRIGHT_MAGENTA: Color = Color::Indexed(13);
pub(crate) const BRIGHT_CYAN: Color = Color::Indexed(14);
pub(crate) const BRIGHT_WHITE: Color = Color::Indexed(15);

pub(crate) const BOLD: Modifier = Modifier::BOLD;
pub(crate) const DIM: Modifier = Modifier::DIM;
pub(crate) const ITALIC: Modifier = Modifier::ITALIC;
pub(crate) const UNDERLINED: Modifier = Modifier::UNDERLINED;

pub(crate) fn s(text: &str, fg: Color) -> Span<'static> {
    Span::styled(text.to_string(), Style::default().fg(fg))
}

pub(crate) fn sb(text: &str, fg: Color) -> Span<'static> {
    Span::styled(
        text.to_string(),
        Style::default().fg(fg).add_modifier(BOLD),
    )
}

pub(crate) fn sd(text: &str, fg: Color) -> Span<'static> {
    Span::styled(
        text.to_string(),
        Style::default().fg(fg).add_modifier(DIM),
    )
}

pub(crate) fn si(text: &str, fg: Color) -> Span<'static> {
    Span::styled(
        text.to_string(),
        Style::default().fg(fg).add_modifier(ITALIC),
    )
}

pub(crate) fn su(text: &str, fg: Color) -> Span<'static> {
    Span::styled(
        text.to_string(),
        Style::default().fg(fg).add_modifier(UNDERLINED),
    )
}

pub(crate) fn sfgbg(text: &str, fg: Color, bg: Color) -> Span<'static> {
    Span::styled(text.to_string(), Style::default().fg(fg).bg(bg))
}

pub(crate) fn blank() -> Line<'static> {
    Line::from("")
}

pub(crate) fn col(text: &str, width: usize, fg: Color) -> Span<'static> {
    Span::styled(format!("{:<width$}", text), Style::default().fg(fg))
}

pub(crate) fn colb(text: &str, width: usize, fg: Color) -> Span<'static> {
    Span::styled(
        format!("{:<width$}", text),
        Style::default().fg(fg).add_modifier(BOLD),
    )
}

pub(crate) fn cold(text: &str, width: usize, fg: Color) -> Span<'static> {
    Span::styled(
        format!("{:<width$}", text),
        Style::default().fg(fg).add_modifier(DIM),
    )
}

pub(crate) fn colr(text: &str, width: usize, fg: Color) -> Span<'static> {
    Span::styled(format!("{:>width$}", text), Style::default().fg(fg))
}

pub(crate) fn colrb(text: &str, width: usize, fg: Color) -> Span<'static> {
    Span::styled(
        format!("{:>width$}", text),
        Style::default().fg(fg).add_modifier(BOLD),
    )
}

pub struct Panel {
    pub title: String,
    pub icon: char,
    pub border_color: Color,
    pub content: Vec<Line<'static>>,
}

pub fn build_all(tools: &DetectedTools) -> Vec<Panel> {
    vec![
        languages::python_panel(tools),
        languages::node_panel(tools),
        languages::rust_panel(tools),
        languages::go_panel(tools),
        languages::cpp_panel(tools),
        languages::zig_panel(tools),
        languages::java_panel(tools),
        languages::csharp_panel(tools),
        languages::ruby_panel(tools),
        languages::lua_panel(tools),
        git::git_log_panel(),
        git::git_diff_panel(),
        system::docker_panel(),
        system::file_listing_panel(),
        system::system_info_panel(),
        system::process_monitor_panel(),
        devtools::log_output_panel(),
        devtools::ripgrep_panel(),
        devtools::kubectl_panel(),
        devtools::json_panel(),
        interactive::python_repl_panel(),
        interactive::node_repl_panel(),
        interactive::sudo_panel(tools),
        interactive::color_palette_panel(),
    ]
}

pub(crate) fn prompt_lines(tools: &DetectedTools, lang: &str) -> Vec<Line<'static>> {
    tools
        .prompts
        .iter()
        .find(|(name, _)| name == lang)
        .map(|(_, text)| text.lines.clone())
        .unwrap_or_else(|| vec![Line::from(s("$ ", WHITE))])
}

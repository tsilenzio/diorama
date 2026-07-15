use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

use crate::scaffold::DetectedTools;

mod devtools;
mod git;
mod interactive;
mod languages;
mod samples;
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
    Span::styled(text.to_string(), Style::default().fg(fg).add_modifier(BOLD))
}

pub(crate) fn sd(text: &str, fg: Color) -> Span<'static> {
    Span::styled(text.to_string(), Style::default().fg(fg).add_modifier(DIM))
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
        languages::node_panel(tools),
        languages::python_panel(tools),
        languages::rust_panel(tools),
        languages::go_panel(tools),
        languages::cpp_panel(tools),
        languages::zig_panel(tools),
        languages::java_panel(tools),
        languages::csharp_panel(tools),
        languages::ruby_panel(tools),
        languages::lua_panel(tools),
        samples::cpp_real_panel(tools),
        samples::ruby_real_panel(tools),
        samples::lua_real_panel(tools),
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

pub fn find_panel_index(panels: &[Panel], query: &str) -> Option<usize> {
    let q = query.to_lowercase();
    panels
        .iter()
        .position(|p| p.title.to_lowercase().contains(&q))
}

pub(crate) fn prompt_lines(tools: &DetectedTools, lang: &str) -> Vec<Line<'static>> {
    tools
        .prompts
        .iter()
        .find(|(name, _)| name == lang)
        .map(|(_, text)| text.lines.clone())
        .unwrap_or_else(|| vec![Line::from(s("$ ", WHITE))])
}

// The prompt with the command typed onto its input line, matching how it looks
// when you actually run the command rather than putting it on a fresh line.
pub(crate) fn prompt_with_command(
    tools: &DetectedTools,
    lang: &str,
    command: &str,
) -> Vec<Line<'static>> {
    let mut lines = prompt_lines(tools, lang);
    match lines.last_mut() {
        Some(last) => last.spans.push(s(command, BRIGHT_WHITE)),
        None => lines.push(Line::from(s(command, BRIGHT_WHITE))),
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scaffold;

    fn test_panels() -> Vec<Panel> {
        let tools = scaffold::offline_tools();
        build_all(&tools)
    }

    #[test]
    fn find_panel_exact() {
        let panels = test_panels();
        let idx = find_panel_index(&panels, "Rust").unwrap();
        assert_eq!(panels[idx].title, "Rust");
    }

    #[test]
    fn find_panel_case_insensitive() {
        let panels = test_panels();
        let idx = find_panel_index(&panels, "RUST").unwrap();
        assert_eq!(panels[idx].title, "Rust");
    }

    #[test]
    fn find_panel_substring() {
        let panels = test_panels();
        let idx = find_panel_index(&panels, "log").unwrap();
        assert_eq!(panels[idx].title, "Git Log");
    }

    #[test]
    fn find_panel_no_match() {
        let panels = test_panels();
        assert!(find_panel_index(&panels, "nope").is_none());
    }

    #[test]
    fn find_panel_returns_first_match() {
        let panels = test_panels();
        // "python" matches both "Python" and "Python REPL", should return first
        let idx = find_panel_index(&panels, "python").unwrap();
        assert_eq!(panels[idx].title, "Python");
    }

    #[test]
    fn panel_count() {
        let panels = test_panels();
        assert_eq!(panels.len(), 27);
    }
}

use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

use crate::scaffold::DetectedTools;

const BRIGHT_MAGENTA: Color = Color::Indexed(13);

fn s(text: &str, fg: Color) -> Span<'static> {
    Span::styled(text.to_string(), Style::default().fg(fg))
}

pub struct Panel {
    pub title: String,
    pub icon: char,
    pub border_color: Color,
    pub content: Vec<Line<'static>>,
}

// TODO: add more panels
pub fn build_all(_tools: &DetectedTools) -> Vec<Panel> {
    vec![
        color_palette_panel(),
    ]
}

fn color_palette_panel() -> Panel {
    let colors = [
        (0, "Black"), (1, "Red"), (2, "Green"), (3, "Yellow"),
        (4, "Blue"), (5, "Magenta"), (6, "Cyan"), (7, "White"),
        (8, "Bright Black"), (9, "Bright Red"), (10, "Bright Green"),
        (11, "Bright Yellow"), (12, "Bright Blue"), (13, "Bright Magenta"),
        (14, "Bright Cyan"), (15, "Bright White"),
    ];

    let mut lines = Vec::new();
    for (idx, name) in colors {
        let color = Color::Indexed(idx);
        lines.push(Line::from(vec![
            Span::styled(
                "  ████  ",
                Style::default().fg(color).bg(color),
            ),
            s(&format!(" {:>2}  {}", idx, name), color),
        ]));
    }

    Panel {
        title: "Color Palette".into(),
        icon: '\u{f0266}',
        border_color: BRIGHT_MAGENTA,
        content: lines,
    }
}
